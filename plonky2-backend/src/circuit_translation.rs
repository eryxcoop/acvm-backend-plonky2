
use std::collections::{HashMap};
use std::error::Error;
use acir::circuit::{Circuit};
use acir::circuit::{ExpressionWidth, PublicInputs};
use acir::circuit::Opcode::AssertZero;
use acir::FieldElement;
use acir::native_types::{Expression, Witness};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::{Field, Field64};
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig};
use num_bigint::BigUint;
use plonky2::iop::witness::PartialWitness;
use plonky2::iop::witness:: WitnessWrite;
use plonky2::plonk::proof::ProofWithPublicInputs;

const D: usize = 2;
type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type CB = CircuitBuilder::<F, D>;

fn translate_assert_zero(builder: &mut CB, expression: &Expression, witness_target_map: &mut HashMap<Witness, Target>) {
    println!("{:?}", expression);
    assert_eq!(0, expression.mul_terms.len());
    assert_eq!(FieldElement::zero(), expression.q_c);
    let linear_combinations = &expression.linear_combinations;
    assert_eq!(1, linear_combinations.len());
    let (field_element, witness) = &linear_combinations[0];

    let target = if let Some(target) = witness_target_map.get(witness) {
        *target
    } else {
        todo!();

        // let new_target = builder.add_virtual_target();
        // witness_target_map.insert(*witness, new_target);
        // new_target
    };

    let g_field_element= GoldilocksField::from_canonical_u64(1);

    let result_target = builder.mul_const(g_field_element, target);
    builder.assert_zero(result_target);
}


fn generate_plonky2_proof_from_acir_circuit(circuit: &Circuit) -> (){
    let config = CircuitConfig { zero_knowledge: true, ..CircuitConfig::default() };
    let mut builder = CB::new(config);

    let public_input_1 = circuit.public_parameters.0.first().unwrap();
    let target_1 = builder.add_virtual_target();
    let mut witness_target_map: HashMap<Witness, Target> = HashMap::new();
    witness_target_map.insert(*public_input_1, target_1);

    for opcode in &circuit.opcodes {
        match opcode {
            AssertZero(expr) => translate_assert_zero(&mut builder, &expr, &mut witness_target_map),
            _ => { () }
        }
    }

    let circuit_data = builder.build::<C>();
    let mut witnesses = PartialWitness::<GoldilocksField>::new();
    let g_zero= GoldilocksField::default();
    witnesses.set_target(target_1, g_zero);
    circuit_data.prove(witnesses);
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use plonky2::field::types::Field;
    use plonky2::iop::witness::WitnessWrite;
    use super::*;
    #[test]
    fn test_plonky2_vm_can_traslate_the_most_basic_assert_zero_program(){
        // Given
        let only_expr = Expression {mul_terms: Vec::new(),
                                    linear_combinations: vec![(FieldElement::one(), Witness(0))],
                                    q_c: FieldElement::zero() };

        let circuit = Circuit {
            current_witness_index: 0,
            expression_width: ExpressionWidth::Unbounded,
            opcodes: vec![AssertZero(only_expr)],
            private_parameters: BTreeSet::new(),
            public_parameters: PublicInputs(BTreeSet::from_iter(vec![Witness(0)])),
            return_values: PublicInputs(BTreeSet::new()),
            assert_messages: Default::default(),
            recursive: false,
        };

        // When
        generate_plonky2_proof_from_acir_circuit(&circuit);
        // Plonky2 doesn't panic
    }

    #[test]
    fn test_plonky2_vm_can_translate_an_assert_zero_program_with_constant_multiplication(){
        // Given
        let only_expr = Expression {mul_terms: Vec::new(),
            linear_combinations: vec![(FieldElement::from_hex("0x04").unwrap(), Witness(0))],
            q_c: FieldElement::zero() };

        let circuit = Circuit {
            current_witness_index: 0,
            expression_width: ExpressionWidth::Unbounded,
            opcodes: vec![AssertZero(only_expr)],
            private_parameters: BTreeSet::new(),
            public_parameters: PublicInputs(BTreeSet::from_iter(vec![Witness(0)])),
            return_values: PublicInputs(BTreeSet::new()),
            assert_messages: Default::default(),
            recursive: false,
        };

        // When
        generate_plonky2_proof_from_acir_circuit(&circuit);
        // Plonky2 doesn't panic
    }

    #[test]
    fn test_xxxxxx() {
        let config = CircuitConfig { zero_knowledge: true, ..CircuitConfig::default() };
        let mut builder = CB::new(config);
        // let target = builder.add_virtual_target();
        // builder.assert_zero(target);

        let mut witnesses = PartialWitness::<GoldilocksField>::new();
        // let one = GoldilocksField::from_canonical_u64(0);
        // witnesses.set_target(target, one);
        let circuit_data = builder.build::<C>();
        // println!("{:#?}", circuit_data);
        let proof = circuit_data.prove(witnesses);
        // println!("{:?}", proof);
    }
}
