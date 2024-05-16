
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
use std::collections::BTreeSet;
use acir::circuit::Opcode;

const D: usize = 2;
type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type CB = CircuitBuilder::<F, D>;

fn field_element_to_goldilocks_field(fe: &FieldElement) -> F {
    let fe_as_big_uint = BigUint::from_bytes_be(&fe.to_be_bytes() as &[u8]);
    F::from_noncanonical_biguint(fe_as_big_uint)
}

fn _compute_linear_combination_target(builder: &mut CB,
                                      witness_target_map: &HashMap<Witness, Target>,
                                      multiply_constant_factor: &FieldElement,
                                      public_input_witness: &Witness) -> Target{
    let first_public_input_target = *witness_target_map.get(public_input_witness).unwrap();
    let g_first_pi_factor = field_element_to_goldilocks_field(multiply_constant_factor);
    builder.mul_const(g_first_pi_factor, first_public_input_target)
}

fn translate_assert_zero(builder: &mut CB, expression: &Expression, witness_target_map: &mut HashMap<Witness, Target>) {
    println!("{:?}", expression);
    let g_constant = field_element_to_goldilocks_field(&expression.q_c);
    let linear_combinations = &expression.linear_combinations;

    let constant_target = builder.constant(g_constant);
    let mut current_acc_target = constant_target;
    for (f_second_multiply_factor, second_public_input_witness) in linear_combinations {
        let linear_combination_target = _compute_linear_combination_target(builder,
               witness_target_map, f_second_multiply_factor, second_public_input_witness);
        let new_target = builder.add(linear_combination_target, current_acc_target);
        current_acc_target = new_target;
    }

    builder.assert_zero(current_acc_target);
}

fn generate_plonky2_circuit_from_acir_circuit(circuit: &Circuit) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>){
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CB::new(config);
    let mut witness_target_map: HashMap<Witness, Target> = HashMap::new();

    _register_public_parameters_from_acir_circuit(circuit, &mut builder, &mut witness_target_map);

    for opcode in &circuit.opcodes {
        match opcode {
            AssertZero(expr) => translate_assert_zero(&mut builder, &expr, &mut witness_target_map),
            _ => { () }
        }
    }

    (builder.build::<C>(), witness_target_map)
}

fn _register_public_parameters_from_acir_circuit(circuit: &Circuit, mut builder: &mut CircuitBuilder<GoldilocksField, 2>, mut witness_target_map: &mut HashMap<Witness, Target>) {
    let public_parameters_as_list: Vec<Witness> = circuit.public_parameters.0.iter().cloned().collect();
    for public_parameter_witness in public_parameters_as_list {
        _register_new_public_input_from_witness(public_parameter_witness, &mut builder, &mut witness_target_map);
    }
}

