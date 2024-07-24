use std::fmt::Display;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use acir::native_types::Witness;
use acir_field::AcirField;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2_backend::{C, D, F};
use plonky2_backend::circuit_translation::tests::factories::{circuit_parser, utils};

fn generate_circuit_and_witness(program_name: &str) -> (CircuitData<F, C, D>, PartialWitness<F>) {
    let (circuit, mut witnesses) =
        circuit_parser::precompiled_circuit_and_withesses_with_name(program_name);
    let witness_mapping = witnesses.pop().unwrap().witness;

    // print!("{:?}", circuit);

    // When
    let (circuit_data, witness_target_map) =
        utils::generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let mut witness_assignment: Vec<(Witness, F)> = vec![];
    for (witness, value) in witness_mapping {
        witness_assignment.push((witness, F::from_canonical_u64(value.try_to_u64().unwrap())));
    }

    let mut witnesses = PartialWitness::<F>::new();
    for (witness, value) in witness_assignment {
        let plonky2_target = witness_target_map.get(&witness).unwrap();
        witnesses.set_target(*plonky2_target, value);
    }

    (circuit_data, witnesses)
}

fn to_bench(x: &(CircuitData<F, C, D>, PartialWitness<F>)) {
    let (circuit_data, witnesses) = x;
    circuit_data.prove(witnesses.clone());
}


pub fn criterion_benchmark(c: &mut Criterion) {
    let x = generate_circuit_and_witness("basic_memory_write");
    // c.bench_function("test", |b| b.iter(circuit_data.prove(witnesses)));

    c.bench_with_input(BenchmarkId::new("input_example", "some_parameter"), &x, |b, s| {
        b.iter(|| to_bench(s));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
