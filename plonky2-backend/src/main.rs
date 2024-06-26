extern crate core;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::vec::Vec;

use circuit_translation::*;
use jemallocator::Jemalloc;

use noir_and_plonky2_serialization::*;
use plonky2::plonk::circuit_data::VerifierCircuitData;
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig};
use plonky2::plonk::proof::CompressedProofWithPublicInputs;
use plonky2::util::serialization::DefaultGateSerializer;

use crate::circuit_translation::CircuitBuilderFromAcirToPlonky2;

const D: usize = 2;

type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

pub mod actions;
pub mod circuit_translation;
pub mod noir_and_plonky2_serialization;

#[global_allocator] // This is a plonky2 recommendation
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: &str = &args.get(1).expect("Must specify a command");
    match command {
        "prove" => _execute_prove_command(&args),
        "write_vk" => _execute_write_vk_command(&args),
        "verify" => _execute_verify_command(&args),
        other => eprintln!("Invalid command: {:?}", other),
    }
}

fn _execute_prove_command(args: &Vec<String>) {
    let acir_program_json_path = &args[3];
    let witness_stack_zip_path = &args[5];
    let resulting_proof_file_path = &args[7];
    let prove_action = actions::prove_action::ProveAction {
        acir_program_json_path: acir_program_json_path.clone(),
        witness_stack_zip_path: witness_stack_zip_path.clone(),
        resulting_proof_file_path: resulting_proof_file_path.clone(),
    };
    prove_action.run();
}

fn _execute_write_vk_command(args: &Vec<String>) {
    let acir_program_json_path = &args[3];
    let vk_path_output = &args[5];
    let write_vk_action = actions::write_vk_action::WriteVKAction {
        acir_program_json_path: acir_program_json_path.clone(),
        vk_path_output: vk_path_output.clone(),
    };
    write_vk_action.run()
}

fn _execute_verify_command(args: &Vec<String>) {
    let vk_path = &args[3];
    let proof_path = &args[5];
    let verify_action = actions::verify_action::VerifyAction {
        proof_path: proof_path.clone(),
        vk_path: vk_path.clone(),
    };
    verify_action.run()
}
