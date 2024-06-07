use crate::circuit_translation::tests::factories::circuit_factory;
use crate::circuit_translation::tests::factories::utils::*;
use super::*;

#[test]
fn test_backend_can_translate_blackbox_func_call_range_check_u8() {
    let max_num_bits = 8;
    let max_allowed_witness_value = 2u16.pow(max_num_bits.clone()) - 1;
    let max_allowed_witness_value_field = F::from_noncanonical_u64(max_allowed_witness_value.into());
    test_range_check_with_witness_value(max_allowed_witness_value_field, max_num_bits);
}

#[test]
#[should_panic]
fn test_backend_cannot_provide_witness_value_bigger_than_u8_for_u8_range_check() {
    let max_num_bits = 8;
    let min_not_allowed_witness_value = 2u16.pow(max_num_bits.clone());
    let min_not_allowed_witness_value_field = F::from_noncanonical_u64(min_not_allowed_witness_value.into());
    test_range_check_with_witness_value(min_not_allowed_witness_value_field, max_num_bits);
}

#[test]
fn test_backend_can_translate_blackbox_func_call_range_check_u16() {
    let max_num_bits = 16;
    let max_allowed_witness_value = 2u32.pow(max_num_bits.clone()) - 1;
    let max_allowed_witness_value_field = F::from_noncanonical_u64(max_allowed_witness_value.into());
    test_range_check_with_witness_value(max_allowed_witness_value_field, max_num_bits);
}

#[test]
#[should_panic]
fn test_backend_cannot_provide_witness_value_bigger_than_u16_for_u16_range_check() {
    let max_num_bits = 16;
    let min_not_allowed_witness_value = 2u32.pow(max_num_bits.clone());
    let min_not_allowed_witness_value_field = F::from_noncanonical_u64(min_not_allowed_witness_value.into());
    test_range_check_with_witness_value(min_not_allowed_witness_value_field, max_num_bits);
}

#[test]
fn test_backend_can_translate_blackbox_func_call_range_check_u32() {
    let max_num_bits = 32;
    let max_allowed_witness_value = 2u64.pow(max_num_bits.clone()) - 1;
    let max_allowed_witness_value_field = F::from_noncanonical_u64(max_allowed_witness_value.into());
    test_range_check_with_witness_value(max_allowed_witness_value_field, max_num_bits);
}

#[test]
#[should_panic]
fn test_backend_cannot_provide_witness_value_bigger_than_u32_for_u32_range_check() {
    let max_num_bits = 32;
    let min_not_allowed_witness_value = 2u64.pow(max_num_bits.clone());
    let min_not_allowed_witness_value_field = F::from_noncanonical_u64(min_not_allowed_witness_value.into());
    test_range_check_with_witness_value(min_not_allowed_witness_value_field, max_num_bits);
}

#[test]
#[should_panic(expected = "Range checks with more than 32 bits are not allowed yet while using Plonky2 prover")]
fn test_backend_does_not_support_range_check_for_u64_or_bigger() {
    let max_num_bits = 64;
    let goldilocks_max_value = (2u128.pow(64) - 2u128.pow(32)) as u64;
    let goldilocks_max_value_field = F::from_noncanonical_u64(goldilocks_max_value.into());
    test_range_check_with_witness_value(goldilocks_max_value_field, max_num_bits);
}

fn test_range_check_with_witness_value(witness_value: F, max_num_bits: u32){
    //Given
    let public_input_witness = Witness(0);
    let black_box_range_8_opcode = circuit_factory::black_box_range_opcode(public_input_witness, max_num_bits);
    let circuit = circuit_factory::circuit_with_single_opcode(black_box_range_8_opcode, vec![public_input_witness]);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_input_witness, witness_value)], &witness_target_map, &circuit_data);
    assert!(circuit_data.verify(proof).is_ok());
}

// ---------------- BITWISE OPERATIONS ------------------ //

#[test]
fn test_backend_supports_bitwise_and_up_to_8_bits(){
    // fn main(mut x: u8, y: u8) -> pub u8{
    //     x & y
    // }

    let one = F::from_canonical_u64(1);
    let three = F::from_canonical_u64(3);
    let five = F::from_canonical_u64(5);
    _assert_backend_supports_bitwise_operation(8, five, three, one, _bitwise_and_circuit);
}

#[test]
fn test_backend_supports_bitwise_and_up_to_16_bits(){
    // fn main(mut x: u16, y: u16) -> pub u16{
    //     x & y
    // }

    let a = F::from_canonical_u64(0xFF00);
    let b = F::from_canonical_u64(0xF0F0);
    let output = F::from_canonical_u64(0xF000);
    _assert_backend_supports_bitwise_operation(16, a, b, output, _bitwise_and_circuit);
}

#[test]
fn test_backend_supports_bitwise_xor_up_to_8_bits(){
    // fn main(mut x: u8, y: u8) -> pub u8{
    //     x ^ y
    // }

    let three = F::from_canonical_u64(3);
    let five = F::from_canonical_u64(5);
    let six = F::from_canonical_u64(6);
    _assert_backend_supports_bitwise_operation(8, three, five, six, _bitwise_xor_circuit);
}

#[test]
fn test_backend_supports_bitwise_xor_up_to_16_bits(){
    // fn main(mut x: u16, y: u16) -> pub u16{
    //     x ^ y
    // }

    let a = F::from_canonical_u64(0xFF00);
    let b = F::from_canonical_u64(0xF0F0);
    let output = F::from_canonical_u64(0x0FF0);
    _assert_backend_supports_bitwise_operation(16, a, b, output, _bitwise_xor_circuit);
}

fn _assert_backend_supports_bitwise_operation(max_bits: u32, a: GoldilocksField, b: GoldilocksField,
                                              output: GoldilocksField, operation: fn(Witness, Witness, Witness, u32) -> Circuit){
    // fn main(mut x: u_maxbits, y: u_maxbits) -> pub u_maxbits{
    //     x (operation) y
    // }

    // Given
    let public_input_witness_0 = Witness(0);
    let public_input_witness_1 = Witness(1);
    let output_witness_2 = Witness(2);
    let circuit = operation(public_input_witness_0, public_input_witness_1, output_witness_2, max_bits);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let witness_assignment = vec![
        (public_input_witness_0, a),
        (public_input_witness_1, b),
        (output_witness_2, output)];

    let proof = generate_plonky2_proof_using_witness_values(
        witness_assignment, &witness_target_map, &circuit_data);

    assert!(circuit_data.verify(proof).is_ok());
}

fn _bitwise_and_circuit(public_input_witness_0: Witness, public_input_witness_1: Witness, output_witness_2: Witness, max_bits: u32) -> Circuit {
    circuit_factory::bitwise_and_circuit(
        public_input_witness_0, public_input_witness_1, output_witness_2, max_bits)
}

fn _bitwise_xor_circuit(public_input_witness_0: Witness, public_input_witness_1: Witness, output_witness_2: Witness, max_bits: u32) -> Circuit {
    circuit_factory::bitwise_xor_circuit(
        public_input_witness_0, public_input_witness_1, output_witness_2, max_bits)
}