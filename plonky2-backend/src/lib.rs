extern crate core;

use jemallocator::Jemalloc;

use circuit_translation::*;
use noir_and_plonky2_serialization::*;
use plonky2::plonk::circuit_data::VerifierCircuitData;
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig};
use plonky2::plonk::proof::CompressedProofWithPublicInputs;

const D: usize = 2;

pub type C = KeccakGoldilocksConfig;
pub(crate) type F = <C as GenericConfig<D>>::F;
pub mod actions;
pub mod argument_parsing;
pub mod circuit_translation;
pub mod noir_and_plonky2_serialization;
pub mod plonky2_ecdsa;
pub mod binary_digits_target;

#[global_allocator] // This is a plonky2 recommendation
static GLOBAL: Jemalloc = Jemalloc;
