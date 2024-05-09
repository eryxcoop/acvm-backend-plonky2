use std::env;
// use std::fs;
use std::fs::File;
use std::vec::Vec;
use std::io::Read;
use acir::circuit::Program;
// use serde::Deserialize;

fn deserialize_program_within_file_path(acir_program_path: &String) -> Program {
    let mut file = File::open(acir_program_path).expect("There was a problem reading the file");
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    let file_contents_slice: &[u8] = &buffer;
    let program = Program::deserialize_program(file_contents_slice);
    program.unwrap()
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "info" {

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

    } else if args.len() > 1 && args[1] == "prove" {
        // println!("If you are reading this you probably did some cryptohacks in the past");

        // let crs_path = &args[3];
        // let witness_path = &args[7];
        let acir_program: Program = deserialize_program_within_file_path(&args[5]);

    } else {
        println!("If you're watching this you probably shouldn't want to");
    }
}