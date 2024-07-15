use crate::circuit_translation::binary_digits_target::BinaryDigitsTarget;
use crate::circuit_translation::CircuitBuilderFromAcirToPlonky2;
use super::*;

// These are unit tests for internal functions of the sha256 algorithm
// They are agnostic to acir code

fn test_rotate_right_1(){
    // pub fn rotate_right(
    //     &mut self,
    //     binary_target: &BinaryDigitsTarget,
    //     times: usize,
    // ) -> BinaryDigitsTarget {
    let circuit_translator = CircuitBuilderFromAcirToPlonky2::new();

    // let bits = vec![circuit_translator.builder.add_virtual_bool_target_unsafe()]
    // let binary_input = BinaryDigitsTarget{bits: bits};

}