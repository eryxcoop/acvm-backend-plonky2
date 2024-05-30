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
    let max_allowed_witness_value_field = F::from_noncanonical_u64(min_not_allowed_witness_value.into());
    test_range_check_with_witness_value(max_allowed_witness_value_field, max_num_bits);
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
    let max_allowed_witness_value_field = F::from_noncanonical_u64(min_not_allowed_witness_value.into());
    test_range_check_with_witness_value(max_allowed_witness_value_field, max_num_bits);
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
    let max_allowed_witness_value_field = F::from_noncanonical_u64(min_not_allowed_witness_value.into());
    test_range_check_with_witness_value(max_allowed_witness_value_field, max_num_bits);
}

// #[test]
// #[should_panic]
// fn test_backend_does_not_support_range_check_for_u64_or_bigger() {
//     let max_num_bits = 16;
//     let min_not_allowed_witness_value = 2u32.pow(max_num_bits.clone());
//     let max_allowed_witness_value_field = F::from_noncanonical_u64(min_not_allowed_witness_value.into());
//     test_range_check_with_witness_value(max_allowed_witness_value_field, max_num_bits);
// }

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
    circuit_data.verify(proof).expect("Verification failed");
}