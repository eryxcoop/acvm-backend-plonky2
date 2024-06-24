use super::*;
use crate::circuit_translation::*;

pub fn deserialize_verifying_key_within_file_path(verifying_key_path: &String) -> VerifierCircuitData<F,C,D> {
    let buffer = read_file_to_bytes(verifying_key_path);
    let gate_serializer = DefaultGateSerializer;
    VerifierCircuitData::from_bytes(buffer, &gate_serializer).unwrap()
}

pub fn deserialize_proof_within_file_path(proof_path: &String, verifier_data: &VerifierCircuitData<F,C,D>) -> CompressedProofWithPublicInputs<F, C, D> {
    let buffer = read_file_to_bytes(proof_path);
    let common_circuit_data = &verifier_data.common;
    let proof: CompressedProofWithPublicInputs<F, C, D> = CompressedProofWithPublicInputs::from_bytes(
        buffer, common_circuit_data).unwrap();
    proof
}

pub fn read_file_to_bytes(file_path: &String) -> Vec<u8> {
    let mut file = File::open(file_path).expect("There was a problem reading the file");
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    return buffer
}

pub fn deserialize_program_within_file_path(acir_program_path: &String) -> Program {
    let buffer = read_file_to_bytes(acir_program_path);
    let file_contents_slice: &[u8] = &buffer;
    let program = Program::deserialize_program(file_contents_slice);
    program.unwrap()
}

pub fn deserialize_witnesses_within_file_path(witnesses_path: &String) -> WitnessStack {
    let buffer = read_file_to_bytes(witnesses_path);
    let file_contents_slice: &[u8] = &buffer;
    let witness_stack = WitnessStack::try_from(file_contents_slice);
    witness_stack.unwrap()
}

pub fn write_bytes_to_file_path(bytes: Vec<u8>, path: &String){
    let mut file = File::create(path).expect("Failed to create file for vk");
    file.write_all(&bytes).expect("Failed to write vk into file");
}