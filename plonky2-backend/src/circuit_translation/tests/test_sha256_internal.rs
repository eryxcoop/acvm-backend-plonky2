use acir::circuit::opcodes::BlackBoxFuncCall::Sha256Compression;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};

use super::*;
use crate::binary_digits_target::BinaryDigitsTarget;
use crate::circuit_translation::tests::factories::circuit_factory::circuit_with_single_opcode;
use crate::circuit_translation::tests::factories::utils;

/// These are unit tests for internal functions of the sha256 algorithm. They are agnostic to ACIR.

#[test]
fn test_rotate_right_4_1() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_zero, g_zero, g_one, g_zero];
    let outputs = vec![g_zero, g_zero, g_zero, g_one];
    test_rotate_right(1, 4, inputs, outputs);
}
#[test]
#[should_panic]
fn test_rotate_right_failed() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_zero, g_zero, g_one, g_zero];
    let outputs = vec![g_zero, g_zero, g_zero, g_zero];
    test_rotate_right(1, 4, inputs, outputs);
}

#[test]
fn test_rotate_right_32_1() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![
        g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one,
        g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero,
        g_one, g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero,
    ];
    let outputs = vec![
        g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero,
        g_one, g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero, g_zero,
        g_zero, g_one, g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one,
    ];
    test_rotate_right(1, 32, inputs, outputs);
}

#[test]
fn test_rotate_right_32_2() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![
        g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let outputs = vec![
        g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    test_rotate_right(2, 32, inputs, outputs);
}

#[test]
fn test_rotate_right_32_32() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![
        g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let outputs = vec![
        g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    test_rotate_right(32, 32, inputs, outputs);
}

#[test]
fn test_shift_right_4_1() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_one, g_one, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_one, g_one];
    test_shift_right(1, 4, inputs, outputs);
}

#[test]
#[should_panic]
fn test_shift_right_failed() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![g_one, g_one, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_one, g_zero];
    test_shift_right(1, 4, inputs, outputs);
}

#[test]
fn test_shift_right_32_16() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs = vec![
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let outputs = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
    ];
    test_shift_right(16, 32, inputs, outputs);
}

#[test]
fn test_choose_4() {
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
fn test_choose_4_failed() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let chooser = vec![g_zero, g_one, g_zero, g_one];
    let inputs_1 = vec![g_one, g_one, g_zero, g_zero];
    let inputs_2 = vec![g_zero, g_zero, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_one, g_one];
    test_choose(4, chooser, inputs_1, inputs_2, outputs);
}

#[test]
fn test_choose_32() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let chooser = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let inputs_1 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let inputs_2 = vec![
        g_zero, g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero,
        g_one, g_zero, g_one, g_zero, g_zero, g_zero, g_zero, g_one, g_zero, g_zero, g_zero, g_one,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_one, g_zero, g_zero,
    ];
    let outputs = inputs_2.clone();
    test_choose(32, chooser, inputs_1, inputs_2, outputs);
}

#[test]
fn test_majority_4() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs_0 = vec![g_zero, g_one, g_zero, g_one];
    let inputs_1 = vec![g_one, g_one, g_zero, g_zero];
    let inputs_2 = vec![g_zero, g_zero, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_zero, g_one];
    test_majority(4, inputs_0, inputs_1, inputs_2, outputs);
}

#[test]
#[should_panic]
fn test_majority_4_failed() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs_0 = vec![g_zero, g_one, g_zero, g_one];
    let inputs_1 = vec![g_one, g_one, g_zero, g_zero];
    let inputs_2 = vec![g_zero, g_zero, g_one, g_one];
    let outputs = vec![g_zero, g_one, g_zero, g_zero];
    test_majority(4, inputs_0, inputs_1, inputs_2, outputs);
}

#[test]
fn test_majority_32() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs_0 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let inputs_1 = inputs_0.clone();
    let inputs_2 = vec![
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one,
    ];
    let outputs = inputs_0.clone();
    test_majority(32, inputs_0, inputs_1, inputs_2, outputs);
}

#[test]
fn test_add_module_32_bits_without_any_carry() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs_0 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    let inputs_1 = vec![
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one,
    ];
    let outputs = inputs_1.clone();
    test_add_module_32_bits(inputs_0, inputs_1, outputs);
}

