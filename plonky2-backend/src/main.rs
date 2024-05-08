use std::env;
use std::fs;
use acir::circuit::Program;
//use serde::Deserialize;

/*fn parse_program(path: &String){


    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("{}", contents);

    let program: Program = serde::deserialize_xxx()
}*/


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

        let crs_path = &args[3];
        let bytecode_path = &args[5];
        let witness_path = &args[7];

        //parse_program(&bytecode_path);

        //println!("{:?}", crs_path);
        //println!("{:?}", bytecode_path);
        //println!("{:?}", witness_path);

    } else {
        println!("If you're watching this you probably shouldn't want to");
    }
}