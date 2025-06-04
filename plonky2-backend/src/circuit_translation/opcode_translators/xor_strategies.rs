use std::sync::Arc;
use plonky2::field::types::Field;
use plonky2::gates::lookup_table::LookupTable;
use plonky2::iop::target::Target;
use crate::binary_digits_target::BinaryDigitsTarget;
use crate::circuit_translation::CB;
use crate::circuit_translation::opcode_translators::limb_decomposition_8_bits_generator::LimbDecomposition8BitsGenerator;
use crate::F;

pub trait XorStrategy {
    fn perform_xor_operation_for_input(&mut self,
                                       target_left: Target,
                                       target_right: Target,
                                       target_output: Target,
                                       num_bits: u32,
                                       builder: &mut CB);
}

pub struct XorWithLookupTable {
    u8_xor_table_index: Option<usize>,
}

impl XorWithLookupTable {
    pub fn new() -> Self {
        Self {
            u8_xor_table_index: None,
        }
    }

    fn _xor_to_compressed_value(compressed_value: u16) -> u16 {
        /// We represent a xor operation (a xor b) = c in a lookup table as
        /// a * 256 + b --> c
        /// since lookup tables limit us to (u16, u16) pairs
        let a = compressed_value / 256;
        let b = compressed_value % 256;
        a ^ b
    }
}

impl XorStrategy for XorWithLookupTable {
    fn perform_xor_operation_for_input(&mut self, target_left: Target, target_right: Target, target_output: Target, num_bits: u32, builder: &mut CB) {
        if num_bits <= 8 {
            let target_256 = builder.constant(F::from_canonical_u32(256));
            let target_index_lookup =
                builder.mul_add(target_left, target_256, target_right);
            match self.u8_xor_table_index {
                Some(_index) => {}
                None => {
                    let mut supported_indexes: Vec<u16> = (0..65535).collect();
                    supported_indexes.push(65535u16);
                    let supported_indexes: &[u16] = &supported_indexes;
                    let u8_xor_table_index =
                        builder.add_lookup_table_from_fn(
                            Self::_xor_to_compressed_value,
                            supported_indexes,
                        );
                    self.u8_xor_table_index = Some(u8_xor_table_index);
                }
            }
            let output_lookup = builder.add_lookup_from_index(
                target_index_lookup,
                self.u8_xor_table_index.unwrap(),
            );
            builder.connect(output_lookup, target_output);
        } else {
            BinaryDigitsTarget::extend_circuit_with_bitwise_operation(
                target_left, target_right, target_output, num_bits, builder,
                BinaryDigitsTarget::xor,
            );
        }
    }
}

pub struct XorBitSplit;
impl XorBitSplit {
    pub fn new() -> Self{
        Self {}
    }
}

impl XorStrategy for XorBitSplit {
    fn perform_xor_operation_for_input(&mut self, target_left: Target, target_right: Target, target_output: Target, num_bits: u32, builder: &mut CB) {
        BinaryDigitsTarget::extend_circuit_with_bitwise_operation(
            target_left, target_right, target_output, num_bits, builder,
            BinaryDigitsTarget::xor,
        );
    }
}

// -----------------------------

pub struct XorWith8bitLimbDecomposition {
    u8_xor_table_index: Option<usize>,
    u8_range_table_index: Option<usize>,
}

impl XorWith8bitLimbDecomposition {
    pub fn new() -> Self {
        Self {
            u8_xor_table_index: None,
            u8_range_table_index: None,
        }
    }

    fn xor_to_compressed_value(compressed_value: u16) -> u16 {
        /// We represent a xor operation (a xor b) = c in a lookup table as
        /// a * 256 + b --> c
        /// since lookup tables limit us to (u16, u16) pairs
        let a = compressed_value / 256;
        let b = compressed_value % 256;
        a ^ b
    }

    fn create_lookup_tables_lazy(&mut self, builder: &mut CB) {
        match self.u8_xor_table_index {
            Some(_index) => {}
            None => {
                // ------- Xor ------- //
                let supported_indexes: Vec<u16> = (0..=65535).collect();
                let supported_indexes: &[u16] = &supported_indexes;
                self.u8_xor_table_index = Some(builder.add_lookup_table_from_fn(Self::xor_to_compressed_value, supported_indexes));
                // ------- Range check ------- //
                let table: LookupTable = Arc::new((0..256u16).zip(0..256u16).collect());
                let u8_range_table_index = builder.add_lookup_table_from_pairs(table);
                self.u8_range_table_index = Some(u8_range_table_index);
            }
        }
    }

    fn split_target_into_8_bit_limbs<const LIMBS: usize>(&mut self, builder: &mut CB, full_number: Target) -> [Target; LIMBS] {
        self.create_lookup_tables_lazy(builder);

        let limbs: [Target; LIMBS] = builder.add_virtual_targets(LIMBS).try_into().unwrap();
        builder.add_simple_generator(LimbDecomposition8BitsGenerator::<LIMBS> { full_number: full_number.clone(), limbs: limbs.clone() });

        for i in 0..LIMBS {
            builder.add_lookup_from_index(limbs[i], self.u8_range_table_index.unwrap());
        }

        let mut acc = limbs[0].clone();
        for i in 1..LIMBS {
            acc = builder.mul_const_add(F::from_canonical_u32(256u32.pow(i as u32)), limbs[i].clone(), acc);
        }

        builder.connect(acc, full_number);

        limbs
    }

    fn lookup_xor<const LIMBS: usize>(
        &mut self,
        builder: &mut CB,
        left_limbs: [Target; LIMBS],
        right_limbs: [Target; LIMBS],
        output_limbs: [Target; LIMBS])
    {
        let target_256 = builder.constant(F::from_canonical_u32(256));
        for ((left_limb, right_limb), output_limb) in left_limbs.iter().zip(right_limbs).zip(output_limbs){
            let target_index_lookup = builder.mul_add(*left_limb, target_256, right_limb);
            let output_lookup = builder.add_lookup_from_index(target_index_lookup, self.u8_xor_table_index.unwrap());
            builder.connect(output_lookup, output_limb);
        }
    }
}

impl XorStrategy for XorWith8bitLimbDecomposition {
    fn perform_xor_operation_for_input(
        &mut self,
        target_left: Target,
        target_right: Target,
        target_output: Target,
        num_bits: u32,
        builder: &mut CB)
    {
        if num_bits == 8 {
            self.create_lookup_tables_lazy(builder);
            self.lookup_xor::<1>(builder, [target_left], [target_right], [target_output])
        } else if num_bits == 16 {
            let limbs_left = self.split_target_into_8_bit_limbs::<2>(builder, target_left);
            let limbs_right = self.split_target_into_8_bit_limbs::<2>(builder, target_right);
            let limbs_output = self.split_target_into_8_bit_limbs::<2>(builder, target_output);
            self.lookup_xor::<2>(builder, limbs_left, limbs_right, limbs_output)
        } else if num_bits == 32 {
            let limbs_left = self.split_target_into_8_bit_limbs::<4>(builder, target_left);
            let limbs_right = self.split_target_into_8_bit_limbs::<4>(builder, target_right);
            let limbs_output = self.split_target_into_8_bit_limbs::<4>(builder, target_output);
            self.lookup_xor::<4>(builder, limbs_left, limbs_right, limbs_output)
        } else {
            BinaryDigitsTarget::extend_circuit_with_bitwise_operation(
                target_left, target_right, target_output, num_bits, builder,
                BinaryDigitsTarget::xor,
            );
        }
    }
}
