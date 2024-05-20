mod tests;
pub mod assert_zero_translator;

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
use plonky2::iop::witness::WitnessWrite;
use plonky2::plonk::proof::ProofWithPublicInputs;
use std::collections::BTreeSet;
use acir::circuit::Opcode;

const D: usize = 2;

type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type CB = CircuitBuilder::<F, D>;

pub struct CircuitBuilderFromAcirToPlonky2 {
    pub builder: CB,
    pub witness_target_map: HashMap<Witness, Target>,
}

impl CircuitBuilderFromAcirToPlonky2 {
    pub fn new() -> Self {
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CB::new(config);
        let mut witness_target_map: HashMap<Witness, Target> = HashMap::new();
        Self { builder, witness_target_map }
    }

    pub fn translate_circuit(self: &mut Self, circuit: &Circuit) {
        self._register_public_parameters_from_acir_circuit(circuit);
        for opcode in &circuit.opcodes {
            match opcode {
                AssertZero(expr) => {
                    self._register_intermediate_witnesses_for_assert_zero(&expr);
                    self._translate_assert_zero(&expr);
                },
                _ => { () }
            }
        }
    }

    fn _register_public_parameters_from_acir_circuit(self: &mut Self, circuit: &Circuit) {
        let public_parameters_as_list: Vec<Witness> = circuit.public_parameters.0.iter().cloned().collect();
        for public_parameter_witness in public_parameters_as_list {
            self._register_new_public_input_from_witness(public_parameter_witness);
        }
    }

    fn _register_new_public_input_from_witness(self: &mut Self, public_input_witness: Witness) {
        let public_input_target = self.builder.add_virtual_target();
        self.builder.register_public_input(public_input_target);
        self.witness_target_map.insert(public_input_witness, public_input_target);
    }

    fn _field_element_to_goldilocks_field(self: &mut Self, fe: &FieldElement) -> F {
        let fe_as_big_uint = BigUint::from_bytes_be(&fe.to_be_bytes() as &[u8]);
        F::from_noncanonical_biguint(fe_as_big_uint)
    }

    fn _register_intermediate_witnesses_for_assert_zero(self: &mut Self, expr: &Expression) {
        for (_, witness_1, witness_2) in &expr.mul_terms {
            self._register_witness_if_not_already_registered(*witness_1);
            self._register_witness_if_not_already_registered(*witness_2);
        }
        for (_, witness) in &expr.linear_combinations {
            self._register_witness_if_not_already_registered(*witness);
        }
    }

    fn _register_witness_if_not_already_registered(self: &mut Self, witness: Witness) {
        self.witness_target_map.entry(witness).or_insert(self.builder.add_virtual_target());
    }

    fn _translate_assert_zero(self: &mut Self, expression: &Expression) {
        println!("{:?}", expression);
        let g_constant = self._field_element_to_goldilocks_field(&expression.q_c);

        let constant_target = self.builder.constant(g_constant);
        let mut current_acc_target = constant_target;
        current_acc_target = self._add_linear_combinations(expression, current_acc_target);
        current_acc_target = self._add_cuadratic_combinations(expression, current_acc_target);
        self.builder.assert_zero(current_acc_target);
    }

    fn _add_cuadratic_combinations(self: &mut Self, expression: &Expression, mut current_acc_target: Target) -> Target {
        let mul_terms = &expression.mul_terms;
        for mul_term in mul_terms {
            let (f_cuadratic_factor, public_input_witness_1, public_input_witness_2) = mul_term;
            let cuadratic_target = self._compute_cuadratic_term_target(f_cuadratic_factor, public_input_witness_1, public_input_witness_2);
            let new_target = self.builder.add(cuadratic_target, current_acc_target);
            current_acc_target = new_target;
        }
        current_acc_target
    }

    fn _add_linear_combinations(self: &mut Self, expression: &Expression, mut current_acc_target: Target) -> Target {
        let linear_combinations = &expression.linear_combinations;
        for (f_multiply_factor, public_input_witness) in linear_combinations {
            let linear_combination_target = self._compute_linear_combination_target(f_multiply_factor, public_input_witness);
            let new_target = self.builder.add(linear_combination_target, current_acc_target);
            current_acc_target = new_target;
        }
        current_acc_target
    }

    fn _compute_linear_combination_target(self: &mut Self,
                                          f_multiply_constant_factor: &FieldElement,
                                          public_input_witness: &Witness) -> Target {
        let factor_target = *self.witness_target_map.get(public_input_witness).unwrap();
        let g_first_pi_factor = self._field_element_to_goldilocks_field(f_multiply_constant_factor);
        self.builder.mul_const(g_first_pi_factor, factor_target)
    }

    fn _compute_cuadratic_term_target(self: &mut Self,
                                      f_cuadratic_factor: &FieldElement,
                                      public_input_witness_1: &Witness,
                                      public_input_witness_2: &Witness) -> Target {
        let g_cuadratic_factor = self._field_element_to_goldilocks_field(f_cuadratic_factor);
        let first_public_input_target = *self.witness_target_map.get(public_input_witness_1).unwrap();
        let second_public_input_target = *self.witness_target_map.get(public_input_witness_2).unwrap();

        let cuadratic_target = self.builder.mul(first_public_input_target, second_public_input_target);
        self.builder.mul_const(g_cuadratic_factor, cuadratic_target)
    }
}

