use crate::actions;
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

/// Commands: prove, write_vk, verify.
///     prove -c circuit/path -w witness/path -o output/proof/path
///     write_vk -b circuit/path -o output/verification/key/path
///     verify -k verification/key/path -p proof/path

pub fn parse_commands() {
    let prove_command = _create_prove_command();
    let write_vk_command = _create_write_vk_command();
    let verify_command = _create_verify_command();

    let main_command = Command::new("plonky2_backend")
        .subcommand_required(true)
        .subcommand(prove_command.clone())
        .subcommand(write_vk_command.clone())
        .subcommand(verify_command.clone());

    _match_command_values(
        prove_command,
        write_vk_command,
        verify_command,
        main_command,
    );
}

fn _match_command_values(
    prove_command: Command,
    write_vk_command: Command,
    verify_command: Command,
    main_command: Command,
) {
    let matches = main_command.get_matches();
    if let Some(subcommand_matches) = matches.subcommand_matches(prove_command.get_name()) {
        let circuit_path = _get_argument_value(subcommand_matches, _prove_argument_circuit_path());
        let witness_path = _get_argument_value(subcommand_matches, _prove_argument_witness_path());
        let output_path = _get_argument_value(subcommand_matches, _prove_argument_output_path());

        _execute_prove_command(circuit_path, witness_path, output_path);
    } else if let Some(subcommand_matches) = matches.subcommand_matches(write_vk_command.get_name())
    {
        let circuit_path =
            _get_argument_value(subcommand_matches, _write_vk_argument_circuit_path());
        let output_path = _get_argument_value(subcommand_matches, _write_vk_argument_output_path());

        _execute_write_vk_command(circuit_path, output_path);
    } else if let Some(subcommand_matches) = matches.subcommand_matches(verify_command.get_name()) {
        let vk_path = _get_argument_value(subcommand_matches, _verify_argument_vk_path());
        let proof_path = _get_argument_value(subcommand_matches, _verify_argument_proof());

        _execute_verify_command(vk_path, proof_path);
    }
}

fn create_command_argument(
    argument_id: &'static str,
    short_identifier: char,
    long_identifier: &'static str,
    short_help: &'static str,
    long_help: &'static str,
) -> Arg {
    Arg::new(argument_id)
        .help(short_help)
        .long_help(long_help)
        .short(short_identifier)
        .long(long_identifier)
        .required(true)
        .action(clap::ArgAction::Set)
        .value_parser(value_parser!(PathBuf))
}

fn create_command_from_arguments(command_name: &'static str, args: Vec<Arg>) -> Command {
    args.iter()
        .fold(Command::new(command_name), |acc_command, arg| {
            acc_command.arg(arg)
        })
}

fn _get_argument_value(subcommand_matches: &ArgMatches, argument: Arg) -> &PathBuf {
    subcommand_matches
        .get_one::<PathBuf>(argument.get_id().to_string().as_str())
        .expect("Value for command not found")
}

fn _create_prove_command() -> Command {
    let prove_command_name = "prove";
    let prove_command = create_command_from_arguments(
        prove_command_name,
        vec![
            _prove_argument_circuit_path(),
            _prove_argument_witness_path(),
            _prove_argument_output_path(),
        ],
    );
    prove_command
}

fn _create_write_vk_command() -> Command {
    let write_vk_command_name = "write_vk";
    let prove_command = create_command_from_arguments(
        write_vk_command_name,
        vec![
            _write_vk_argument_circuit_path(),
            _write_vk_argument_output_path(),
        ],
    );
    prove_command
}

fn _create_verify_command() -> Command {
    let verify_command_name = "verify";
    let prove_command = create_command_from_arguments(
        verify_command_name,
        vec![_verify_argument_vk_path(), _verify_argument_proof()],
    );
    prove_command
}

fn _prove_argument_circuit_path() -> Arg {
    let argument_id = "circuit_path";
    let short_command_identifier = 'c';
    let long_command_identifier = "circuit-path";
    let short_help = "Path to the generated ACIR circuit";
    let long_help = "";
    create_command_argument(
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
    create_command_argument(
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
    create_command_argument(
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
    create_command_argument(
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
    create_command_argument(
        argument_id,
        short_command_identifier,
        long_command_identifier,
        short_help,
        long_help,
    )
}

fn _verify_argument_vk_path() -> Arg {
    let argument_id = "vk_path";
    let short_command_identifier = 'k';
    let long_command_identifier = "vk-path";
    let short_help = "Path to the verification key";
    let long_help = "";
    create_command_argument(
        argument_id,
        short_command_identifier,
        long_command_identifier,
        short_help,
        long_help,
    )
}

fn _verify_argument_proof() -> Arg {
    let argument_id = "proof_path";
    let short_command_identifier = 'p';
    let long_command_identifier = "proof-path";
    let short_help = "Path to the proof";
    let long_help = "";
    create_command_argument(
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
    }
    .run();
}

fn _execute_write_vk_command(circuit_path: &PathBuf, output_path: &PathBuf) {
    actions::write_vk_action::WriteVKAction {
        acir_program_json_path: String::from(circuit_path.to_str().unwrap()),
        vk_path_output: String::from(output_path.to_str().unwrap()),
    }
    .run()
}

fn _execute_verify_command(vk_path: &PathBuf, proof_path: &PathBuf) {
    actions::verify_action::VerifyAction {
        proof_path: String::from(proof_path.to_str().unwrap()),
        vk_path: String::from(vk_path.to_str().unwrap()),
    }
    .run()
}
