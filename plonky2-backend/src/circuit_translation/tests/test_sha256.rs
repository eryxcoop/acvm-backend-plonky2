use plonky2::iop::witness::{PartialWitness, WitnessWrite};

use crate::circuit_translation::binary_digits_target::BinaryDigitsTarget;

use super::*;

// These are unit tests for internal functions of the sha256 algorithm
// They are agnostic to acir code

#[test]
fn test_rotate_right_1(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_zero, g_zero, g_one, g_zero];
    let outputs = vec![g_zero, g_zero, g_zero, g_one];
    test_rotate_right(1, 4, inputs, outputs);

}

fn test_rotate_right(n: usize, size: usize, input_values: Vec<F>, output_values: Vec<F>){
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits = (0..size).into_iter().map(|_| circuit_builder.add_virtual_bool_target_unsafe()).collect();

    let binary_input = BinaryDigitsTarget{bits};
    let rotated_bits = BinaryDigitsTarget::rotate_right(&binary_input, n, &mut circuit_builder);

    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);

    let mut partial_witnesses = PartialWitness::<F>::new();
    for i in 0..size {
        partial_witnesses.set_target(binary_input.bits[i].target, input_values[i]);
        partial_witnesses.set_target(rotated_bits.bits[i].target, output_values[i]);
    }

    let circuit_data =circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}