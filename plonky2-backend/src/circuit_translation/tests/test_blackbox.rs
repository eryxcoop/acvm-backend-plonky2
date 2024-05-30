use crate::circuit_translation::tests::factories::circuit_factory;
use crate::circuit_translation::tests::factories::utils::*;
use super::*;

#[test]
fn test_backend_can_translate_blackbox_func_call_range_check_u8() {
    //Given
    let public_input_witness = Witness(0);
    let black_box_range_8_opcode = circuit_factory::black_box_range_8_opcode(public_input_witness);
    let circuit = circuit_factory::circuit_with_single_opcode(black_box_range_8_opcode, vec![public_input_witness]);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let g_three = F::from_canonical_usize(255);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(public_input_witness, g_three)], &witness_target_map, &circuit_data);
    circuit_data.verify(proof).expect("Verification failed");
}