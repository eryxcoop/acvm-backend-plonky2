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
use noir_and_plonky2_serialization::*;

const D: usize = 2;

type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type CB = CircuitBuilder::<F, D>;

pub mod circuit_translation;
pub mod actions;
pub mod noir_and_plonky2_serialization;

#[global_allocator] // This is a plonky2 recommendation
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: &str = &args.get(1).expect("Must specify a command");
    match command {
        "info" => _print_info_string(),
        "prove" =>  _execute_prove_command(&args),
        "write_vk" => _execute_write_vk_command(&args),
        "verify" => _execute_verify_command(&args),
        other => eprintln!("Invalid command: {:?}", other)
    }
}

fn _execute_prove_command(args: &Vec<String>) {
    let acir_program_path = &args[5];
    let witness_stack_path = &args[7];
    let prove_action = actions::prove_action::ProveAction {
        acir_program_path: acir_program_path.clone(),
        witness_stack_path: witness_stack_path.clone(),
    };
    prove_action.run();
}

fn _execute_verify_command(args: &Vec<String>) {
    let proof_path = &args[5];
    let vk_path = &args[7];
    let verify_action = actions::verify_action::VerifyAction{
        proof_path: proof_path.clone(),
        vk_path: vk_path.clone()};
    verify_action.run()
}

fn _execute_write_vk_command(args: &Vec<String>) {
    let bytecode_path = &args[5];
    let vk_path_output = &args[7];
    let write_vk_action = actions::write_vk_action::WriteVKAction{
        bytecode_path: bytecode_path.clone(),
        vk_path_output: vk_path_output.clone()};
    write_vk_action.run()
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