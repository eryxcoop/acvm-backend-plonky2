use plonky2::iop::witness::{PartialWitness, WitnessWrite};

use crate::circuit_translation::binary_digits_target::BinaryDigitsTarget;

use super::*;

// These are unit tests for internal functions of the sha256 algorithm
// They are agnostic to acir code

#[test]
fn test_rotate_right_4_1(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_zero, g_zero, g_one, g_zero];
    let outputs = vec![g_zero, g_zero, g_zero, g_one];
    test_rotate_right(1, 4, inputs, outputs);
}
#[test]
#[should_panic]
fn test_rotate_right_failed(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_zero, g_zero, g_one, g_zero];
    let outputs = vec![g_zero, g_zero, g_zero, g_zero];
    test_rotate_right(1, 4, inputs, outputs);
}

#[test]
fn test_rotate_right_32_1(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![
        g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero,
        g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero,
        g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero,
        g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero
    ];
    let outputs = vec![
        g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one,
        g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one,
        g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one,
        g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one,
    ];
    test_rotate_right(1, 32, inputs, outputs);
}

#[test]
fn test_rotate_right_32_2(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![
        g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let outputs = vec![
        g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    test_rotate_right(2, 32, inputs, outputs);
}

#[test]
fn test_rotate_right_32_32(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![
        g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let outputs = vec![
        g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    test_rotate_right(32, 32, inputs, outputs);
}

#[test]
fn test_shift_right_4_1(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_one, g_one, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_one, g_one];
    test_shift_right(1, 4, inputs, outputs);
}

#[test]
#[should_panic]
fn test_shift_right_failed(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_one, g_one, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_one, g_zero];
    test_shift_right(1, 4, inputs, outputs);
}


#[test]
fn test_shift_right_32_16(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let outputs = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
    ];
    test_shift_right(16, 32, inputs, outputs);
}

#[test]
fn test_choose_4(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let chooser = vec![g_zero, g_one, g_zero, g_one];
    let inputs_1 = vec![g_one, g_one, g_zero, g_zero];
    let inputs_2 = vec![g_zero, g_zero, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_one, g_zero];
    test_choose(4, chooser, inputs_1, inputs_2, outputs);
}

#[test]
#[should_panic]
fn test_choose_4_failed(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let chooser = vec![g_zero, g_one, g_zero, g_one];
    let inputs_1 = vec![g_one, g_one, g_zero, g_zero];
    let inputs_2 = vec![g_zero, g_zero, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_one, g_one];
    test_choose(4, chooser, inputs_1, inputs_2, outputs);
}

#[test]
fn test_choose_32(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let chooser = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let inputs_1 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let inputs_2 = vec![
        g_zero, g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_one,
        g_zero, g_zero, g_zero, g_one, g_zero, g_one, g_zero, g_zero,
        g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_one, g_zero, g_zero,
    ];
    let outputs = inputs_2.clone();
    test_choose(32, chooser, inputs_1, inputs_2, outputs);
}


fn test_choose(size: usize, chooser_values: Vec<F>, input_values_1: Vec<F>, input_values_2: Vec<F>, output_values: Vec<F>){
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits_chooser = (0..size).into_iter().map(|_| circuit_builder.add_virtual_bool_target_unsafe()).collect();
    let bits_1 = (0..size).into_iter().map(|_| circuit_builder.add_virtual_bool_target_unsafe()).collect();
    let bits_2 = (0..size).into_iter().map(|_| circuit_builder.add_virtual_bool_target_unsafe()).collect();

    let binary_chooser = BinaryDigitsTarget{bits: bits_chooser};
    let binary_input_1 = BinaryDigitsTarget{bits: bits_1};
    let binary_input_2 = BinaryDigitsTarget{bits: bits_2};
    let chosen_bits = BinaryDigitsTarget::choose(&binary_chooser, &binary_input_1, &binary_input_2, &mut circuit_builder);

    let mut partial_witnesses = PartialWitness::<F>::new();
    for i in 0..size {
        partial_witnesses.set_target(binary_chooser.bits[i].target, chooser_values[i]);
        partial_witnesses.set_target(binary_input_1.bits[i].target, input_values_1[i]);
        partial_witnesses.set_target(binary_input_2.bits[i].target, input_values_2[i]);
        partial_witnesses.set_target(chosen_bits.bits[i].target, output_values[i]);
    }

    let circuit_data = circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}

fn test_rotate_right(n: usize, size: usize, input_values: Vec<F>, output_values: Vec<F>){
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits = (0..size).into_iter().map(|_| circuit_builder.add_virtual_bool_target_unsafe()).collect();

    let binary_input = BinaryDigitsTarget{bits};
    let rotated_bits = BinaryDigitsTarget::rotate_right(&binary_input, n, &mut circuit_builder);

    let mut partial_witnesses = PartialWitness::<F>::new();
    for i in 0..size {
        partial_witnesses.set_target(binary_input.bits[i].target, input_values[i]);
        partial_witnesses.set_target(rotated_bits.bits[i].target, output_values[i]);
    }

    let circuit_data =circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}

fn test_shift_right(n: usize, size: usize, input_values: Vec<F>, output_values: Vec<F>){
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits = (0..size).into_iter().map(|_| circuit_builder.add_virtual_bool_target_unsafe()).collect();

    let binary_input = BinaryDigitsTarget{bits};
    let rotated_bits = BinaryDigitsTarget::shift_right(&binary_input, n, &mut circuit_builder);

    let mut partial_witnesses = PartialWitness::<F>::new();
    for i in 0..size {
        partial_witnesses.set_target(binary_input.bits[i].target, input_values[i]);
        partial_witnesses.set_target(rotated_bits.bits[i].target, output_values[i]);
    }

    let circuit_data =circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}