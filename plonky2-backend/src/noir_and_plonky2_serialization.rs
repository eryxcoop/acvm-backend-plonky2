use super::*;

use base64;
use base64::Engine;
use serde_json;
use std::fs::File;
use std::io::{Read, Write};
use std::vec::Vec;

pub fn deserialize_verifying_key_within_file_path(
    verifying_key_path: &String,
) -> VerifierCircuitData<F, C, D> {
    let buffer = read_file_to_bytes(verifying_key_path);
    let gate_serializer = DefaultGateSerializer;
    VerifierCircuitData::from_bytes(buffer, &gate_serializer).unwrap()
}

pub fn deserialize_proof_within_file_path(
    proof_path: &String,
    verifier_data: &VerifierCircuitData<F, C, D>,
) -> CompressedProofWithPublicInputs<F, C, D> {
    let buffer = read_file_to_bytes(proof_path);
    let common_circuit_data = &verifier_data.common;
    let proof: CompressedProofWithPublicInputs<F, C, D> =
        CompressedProofWithPublicInputs::from_bytes(buffer, common_circuit_data).unwrap();
    proof
}

pub fn read_file_to_bytes(file_path: &String) -> Vec<u8> {
    let mut file = File::open(file_path).expect("There was a problem reading the file");
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    return buffer;
}

pub fn deserialize_program_within_file_path(acir_program_path: &String) -> Program {
    let mut file = File::open(acir_program_path).expect("There was a problem opening the file");
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)
        .expect("There was a problem reading the file content");
    let json_str: &str = &json_string;
    let json: serde_json::Value =
        serde_json::from_str(json_str).expect("There was a problem parsing the json program");
    let Some(bytecode_str) = json["bytecode"].as_str() else {
        panic!("Expected a different circuit format")
    };
    let bytecode: &[u8] = &base64::prelude::BASE64_STANDARD
        .decode(bytecode_str)
        .expect("There was a problem decoding the program from base 64");
    let program = Program::deserialize_program(bytecode);
    program.unwrap()
}

pub fn deserialize_witnesses_within_file_path(mut witnesses_path: String) -> WitnessStack {
    witnesses_path.push_str(".gz");
    let file_content: &[u8] = &read_file_to_bytes(&witnesses_path);
    let witness_stack = WitnessStack::try_from(file_content);
    witness_stack.unwrap()
}

pub fn write_bytes_to_file_path(bytes: Vec<u8>, path: &String) {
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(&bytes).expect("Failed to write file");
}
