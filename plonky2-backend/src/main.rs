extern crate core;

#[cfg(test)]
mod integration_tests;

use std::collections::HashMap;
use std::{env, io};
use std::fs::File;
use std::io::{Read, Write};
use std::vec::Vec;

use jemallocator::Jemalloc;
use num_bigint::BigUint;

use acir::circuit::{Circuit, Program};
use acir::FieldElement;
use acir::native_types::{Witness, WitnessMap, WitnessStack};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{VerifierCircuitData, VerifierOnlyCircuitData};
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig};
use plonky2::plonk::proof::CompressedProofWithPublicInputs;
use plonky2::util::serialization::DefaultGateSerializer;

use crate::circuit_translation::CircuitBuilderFromAcirToPlonky2;
use crate::write_vk_action::WriteVKAction;

const D: usize = 2;

type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type CB = CircuitBuilder::<F, D>;

pub mod circuit_translation;
pub mod prove_action;
pub mod write_vk_action;

#[global_allocator] // This is a plonky2 recommendation
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = get_command(&args).unwrap();

    if command.eq("info") {
        _print_info_string();
    } else if command.eq("prove") {
        _execute_prove_command(&args);
    } else if command.eq("write_vk") {
        _execute_write_vk_command(&args);
    } else if command.eq("verify"){
        _execute_verify_command(&args);
    } else {
        eprintln!("If you're watching this you probably shouldn't want to");
    }
}

fn get_command(args: &Vec<String>) -> Result<&String, &str> {
    if args.len() == 1 {
        Err("You must specify a command")
    } else {
        Ok(&args[1])
    }
}

fn deserialize_verifying_key_within_file_path(verifying_key_path: &String) -> VerifierCircuitData<F,C,D> {
    let buffer = read_file_to_bytes(verifying_key_path);
    let gate_serializer = DefaultGateSerializer;
    VerifierCircuitData::from_bytes(buffer, &gate_serializer).unwrap()
}

fn deserialize_proof_within_file_path(proof_path: &String, verifier_data: &VerifierCircuitData<F,C,D>) -> CompressedProofWithPublicInputs<F, C, D> {
    let buffer = read_file_to_bytes(proof_path);
    let common_circuit_data = &verifier_data.common;
    let proof: CompressedProofWithPublicInputs<F, C, D> = CompressedProofWithPublicInputs::from_bytes(
        buffer, common_circuit_data).unwrap();
    proof
}

fn read_file_to_bytes(file_path: &String) -> Vec<u8> {
    let mut file = File::open(file_path).expect("There was a problem reading the file");
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    return buffer
}

fn deserialize_program_within_file_path(acir_program_path: &String) -> Program {
    let buffer = read_file_to_bytes(acir_program_path);
    let file_contents_slice: &[u8] = &buffer;
    let program = Program::deserialize_program(file_contents_slice);
    program.unwrap()
}

fn deserialize_witnesses_within_file_path(witnesses_path: &String) -> WitnessStack {
    let buffer = read_file_to_bytes(witnesses_path);
    let file_contents_slice: &[u8] = &buffer;
    let witness_stack = WitnessStack::try_from(file_contents_slice);
    witness_stack.unwrap()
}

fn _execute_prove_command(args: &Vec<String>) {
    let acir_program: Program = deserialize_program_within_file_path(&args[5]);
    let mut witness_stack: WitnessStack = deserialize_witnesses_within_file_path(&args[7]);
    let prove_action = prove_action::ProveAction;
    let proof = prove_action.run(acir_program, witness_stack);

    let mut stdout = io::stdout();
    stdout.write_all(&proof).expect("Failed to write in stdout");
    stdout.flush().expect("Failed to flush");
}

fn _execute_verify_command(args: &Vec<String>) {
    let proof_path = &args[5];
    let vk_path = &args[7];
    let verifier_data = deserialize_verifying_key_within_file_path(vk_path);
    let mut compressed_proof = deserialize_proof_within_file_path(proof_path, &verifier_data);
    verifier_data.verify_compressed(compressed_proof).expect("Verification failed");
}

fn _execute_write_vk_command(args: &Vec<String>) {
    let bytecode_path = &args[5];
    let vk_path_output = &args[7];
    let write_vk_action = WriteVKAction{
        bytecode_path: bytecode_path.clone(),
        vk_path_output: vk_path_output.clone()};
    write_vk_action.run()
}

fn write_bytes_to_file_path(bytes: Vec<u8>, path: &String){
    let mut file = File::create(path).expect("Failed to create file for vk");
    file.write_all(&bytes).expect("Failed to write vk into file");
}

fn _print_info_string() {
    println!(r#"{{
            "opcodes_supported": [],
            "black_box_functions_supported": [],
            "status": "ok",
            "message": "This is a dummy JSON response.",
            "language": {{
                "name": "PLONK-CSAT",
                "width": 3
            }}
        }}"#);
}