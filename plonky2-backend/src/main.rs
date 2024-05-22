extern crate core;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::vec::Vec;

use acir::circuit::{Circuit, Program};
use acir::FieldElement;
use acir::native_types::{Witness, WitnessMap, WitnessStack};
use jemallocator::Jemalloc;
use num_bigint::BigUint;


pub mod circuit_translation;
pub mod prove_action;

#[global_allocator] // This is a plonky2 recommendation
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = get_command(&args).unwrap();

    if command.eq("info") {
        _print_info_string();
    } else if command.eq("prove") {
        _execute_prove_command(&args);

    } else {
        println!("If you're watching this you probably shouldn't want to");
    }
}

fn get_command(args: &Vec<String>) -> Result<&String, &str> {
    if args.len() == 1 {
        Err("You must specify a command")
    } else {
        Ok(&args[1])
    }
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
    println!("{:?}", proof);
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