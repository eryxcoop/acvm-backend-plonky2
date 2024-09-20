use crate::circuit_translation::opcode_translators::range_check_strategies::{RangeCheckWithLookupTable, RangeCheckBitSplit, RangeCheckStrategy};
use crate::circuit_translation::opcode_translators::xor_strategies::{XorStrategy, XorWithLookupTable, XorBitSplit};
pub struct StrategyPicker;
impl StrategyPicker {
    pub fn range_check_strategy() -> Box<dyn RangeCheckStrategy>{
        let mut range_check_strategy: Box<dyn RangeCheckStrategy> = Box::new(RangeCheckBitSplit::new());
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "strategy-rangecheck-lookup", feature = "strategy-rangecheck-bitsplit"))] {
                compile_error!("feature \"strategy-rangecheck-lookup\" and feature \"strategy-rangecheck-bitsplit\" cannot be enabled at the same time");
            } else if #[cfg(feature = "strategy-rangecheck-lookup")] {
                range_check_strategy = Box::new(RangeCheckWithLookupTable::new());
            } else if #[cfg(feature = "strategy-rangecheck-bitsplit")] {
                range_check_strategy = Box::new(RangeCheckBitSplit::new());
            } else {
                compile_error!("No strategy selected for range check operation");
            }
        }
        range_check_strategy
    }

    pub fn xor_strategy() -> Box<dyn XorStrategy>{
        let mut xor_strategy: Box<dyn XorStrategy> = Box::new(XorBitSplit::new());
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "strategy-xor-lookup", feature = "strategy-xor-bitsplit"))] {
                compile_error!("feature \"strategy-xor-lookup\" and feature \"strategy-xor-bitsplit\" cannot be enabled at the same time");
            } else if #[cfg(feature = "strategy-xor-lookup")] {
                xor_strategy = Box::new(XorWithLookupTable::new());
            } else if #[cfg(feature = "strategy-xor-bitsplit")] {
                xor_strategy = Box::new(XorBitSplit::new());
            } else {
                compile_error!("No strategy selected for xor operation");
            }
        }
        xor_strategy
    }
}