fn _register_new_public_input_from_witness(public_input_witness: Witness,
                                           builder: &mut CB,
                                           witness_target_map: &mut HashMap<Witness, Target>) {
    let public_input_target = builder.add_virtual_target();
    builder.register_public_input(public_input_target);
    witness_target_map.insert(public_input_witness, public_input_target);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_plonky2_vm_can_traslate_the_assert_x_equals_zero_program(){
        // Given
        let public_input_witness = Witness(0);
        let only_opcode = x_equals_0_opcode(public_input_witness);
        let circuit = circuit_with_single_opcode(only_opcode, vec![public_input_witness]);

        // When
        let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

        // Then
        let mut witnesses = PartialWitness::<F>::new();
        let g_zero= F::default();
        let public_input_plonky2_target = witness_target_map.get(&public_input_witness).unwrap();
        witnesses.set_target(*public_input_plonky2_target, g_zero);
        let proof = circuit_data.prove(witnesses).unwrap();
        assert_eq!(g_zero, proof.public_inputs[0]);
        circuit_data.verify(proof).expect("Verification failed");
    }

    fn circuit_with_single_opcode(only_expr: Opcode, public_input_witnesses: Vec<Witness>) -> Circuit {
        Circuit {
            current_witness_index: 0,
            expression_width: ExpressionWidth::Unbounded,
            opcodes: vec![only_expr],
            private_parameters: BTreeSet::new(),
            public_parameters: PublicInputs(BTreeSet::from_iter(public_input_witnesses)),
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
        let circuit = circuit_with_single_opcode(only_opcode, vec![public_input_witness]);

        // When
        let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

        // Then
        let mut witnesses = PartialWitness::<F>::new();
        let four = F::from_canonical_u64(4);
        let public_input_plonky2_target = witness_target_map.get(&public_input_witness).unwrap();
        witnesses.set_target(*public_input_plonky2_target, four);
        let proof = circuit_data.prove(witnesses).unwrap();
        assert_eq!(four, proof.public_inputs[0]);
        circuit_data.verify(proof).expect("Verification failed");
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
        let circuit = circuit_with_single_opcode(only_opcode, vec![public_input_witness]);

        // When
        let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

        // Then
        let mut witnesses = PartialWitness::<F>::new();
        let four = F::from_canonical_u64(4);
        let public_input_plonky2_target = witness_target_map.get(&public_input_witness).unwrap();
        witnesses.set_target(*public_input_plonky2_target, four);
        let proof = circuit_data.prove(witnesses).unwrap();
        assert_eq!(four, proof.public_inputs[0]);
        circuit_data.verify(proof).expect("Verification failed");
    }

    fn x_times_3_equals_12_opcode(public_input_witness: Witness) -> Opcode {
        AssertZero(Expression {
            mul_terms: Vec::new(),
            linear_combinations: vec![(FieldElement::from_hex("0x03").unwrap(), public_input_witness)],
            q_c: -FieldElement::from_hex("0x0C").unwrap()
        })
    }

    #[test]
    fn test_plonky2_vm_can_traslate_the_x_times_3_plus_y_times_4_equals_constant_program() {
        // Given
        let first_public_input_witness = Witness(0);
        let second_public_input_witness = Witness(1);
        let only_opcode = x_times_3_plus_y_times_4_equals_constant(first_public_input_witness, second_public_input_witness);
        let circuit = circuit_with_single_opcode(
            only_opcode, vec![first_public_input_witness, second_public_input_witness]);

        // When
        let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

        // Then
        let mut witnesses = PartialWitness::<F>::new();

        let one = F::from_canonical_u64(1);
        let public_input_plonky2_target = witness_target_map.get(&first_public_input_witness).unwrap();
        witnesses.set_target(*public_input_plonky2_target, one);

        let public_input_plonky2_target_2 = witness_target_map.get(&second_public_input_witness).unwrap();
        witnesses.set_target(*public_input_plonky2_target_2, one);

        let proof = circuit_data.prove(witnesses).unwrap();
        assert_eq!(one, proof.public_inputs[0]);
        assert_eq!(one, proof.public_inputs[1]);
        circuit_data.verify(proof).expect("Verification failed");
    }

    fn x_times_3_plus_y_times_4_equals_constant(first_public_input_witness: Witness, second_public_input_witness: Witness) -> Opcode {
        AssertZero(Expression {
            mul_terms: vec![],
            linear_combinations: vec![
                (FieldElement::from_hex("0x03").unwrap(), first_public_input_witness),
                (FieldElement::from_hex("0x09").unwrap(), second_public_input_witness),
            ],
            q_c: -FieldElement::from_hex("0x0c").unwrap()
        })
    }

    #[test]
    fn test_plonky2_vm_can_traslate_multiple_linear_combinations() {
        // Given
        let public_inputs = vec![Witness(0), Witness(1), Witness(2), Witness(3)];
        let only_opcode = multiple_linear_combinations_opcode(&public_inputs);
        let circuit = circuit_with_single_opcode(only_opcode, public_inputs.clone());

        // When
        let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

        // Then
        let mut witnesses = PartialWitness::<F>::new();

        let one = F::from_canonical_u64(1);
        for pi in &public_inputs {
            let public_input_plonky2_target = witness_target_map.get(pi).unwrap();
            witnesses.set_target(*public_input_plonky2_target, one);
        }

        let proof = circuit_data.prove(witnesses).unwrap();

        assert_eq!(one, proof.public_inputs[0]);
        assert_eq!(one, proof.public_inputs[1]);
        assert_eq!(one, proof.public_inputs[2]);
        assert_eq!(one, proof.public_inputs[3]);
        circuit_data.verify(proof).expect("Verification failed");
    }

    fn multiple_linear_combinations_opcode(public_inputs: &Vec<Witness>) -> Opcode {
        AssertZero(Expression {
            mul_terms: vec![],
            linear_combinations: public_inputs.iter().map(|a_witness| (FieldElement::from_hex("0x03").unwrap(), *a_witness)).rev().collect(),
            q_c: -FieldElement::from_hex("0x0c").unwrap()
        })
    }


    // #[test]
    // fn test_solo_plonky2() {
    //     let config = CircuitConfig::standard_recursion_config();
    //     let mut builder = CB::new(config);
    //
    //     let public_input_target = builder.add_virtual_target();
    //     builder.register_public_input(public_input_target);
    //
    //     let public_input_target_2 = builder.add_virtual_target();
    //     builder.register_public_input(public_input_target_2);
    //
    //     let result_target = builder.mul(public_input_target_2, public_input_target);
    //     builder.assert_zero(result_target);
    //
    //     let mut witnesses = PartialWitness::<F>::new();
    //     let one = GoldilocksField::from_canonical_u64(1);
    //     let zero = GoldilocksField::from_canonical_u64(0);
    //
    //     let circuit_data: CircuitData<F, C, 2> = builder.build();
    //
    //     witnesses.set_target(public_input_target, one);
    //     witnesses.set_target(public_input_target_2, zero);
    //
    //     let proof = circuit_data.prove(witnesses).unwrap();
    //     circuit_data.verify(proof).expect("as");
    // }

}
