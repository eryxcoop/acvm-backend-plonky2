use super::*;
use crate::noir_and_plonky2_serialization;

fn parse_circuit_and_witnesses(
    circuit_path: &String,
    witness_path: String,
) -> (Circuit, WitnessStack) {
    let acir_program: Program =
        noir_and_plonky2_serialization::deserialize_program_within_file_path(circuit_path);
    let circuit = acir_program.functions[0].clone();
    let witness =
        noir_and_plonky2_serialization::deserialize_witnesses_within_file_path(witness_path);
    (circuit, witness)
}

fn _path_for_circuit(nargo_project_name: &str) -> String {
    String::from(format!(
        "src/circuit_translation/tests/factories/precompiled_circuits_0.47.0/{}/target/circuit.json",
        nargo_project_name))
}

fn _path_for_witnesses(nargo_project_name: &str) -> String {
    String::from(format!(
        "src/circuit_translation/tests/factories/precompiled_circuits_0.47.0/{}/target/witness",
        nargo_project_name
    ))
}

pub fn precompiled_circuit_and_withesses_with_name(program_name: &str) -> (Circuit, WitnessStack) {
    let circuit_path = _path_for_circuit(program_name);
    let witness_path = _path_for_witnesses(program_name);
    parse_circuit_and_witnesses(&circuit_path, witness_path)
}

