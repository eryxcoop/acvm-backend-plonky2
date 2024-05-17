extern crate core;

pub mod circuit_translation;

use std::env;
use std::fs::File;
use std::vec::Vec;
use std::io::Read;
use acir::circuit::Program;
use acir::native_types::WitnessStack;
use jemallocator::Jemalloc;

#[global_allocator] // This is a plonky2 recommendation
static GLOBAL: Jemalloc = Jemalloc;


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


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "info" {
        _print_info_string();
    } else if args.len() > 1 && args[1] == "prove" {
        // let crs_path = &args[3];
        let acir_program: Program = deserialize_program_within_file_path(&args[5]);
        let witness_stack: WitnessStack = deserialize_witnesses_within_file_path(&args[7]);
        //println!("{:?}", acir_program);
        println!("{:?}", witness_stack);

        let functions = acir_program.functions;
        let circuit = &functions[0];
        println!("{:?}", circuit);


    } else {
        println!("If you're watching this you probably shouldn't want to");
    }
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