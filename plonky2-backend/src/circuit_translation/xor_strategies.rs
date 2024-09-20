use super::*;

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
        if num_bits == 8 {
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
