use criterion::{black_box, criterion_group, criterion_main, Criterion};
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2_backend::circuit_translation::binary_digits_target::BinaryDigitsTarget;
use plonky2_backend::F;
use plonky2_backend::circuit_translation::CB;

fn xxx(){
    let g_zero = F::default();
    let g_one = F::from_canonical_u32(1);
    let size = 4;
    let n = 1;
    let mut input_values = vec![g_zero, g_zero, g_one, g_zero];
    let mut output_values = vec![g_zero, g_zero, g_zero, g_one];

    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let bits = (0..size)
        .into_iter()
        .map(|_| circuit_builder.add_virtual_bool_target_unsafe())
        .collect();

    let binary_input = BinaryDigitsTarget { bits };
    let rotated_bits = BinaryDigitsTarget::rotate_right(&binary_input, n, &mut circuit_builder);

    let mut partial_witnesses = PartialWitness::<F>::new();
    input_values.reverse();
    output_values.reverse();
    for i in 0..size {
        partial_witnesses.set_target(binary_input.bits[i].target, input_values[i]);
        partial_witnesses.set_target(rotated_bits.bits[i].target, output_values[i]);
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter(|| xxx()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
