use std::env;
// use std::fs;
use std::fs::File;
use std::vec::Vec;
use std::io::Read;
use acir::circuit::Program;
use serde::Deserialize;

fn read_file(acir_program_path: &String){
    println!("{:?}", acir_program_path);
    let mut file = File::open(acir_program_path).expect("There was a problem reading the file");
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    println!("{:?}", buffer);

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
        println!("If you are reading this you probably did some cryptohacks in the past");

        // let crs_path = &args[3];
        let bytecode_path = &args[5];
        // let witness_path = &args[7];

        read_file(&args[7]);
        // read_file(&bytecode_path);

        //println!("{:?}", crs_path);
        //println!("{:?}", bytecode_path);
        //println!("{:?}", witness_path);

    } else {
        println!("If you're watching this you probably shouldn't want to");
    }
}