use super::*;
use crate::circuit_translation::tests::factories::{circuit_parser, utils};
use parameterized::parameterized;

// --------------------- SHA256 --------------------- //

#[parameterized(program_name = {"basic_memory_write", "sha256_4", "assert_x_equals_5"})]
fn test_noir_program(program_name: &str) {
    let (circuit, mut witnesses) =
        circuit_parser::precompiled_circuit_and_withesses_with_name(program_name);
    let witness_mapping = witnesses.pop().unwrap().witness;

    // print!("{:?}", circuit);

    // When
    let (circuit_data, witness_target_map) =
        utils::generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let mut witness_assignment: Vec<(Witness, F)> = vec![];
    for (witness, value) in witness_mapping {
        witness_assignment.push((witness, F::from_canonical_u64(value.try_to_u64().unwrap())));
    }

    // utils::check_linked_output_targets_property(&circuit, &witness_target_map);
    let proof = utils::generate_plonky2_proof_using_witness_values(
        witness_assignment,
        &witness_target_map,
        &circuit_data,
    );

    assert!(circuit_data.verify(proof).is_ok());
}
