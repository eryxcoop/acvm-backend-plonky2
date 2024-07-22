extern crate core;

use std::fs::File;
use std::io::{Read, Write};
use std::vec::Vec;

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
pub mod argument_parsing;
pub mod circuit_translation;
pub mod noir_and_plonky2_serialization;

#[global_allocator] // This is a plonky2 recommendation
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    argument_parsing::parse_commands();
}