#[test]
#[should_panic]
fn test_add_module_32_bits_fail() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs_0 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_one,
    ];
    let inputs_1 = vec![
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_zero,
    ];
    let outputs = inputs_1.clone();
    test_add_module_32_bits(inputs_0, inputs_1, outputs);
}

#[test]
fn test_simple_add_module_32_bits_with_carry() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs_0 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_one,
    ];
    let inputs_1 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_one,
    ];
    let outputs = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_one, g_zero,
    ];
    test_add_module_32_bits(inputs_0, inputs_1, outputs);
}

#[test]
fn test_flooded_add_module_32_bits_with_carry() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs_0 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_one,
    ];
    let inputs_1 = vec![
        g_zero, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one,
    ];
    let outputs = vec![
        g_one, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    test_add_module_32_bits(inputs_0, inputs_1, outputs);
}

#[test]
fn test_add_module_32_bits_with_overflow() {
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let inputs_0 = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_one,
    ];
    let inputs_1 = vec![
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one, g_one,
        g_one, g_one, g_one, g_one, g_one, g_one,
    ];
    let outputs = vec![
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
        g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero, g_zero,
    ];
    test_add_module_32_bits(inputs_0, inputs_1, outputs);
}

fn test_add_module_32_bits(input_values_1: Vec<F>, input_values_2: Vec<F>, output_values: Vec<F>) {
    assert_eq!(input_values_1.len(), input_values_2.len());
    assert_eq!(input_values_1.len(), output_values.len());
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits_1 = (0..32)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();
    let bits_2 = (0..32)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();

    let binary_input_1 = BinaryDigitsTarget { bits: bits_1 };
    let binary_input_2 = BinaryDigitsTarget { bits: bits_2 };
    let result_bits = BinaryDigitsTarget::add_module_32_bits(
        &binary_input_1,
        &binary_input_2,
        &mut circuit_builder,
    );

    let mut partial_witnesses = PartialWitness::<F>::new();
    for i in 0..32 {
        partial_witnesses.set_target(binary_input_1.bits[i].target, input_values_1[i]);
        partial_witnesses.set_target(binary_input_2.bits[i].target, input_values_2[i]);
        partial_witnesses.set_target(result_bits.bits[i].target, output_values[i]);
    }

    let circuit_data = circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}

fn test_majority(
    size: usize,
    input_values_0: Vec<F>,
    input_values_1: Vec<F>,
    input_values_2: Vec<F>,
    output_values: Vec<F>,
) {
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits_0 = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();
    let bits_1 = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();
    let bits_2 = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();

    let binary_input_0 = BinaryDigitsTarget { bits: bits_0 };
    let binary_input_1 = BinaryDigitsTarget { bits: bits_1 };
    let binary_input_2 = BinaryDigitsTarget { bits: bits_2 };
    let chosen_bits = BinaryDigitsTarget::majority(
        &binary_input_0,
        &binary_input_1,
        &binary_input_2,
        &mut circuit_builder,
    );

    let mut partial_witnesses = PartialWitness::<F>::new();
    for i in 0..size {
        partial_witnesses.set_target(binary_input_0.bits[i].target, input_values_0[i]);
        partial_witnesses.set_target(binary_input_1.bits[i].target, input_values_1[i]);
        partial_witnesses.set_target(binary_input_2.bits[i].target, input_values_2[i]);
        partial_witnesses.set_target(chosen_bits.bits[i].target, output_values[i]);
    }

    let circuit_data = circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}

