use plonky2::iop::witness::{PartialWitness, WitnessWrite};

use crate::circuit_translation::binary_digits_target::BinaryDigitsTarget;

use super::*;

// These are unit tests for internal functions of the sha256 algorithm
// They are agnostic to acir code

#[test]
fn test_rotate_right_1(){
    // pub fn rotate_right(
    //     binary_target: &BinaryDigitsTarget,
    //     times: usize,
    //     builder: CB
    // ) -> BinaryDigitsTarget {
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits = vec![
        circuit_builder.add_virtual_bool_target_unsafe(),
        circuit_builder.add_virtual_bool_target_unsafe(),
        circuit_builder.add_virtual_bool_target_unsafe(),
        circuit_builder.add_virtual_bool_target_unsafe(),
    ];
    let binary_input = BinaryDigitsTarget{bits};
    let rotated_bits = BinaryDigitsTarget::rotate_right(&binary_input, 1, &mut circuit_builder);

    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);

    let mut partial_witnesses = PartialWitness::<F>::new();
    partial_witnesses.set_target(binary_input.bits[0].target, g_zero);
    partial_witnesses.set_target(binary_input.bits[1].target, g_zero);
    partial_witnesses.set_target(binary_input.bits[2].target, g_one);
    partial_witnesses.set_target(binary_input.bits[3].target, g_zero);

    partial_witnesses.set_target(rotated_bits.bits[0].target, g_zero);
    partial_witnesses.set_target(rotated_bits.bits[1].target, g_zero);
    partial_witnesses.set_target(rotated_bits.bits[2].target, g_zero);
    partial_witnesses.set_target(rotated_bits.bits[3].target, g_one);

    let circuit_data =circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());

}