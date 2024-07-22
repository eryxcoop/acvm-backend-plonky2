extern crate core;

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::vec::Vec;

use clap::{Arg, Command, value_parser};
use jemallocator::Jemalloc;

use circuit_translation::*;
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

fn create_argument(argument_id: &'static str,
                   short_identifier: char,
                   long_identifier: &'static str,
                   short_help: &'static str,
                   long_help: &'static str) -> Arg {
    Arg::new(argument_id)
        .help(short_help)
        .long_help(long_help)
        .short(short_identifier)
        .long(long_identifier)
        .required(true)
        .action(clap::ArgAction::Set)
        .value_parser(value_parser!(PathBuf))
}

fn create_command_with_args(command_name: &'static str, args: Vec<Arg>) -> Command {
    args.iter().fold(
        Command::new(command_name),
        |acc_command, arg| acc_command.arg(arg)
    )
}

fn create_command_with_subcommands(command_name: &'static str, subcommands: Vec<Command>) -> Command {
    subcommands.iter().fold(
        Command::new(command_name),
        |acc_command, subcommand| acc_command.subcommand(subcommand)
    )
}

fn main() {
    let prove_command = _create_prove_command();
    let write_vk_command = _create_write_vk_command();

    let main_command = Command::new("myprog")
        .subcommand_required(true)
        .subcommand(prove_command.clone())
        .subcommand(write_vk_command.clone());


    let matches = main_command.get_matches();
    if let Some(subcommand_matches) = matches.subcommand_matches(prove_command.get_name()) {
        let circuit_path = subcommand_matches.get_one::<PathBuf>(
            _prove_argument_circuit_path().get_id().to_string().as_str()).expect("---");
        let witness_path = subcommand_matches.get_one::<PathBuf>(
            _prove_argument_witness_path().get_id().to_string().as_str()).expect("---");
        let output_path = subcommand_matches.get_one::<PathBuf>(
            _prove_argument_output_path().get_id().to_string().as_str()).expect("---");

        _execute_prove_command(circuit_path, witness_path, output_path);

    } else if let Some(subcommand_matches) = matches.subcommand_matches(write_vk_command.get_name()) {
        let circuit_path = subcommand_matches.get_one::<PathBuf>(
            _write_vk_argument_circuit_path().get_id().to_string().as_str()).expect("---");
        let output_path = subcommand_matches.get_one::<PathBuf>(
            _write_vk_argument_output_path().get_id().to_string().as_str()).expect("---");

        _execute_write_vk_command(circuit_path, output_path);

    }
}

fn _create_prove_command() -> Command {
    let prove_command_name = "prove";
    let prove_command = create_command_with_args(
        prove_command_name,
        vec![
            _prove_argument_circuit_path(),
            _prove_argument_witness_path(),
            _prove_argument_output_path()
        ]
    );
    prove_command
}

fn _create_write_vk_command() -> Command {
    let write_vk_command_name = "write_vk";
    let prove_command = create_command_with_args(
        write_vk_command_name,
        vec![
            _write_vk_argument_circuit_path(),
            _write_vk_argument_output_path(),
        ]
    );
    prove_command
}



fn _prove_argument_circuit_path() -> Arg {
    let argument_id = "circuit_path";
    let short_command_identifier = 'c';
    let long_command_identifier = "circuit-path";
    let short_help = "Path to the generated ACIR circuit";
    let long_help = "";
    create_argument(
        argument_id,
        short_command_identifier,
        long_command_identifier,
        short_help,
        long_help,
    )
}

fn _prove_argument_witness_path() -> Arg {
    let argument_id = "witness_path";
    let short_command_identifier = 'w';
    let long_command_identifier = "witness-path";
    let short_help = "Path to the generated witness values";
    let long_help = "";
    create_argument(
        argument_id,
        short_command_identifier,
        long_command_identifier,
        short_help,
        long_help,
    )
}

fn _prove_argument_output_path() -> Arg {
    let argument_id = "output_path";
    let short_command_identifier = 'o';
    let long_command_identifier = "output-path";
    let short_help = "Path where the generated proof is to be stored";
    let long_help = "";
    create_argument(
        argument_id,
        short_command_identifier,
        long_command_identifier,
        short_help,
        long_help,
    )
}

fn _write_vk_argument_circuit_path() -> Arg {
    let argument_id = "circuit_path";
    let short_command_identifier = 'b';
    let long_command_identifier = "circuit-path";
    let short_help = "Path to the generated ACIR circuit";
    let long_help = "";
    create_argument(
        argument_id,
        short_command_identifier,
        long_command_identifier,
        short_help,
        long_help,
    )
}

fn _write_vk_argument_output_path() -> Arg {
    let argument_id = "output_path";
    let short_command_identifier = 'o';
    let long_command_identifier = "output-path";
    let short_help = "Path to the generated verification key";
    let long_help = "";
    create_argument(
        argument_id,
        short_command_identifier,
        long_command_identifier,
        short_help,
        long_help,
    )
}

fn _execute_prove_command(circuit_path: &PathBuf, witness_path: &PathBuf, output_path: &PathBuf) {
    actions::prove_action::ProveAction {
        acir_program_json_path: String::from(circuit_path.to_str().unwrap()),
        witness_stack_zip_path: String::from(witness_path.to_str().unwrap()),
        resulting_proof_file_path: String::from(output_path.to_str().unwrap()),
    }.run();
}

fn _execute_write_vk_command(circuit_path: &PathBuf, output_path: &PathBuf) {
    actions::write_vk_action::WriteVKAction {
        acir_program_json_path: String::from(circuit_path.to_str().unwrap()),
        vk_path_output: String::from(output_path.to_str().unwrap()),
    }.run()
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