fn test_choose(
    size: usize,
    chooser_values: Vec<F>,
    input_values_1: Vec<F>,
    input_values_2: Vec<F>,
    output_values: Vec<F>,
) {
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits_chooser = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();
    let bits_1 = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();
    let bits_2 = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();

    let binary_chooser = BinaryDigitsTarget { bits: bits_chooser };
    let binary_input_1 = BinaryDigitsTarget { bits: bits_1 };
    let binary_input_2 = BinaryDigitsTarget { bits: bits_2 };
    let chosen_bits = BinaryDigitsTarget::choose(
        &binary_chooser,
        &binary_input_1,
        &binary_input_2,
        &mut circuit_builder,
    );

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

fn test_rotate_right(n: usize, size: usize, input_values: Vec<F>, output_values: Vec<F>) {
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();

    let binary_input = BinaryDigitsTarget { bits };
    let rotated_bits = BinaryDigitsTarget::rotate_right(&binary_input, n, &mut circuit_builder);

    let mut partial_witnesses = PartialWitness::<F>::new();
    for i in 0..size {
        partial_witnesses.set_target(binary_input.bits[i].target, input_values[i]);
        partial_witnesses.set_target(rotated_bits.bits[i].target, output_values[i]);
    }

    let circuit_data = circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}

fn test_shift_right(n: usize, size: usize, input_values: Vec<F>, output_values: Vec<F>) {
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();

    let binary_input = BinaryDigitsTarget { bits };
    let rotated_bits = BinaryDigitsTarget::shift_right(&binary_input, n, &mut circuit_builder);

    let mut partial_witnesses = PartialWitness::<F>::new();
    for i in 0..size {
        partial_witnesses.set_target(binary_input.bits[i].target, input_values[i]);
        partial_witnesses.set_target(rotated_bits.bits[i].target, output_values[i]);
    }

    let circuit_data = circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}

#[test]
fn test_compression_function() {
    // Given
    let public_input_witnesses: Vec<Witness> = (0..16).into_iter().map(|v| Witness(v)).collect();
    let initial_h: Vec<Witness> = (16..24).into_iter().map(|v| Witness(v)).collect();
    let output_witnesses: Vec<Witness> = (24..32).into_iter().map(|v| Witness(v)).collect();

    let only_opcode: Opcode = sha256_compression_opcode(
        public_input_witnesses.clone(),
        initial_h.clone(),
        output_witnesses.clone(),
    );
    let circuit = circuit_with_single_opcode(only_opcode, vec![]);

    // When
    let (circuit_data, witness_target_map) =
        utils::generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let g_zero = F::default();
    let g_2_to_the_31 = F::from_canonical_u32(1 << 31);
    let mut assignments: Vec<(Witness, F)> = vec![(Witness(0), g_2_to_the_31)];
    let mut a_15_zeroes = public_input_witnesses[1..16]
        .into_iter()
        .map(|w| (*w, g_zero))
        .collect::<Vec<_>>();
    assignments.append(&mut a_15_zeroes);
    let h_values = vec![
        F::from_canonical_u32(0x6a09e667),
        F::from_canonical_u32(0xbb67ae85),
        F::from_canonical_u32(0x3c6ef372),
        F::from_canonical_u32(0xa54ff53a),
        F::from_canonical_u32(0x510e527f),
        F::from_canonical_u32(0x9b05688c),
        F::from_canonical_u32(0x1f83d9ab),
        F::from_canonical_u32(0x5be0cd19),
    ];
    let mut initial_h_values = initial_h
        .into_iter()
        .zip(h_values.clone().into_iter())
        .collect::<Vec<_>>();

    let output_values = vec![
        F::from_canonical_u32(0xe3b0c442),
        F::from_canonical_u32(0x98fc1c14),
        F::from_canonical_u32(0x9afbf4c8),
        F::from_canonical_u32(0x996fb924),
        F::from_canonical_u32(0x27ae41e4),
        F::from_canonical_u32(0x649b934c),
        F::from_canonical_u32(0xa495991b),
        F::from_canonical_u32(0x7852b855),
    ];

    assignments.append(&mut initial_h_values);

    let mut output_values = output_witnesses
        .into_iter()
        .zip(output_values.into_iter())
        .collect::<Vec<_>>();
    assignments.append(&mut output_values);

    let proof = utils::generate_plonky2_proof_using_witness_values(
        assignments,
        &witness_target_map,
        &circuit_data,
    );

    assert!(circuit_data.verify(proof).is_ok());
}

fn sha256_compression_opcode(
    public_input_witnesses: Vec<Witness>,
    initial_h: Vec<Witness>,
    output_witnesses: Vec<Witness>,
) -> Opcode {
    let fi = Box::new(
        public_input_witnesses
            .into_iter()
            .map(|w| FunctionInput {
                witness: w,
                num_bits: 32,
            })
            .collect::<Vec<FunctionInput>>()
            .try_into()
            .unwrap(),
    );

    let ih = Box::new(
        initial_h
            .into_iter()
            .map(|w| FunctionInput {
                witness: w,
                num_bits: 32,
            })
            .collect::<Vec<FunctionInput>>()
            .try_into()
            .unwrap(),
    );

    let aux: [Witness; 8] = output_witnesses.try_into().unwrap();

    let o = Box::new(aux);

    Opcode::BlackBoxFuncCall(Sha256Compression {
        inputs: fi,
        hash_values: ih,
        outputs: o,
    })
}
