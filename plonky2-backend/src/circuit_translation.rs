
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

fn field_element_to_goldilocks_field(fe: &FieldElement) -> GoldilocksField {
    let fe_as_big_uint = BigUint::from_bytes_be(&fe.to_be_bytes() as &[u8]);
    GoldilocksField::from_noncanonical_biguint(fe_as_big_uint)
}

fn translate_assert_zero(builder: &mut CB, expression: &Expression, witness_target_map: &mut HashMap<Witness, Target>) {
    println!("{:?}", expression);
    assert_eq!(0, expression.mul_terms.len());
    let linear_combinations = &expression.linear_combinations;
    assert_eq!(1, linear_combinations.len());

    let (f_single_multiply_factor, witness) = &linear_combinations[0];

    let target = if let Some(target) = witness_target_map.get(witness) {
        *target
    } else {
        let new_target = builder.add_virtual_target();
        witness_target_map.insert(*witness, new_target);
        new_target
    };

    // let target_constant = builder.constant(g_constant);
    // let g_single_multiply_factor = field_element_to_goldilocks_field(f_single_multiply_factor); // Hardcodeado el factor 1
    // let g_zero = GoldilocksField::default();
    // let result_target = builder.mul_const(g_single_multiply_factor, target);
    // let result_target = builder.arithmetic(
    //     g_single_multiply_factor,
    //     g_zero,
    //     target,
    //     target,
    //     target_constant);
    let g_constant = field_element_to_goldilocks_field(&expression.q_c);
    let g_factor = field_element_to_goldilocks_field(f_single_multiply_factor);
    let target_1 = builder.mul_const(g_factor, target);
    let result_target = builder.add_const(target_1, g_constant);
    builder.assert_zero(result_target);
}


fn generate_plonky2_circuit_from_acir_circuit(circuit: &Circuit) -> (CircuitData<GoldilocksField, C, 2>, HashMap<Witness, Target>){
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CB::new(config);

    let public_input = circuit.public_parameters.0.first().unwrap();
    let public_input_target = builder.add_virtual_target();
    builder.register_public_input(public_input_target);

    let mut witness_target_map: HashMap<Witness, Target> = HashMap::new();
    witness_target_map.insert(*public_input, public_input_target);

    for opcode in &circuit.opcodes {
        match opcode {
            AssertZero(expr) => translate_assert_zero(&mut builder, &expr, &mut witness_target_map),
            _ => { () }
        }
    }

    (builder.build::<C>(), witness_target_map)

}


#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use acir::circuit::Opcode;
    use plonky2::field::types::Field;
    use plonky2::iop::witness::WitnessWrite;
    use super::*;
    #[test]
    fn test_plonky2_vm_can_traslate_the_assert_x_equals_zero_program(){
        // Given
        let public_input_witness = Witness(0);
        let only_opcode = x_equals_0_opcode(public_input_witness);
        let circuit = circuit_with_single_opcode(only_opcode, public_input_witness);

        // When
        let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

        // Then
        let mut witnesses = PartialWitness::<GoldilocksField>::new();
        let g_zero= GoldilocksField::default();
        let public_input_plonky2_target = witness_target_map.get(&public_input_witness).unwrap();
        witnesses.set_target(*public_input_plonky2_target, g_zero);
        let proof = circuit_data.prove(witnesses).unwrap();
        assert_eq!(g_zero, proof.public_inputs[0]);
    }

    fn circuit_with_single_opcode(only_expr: Opcode, public_input_witness: Witness) -> Circuit {
        Circuit {
            current_witness_index: 0,
            expression_width: ExpressionWidth::Unbounded,
            opcodes: vec![only_expr],
            private_parameters: BTreeSet::new(),
            public_parameters: PublicInputs(BTreeSet::from_iter(vec![public_input_witness])),
            return_values: PublicInputs(BTreeSet::new()),
            assert_messages: Default::default(),
            recursive: false,
        }
    }

    fn x_equals_0_opcode(public_input_witness: Witness) -> Opcode {
        AssertZero(Expression {
            mul_terms: Vec::new(),
            linear_combinations: vec![(FieldElement::one(), public_input_witness)],
            q_c: FieldElement::zero()
        })
    }

    #[test]
    fn test_plonky2_vm_can_traslate_the_assert_x_equals_constant_program(){
        // Given
        let public_input_witness = Witness(0);
        let only_opcode = x_equals_4_opcode(public_input_witness);
        let circuit = circuit_with_single_opcode(only_opcode, public_input_witness);

        // When
        let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

        // Then
        let mut witnesses = PartialWitness::<GoldilocksField>::new();
        let four = GoldilocksField::from_canonical_u64(4);
        let public_input_plonky2_target = witness_target_map.get(&public_input_witness).unwrap();
        witnesses.set_target(*public_input_plonky2_target, four);
        let proof = circuit_data.prove(witnesses).unwrap();
        assert_eq!(four, proof.public_inputs[0]);
    }

    fn x_equals_4_opcode(public_input_witness: Witness) -> Opcode {
        AssertZero(Expression {
            mul_terms: Vec::new(),
            linear_combinations: vec![(FieldElement::one(), public_input_witness)],
            q_c: -FieldElement::from_hex("0x04").unwrap()
        })
    }

    #[test]
    fn test_plonky2_vm_can_traslate_the_assert_c_times_x_equals_constant_program() {
        // Given
        let public_input_witness = Witness(0);
        let only_opcode = x_times_3_equals_12_opcode(public_input_witness);
        let circuit = circuit_with_single_opcode(only_opcode, public_input_witness);

        // When
        let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

        // Then
        let mut witnesses = PartialWitness::<GoldilocksField>::new();
        let four = GoldilocksField::from_canonical_u64(4);
        let public_input_plonky2_target = witness_target_map.get(&public_input_witness).unwrap();
        witnesses.set_target(*public_input_plonky2_target, four);
        let proof = circuit_data.prove(witnesses).unwrap();
        assert_eq!(four, proof.public_inputs[0]);
    }

    fn x_times_3_equals_12_opcode(public_input_witness: Witness) -> Opcode {
        AssertZero(Expression {
            mul_terms: Vec::new(),
            linear_combinations: vec![(FieldElement::from_hex("0x03").unwrap(), public_input_witness)],
            q_c: -FieldElement::from_hex("0x0C").unwrap()
        })
    }

}
