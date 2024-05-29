use super::*;
use super::factories::circuit_factory::*;
use super::factories::utils::*;

#[test]
fn test_plonky2_vm_can_traslate_the_assert_x_equals_zero_program() {
    // Given
    let public_input_witness = Witness(0);
    let only_opcode = x_equals_0_opcode(public_input_witness);
    let circuit = circuit_with_single_opcode(only_opcode, vec![public_input_witness]);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let g_zero = F::default();
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_input_witness, g_zero)], &witness_target_map, &circuit_data);
    assert_eq!(g_zero, proof.public_inputs[0]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_traslate_the_assert_x_equals_constant_program() {
    // Given
    let public_input_witness = Witness(0);
    let only_opcode = x_equals_4_opcode(public_input_witness);
    let circuit = circuit_with_single_opcode(only_opcode, vec![public_input_witness]);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let four = F::from_canonical_u64(4);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_input_witness, four)], &witness_target_map, &circuit_data);
    assert_eq!(four, proof.public_inputs[0]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_traslate_the_assert_c_times_x_equals_constant_program() {
    // Given
    let public_input_witness = Witness(0);
    let only_opcode = x_times_3_equals_12_opcode(public_input_witness);
    let circuit = circuit_with_single_opcode(only_opcode, vec![public_input_witness]);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let four = F::from_canonical_u64(4);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_input_witness, four)], &witness_target_map, &circuit_data);
    assert_eq!(four, proof.public_inputs[0]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_traslate_the_x_times_3_plus_y_times_4_equals_constant_program() {
    // Given
    let first_public_input_witness = Witness(0);
    let second_public_input_witness = Witness(1);
    let only_opcode = x_times_3_plus_y_times_4_equals_constant(first_public_input_witness, second_public_input_witness);
    let circuit = circuit_with_single_opcode(
        only_opcode, vec![first_public_input_witness, second_public_input_witness]);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let one = F::from_canonical_u64(1);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(first_public_input_witness, one), (second_public_input_witness, one)],
        &witness_target_map, &circuit_data);

    assert_eq!(one, proof.public_inputs[0]);
    assert_eq!(one, proof.public_inputs[1]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_traslate_multiple_linear_combinations() {
    // Given
    let public_inputs = vec![Witness(0), Witness(1), Witness(2), Witness(3)];
    let only_opcode = multiple_linear_combinations_opcode(&public_inputs);
    let circuit = circuit_with_single_opcode(only_opcode, public_inputs.clone());

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let one = F::from_canonical_u64(1);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_inputs[0], one), (public_inputs[1], one),
             (public_inputs[2], one), (public_inputs[3], one)],
        &witness_target_map, &circuit_data);

    assert_eq!(one, proof.public_inputs[0]);
    assert_eq!(one, proof.public_inputs[1]);
    assert_eq!(one, proof.public_inputs[2]);
    assert_eq!(one, proof.public_inputs[3]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_traslate_the_x_times_x_program_equals_constant() {
    // Given
    let public_input = Witness(0);
    let only_opcode = two_times_x_times_x_opcode(public_input);
    let circuit = circuit_with_single_opcode(only_opcode, vec![public_input]);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let four = F::from_canonical_u64(4);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_input, four)], &witness_target_map, &circuit_data);

    assert_eq!(four, proof.public_inputs[0]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_traslate_the_c_times_x_times_y_program_equals_constant() {
    // Given
    let public_input_1 = Witness(0);
    let public_input_2 = Witness(1);
    let only_opcode = two_times_x_times_y_opcode(public_input_1, public_input_2);
    let circuit = circuit_with_single_opcode(only_opcode, vec![public_input_1, public_input_2]);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let four = F::from_canonical_u64(4);
    let five = F::from_canonical_u64(5);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_input_1, four), (public_input_2, five)],
        &witness_target_map, &circuit_data);

    assert_eq!(four, proof.public_inputs[0]);
    assert_eq!(five, proof.public_inputs[1]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_traslate_multiple_cuadratic_terms() {
    // Given
    let public_inputs = vec![Witness(0), Witness(1), Witness(2), Witness(3)];
    let only_opcode = multiple_cuadratic_terms_opcode(&public_inputs);
    let circuit = circuit_with_single_opcode(only_opcode, public_inputs.clone());

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let two = F::from_canonical_u64(2);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_inputs[0], two), (public_inputs[1], two),
             (public_inputs[2], two), (public_inputs[3], two)],
        &witness_target_map, &circuit_data);

    assert_eq!(two, proof.public_inputs[0]);
    assert_eq!(two, proof.public_inputs[1]);
    assert_eq!(two, proof.public_inputs[2]);
    assert_eq!(two, proof.public_inputs[3]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_traslate_multiple_cuadratic_terms_and_linear_combinations() {
    // Given
    let public_inputs = vec![Witness(0), Witness(1), Witness(2), Witness(3)];
    let only_opcode = multiple_cuadratic_terms_and_linear_combinations_opcode(&public_inputs);
    let circuit = circuit_with_single_opcode(only_opcode, public_inputs.clone());

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let two = F::from_canonical_u64(2);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_inputs[0], two), (public_inputs[1], two),
             (public_inputs[2], two), (public_inputs[3], two)],
        &witness_target_map, &circuit_data);

    assert_eq!(two, proof.public_inputs[0]);
    assert_eq!(two, proof.public_inputs[1]);
    assert_eq!(two, proof.public_inputs[2]);
    assert_eq!(two, proof.public_inputs[3]);
    circuit_data.verify(proof).expect("Verification failed");
}

#[test]
fn test_plonky2_vm_can_translate_circuits_with_2_assert_zero_opcodes() {
    // Given
    let public_input_witness = Witness(0);
    let intermediate_witness = Witness(1);
    let circuit = circuit_with_a_public_input_and_two_assert_zero_operands(public_input_witness,
                                                                           intermediate_witness);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    // Then
    let one = F::from_canonical_u64(1);
    let five = F::from_canonical_u64(5);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_input_witness, one), (intermediate_witness, five)],
        &witness_target_map, &circuit_data);

    assert_eq!(one, proof.public_inputs[0]);
    circuit_data.verify(proof).expect("Verification failed");
}

// #[test]
// fn test_solo_plonky2() {
//     let config = CircuitConfig::standard_recursion_config();
//     let mut builder = CB::new(config);
//
//     let tar1 = builder.add_virtual_target();
//     builder.register_public_input(tar1);
//     let tar2 = builder.add_virtual_target();
//
//     let tar3 = builder.mul_const(F::from_canonical_u64(18446744069414584320), tar2);
//     let tar4 = builder.add(tar1, tar3);
//     let tar5 = builder.add_const(tar4, F::from_canonical_u64(4));
//     builder.assert_zero(tar5);
//     let tar6 = builder.mul(tar2, tar2);
//     let tar7 = builder.add_const(tar6, F::from_canonical_u64(18446744069414584296));
//     builder.assert_zero(tar7);
//
//     let circuit_data: CircuitData<F, C, 2> = builder.build();
//
//     let mut witnesses = PartialWitness::<F>::new();
//     witnesses.set_target(tar1, GoldilocksField::from_canonical_u64(1));
//     witnesses.set_target(tar2, GoldilocksField::from_canonical_u64(5));
//
//     let proof = circuit_data.prove(witnesses).unwrap();
//     circuit_data.verify(proof).expect("as");
// }
