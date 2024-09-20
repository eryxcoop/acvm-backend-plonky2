use super::*;

pub trait RangeCheckStrategy {
    fn perform_range_operation_for_input(&mut self, long_max_bits: usize,
                                         target_holding_value: Target,
                                         builder: &mut CB);
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