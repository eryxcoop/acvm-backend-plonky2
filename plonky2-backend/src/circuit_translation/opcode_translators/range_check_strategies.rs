use crate::circuit_translation::opcode_translators::limb_decomposition_8_bits_generator::LimbDecomposition8BitsGenerator;
use crate::circuit_translation::CB;
use crate::F;
use plonky2::field::types::Field;
use plonky2::gates::lookup_table::LookupTable;
use plonky2::iop::target::Target;
use std::sync::Arc;

pub trait RangeCheckStrategy {
    fn perform_range_operation_for_input(&mut self, long_max_bits: usize,
                                         target_holding_value: Target,
                                         builder: &mut CB);
}


#[derive(Default)]
pub struct RangeCheckLimbDecomposition {
    u8_range_table_index: Option<usize>
}

impl RangeCheckStrategy for RangeCheckLimbDecomposition {
    fn perform_range_operation_for_input(&mut self, long_max_bits: usize,
                                         target_holding_value: Target,
                                         builder: &mut CB) {
        if long_max_bits == 8 {
            let _limbs = self.split_target_into_8_bit_limbs::<1>(builder, target_holding_value);
        } else if long_max_bits == 16 {
            let _limbs = self.split_target_into_8_bit_limbs::<2>(builder, target_holding_value);
        } else if long_max_bits == 32 {
            let _limbs = self.split_target_into_8_bit_limbs::<4>(builder, target_holding_value);
        } else {
            assert!(long_max_bits <= 33,
                    "Range checks with more than 33 bits are not allowed yet while using Plonky2 prover");
            builder.range_check(target_holding_value, long_max_bits)
        }
    }
}

impl RangeCheckLimbDecomposition {
    pub fn new() -> Self {
        Self {
            u8_range_table_index: None,
        }
    }

    fn split_target_into_8_bit_limbs<const LIMBS: usize>(&mut self, builder: &mut CB, full_number: Target) -> [Target; LIMBS] {
        self.create_lookup_table_lazy(builder);

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

    fn create_lookup_table_lazy(&mut self, builder: &mut CB){
        match self.u8_range_table_index {
            Some(_index) => {}
            None => {
                let table: LookupTable =
                    Arc::new((0..256u16).zip(0..256u16).collect());
                let u8_range_table_index =
                    builder.add_lookup_table_from_pairs(table);
                self.u8_range_table_index = Some(u8_range_table_index);
            }
        }
    }
}

pub struct RangeCheckWithLookupTable {
    u8_range_table_index: Option<usize>,
}

impl RangeCheckWithLookupTable {
    pub fn new() -> Self {
        Self {
            u8_range_table_index: None,
        }
    }
}

impl RangeCheckStrategy for RangeCheckWithLookupTable {
    fn perform_range_operation_for_input(&mut self, long_max_bits: usize,
                                         target_holding_value: Target,
                                         builder: &mut CB) {
        if long_max_bits == 8 {
            match self.u8_range_table_index {
                Some(_index) => {}
                None => {
                    let table: LookupTable =
                        Arc::new((0..256u16).zip(0..256u16).collect());
                    let u8_range_table_index =
                        builder.add_lookup_table_from_pairs(table);
                    self.u8_range_table_index = Some(u8_range_table_index);
                }
            }
            builder.add_lookup_from_index(
                target_holding_value,
                self.u8_range_table_index.unwrap(),
            );
        } else {
            assert!(long_max_bits <= 33,
                    "Range checks with more than 33 bits are not allowed yet while using Plonky2 prover");
            builder.range_check(target_holding_value, long_max_bits)
        }
    }
}


pub struct RangeCheckBitSplit {}
impl RangeCheckStrategy for RangeCheckBitSplit {
    fn perform_range_operation_for_input(&mut self, long_max_bits: usize,
                                         target_holding_value: Target,
                                         builder: &mut CB) {
        assert!(long_max_bits <= 33,
                "Range checks with more than 33 bits are not allowed yet while using Plonky2 prover");
        builder.range_check(target_holding_value, long_max_bits)
    }
}

impl RangeCheckBitSplit {
    pub fn new() -> Self {
        Self {}
    }
}