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

pub fn precompiled_sha256_circuit_and_witnesses() -> (Circuit, WitnessStack) {
    let circuit_path = String::from(
        "src/circuit_translation/tests/factories/precompiled_circuits_0.47.0/sha256_4/circuit.json",
    );
    let witness_path = String::from(
        "src/circuit_translation/tests/factories/precompiled_circuits_0.47.0/sha256_4/witness",
    );
    parse_circuit_and_witnesses(&circuit_path, witness_path)
}
