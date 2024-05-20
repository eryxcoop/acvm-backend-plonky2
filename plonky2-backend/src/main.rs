extern crate core;

pub mod circuit_translation;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::vec::Vec;
use std::io::Read;
use acir::circuit::{Circuit, Program};
use acir::native_types::{Witness, WitnessMap, WitnessStack};
use jemallocator::Jemalloc;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::proof::ProofWithPublicInputs;


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
        let mut witness_stack: WitnessStack = deserialize_witnesses_within_file_path(&args[7]);
        let functions = acir_program.functions;
        let circuit = &functions[0];

        let witness_assignment = adapt_witness_stack(&mut witness_stack);
        let (circuit_data, witness_target_map) =
            generate_plonky2_circuit_from_acir_circuit(circuit);
        let proof = generate_plonky2_proof_using_witness_values(
            witness_assignment, &witness_target_map, &circuit_data);
        println!("{:?}", proof);

    } else {
        println!("If you're watching this you probably shouldn't want to");
    }
}

fn adapt_witness_stack(witness_stack: &mut WitnessStack) ->  Vec<(Witness, F)> {
    let mut res: Vec<(Witness, F)> = vec![];
    while(witness_stack.length() != 0){

    }
    for (key, value) in witness_stack {
        res.push((key, value));
    }
    res
}

fn generate_plonky2_circuit_from_acir_circuit(circuit: &Circuit) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>) {
    let mut translator = circuit_translation::CircuitBuilderFromAcirToPlonky2::new();
    translator.translate_circuit(circuit);
    let circuit_translation::CircuitBuilderFromAcirToPlonky2 { builder, witness_target_map } = translator;
    (builder.build::<C>(), witness_target_map)
}

fn generate_plonky2_proof_using_witness_values(witness_assignment: Vec<(Witness, F)>,
                                               witness_target_map: &HashMap<Witness, Target>,
                                               circuit_data: &CircuitData<F, C, 2>) -> ProofWithPublicInputs<GoldilocksField, C, 2> {
    let mut witnesses = PartialWitness::<F>::new();
    for (witness, value) in witness_assignment {
        let plonky2_target = witness_target_map.get(&witness).unwrap();
        witnesses.set_target(*plonky2_target, value);
    }
    circuit_data.prove(witnesses).unwrap()
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