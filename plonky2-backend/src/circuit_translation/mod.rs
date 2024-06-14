mod tests;
pub mod assert_zero_translator;
mod binary_digits_target;
mod sha256_translator;


use std::collections::{HashMap};
use std::error::Error;
use acir::circuit::{Circuit};
use acir::circuit::{ExpressionWidth, PublicInputs};
use acir::FieldElement;
use acir::native_types::{Expression, Witness};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::{Field, Field64};
use plonky2::iop::target::{BoolTarget, Target};
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
use acir::circuit::opcodes;
use acir::circuit::opcodes::{FunctionInput, MemOp};
use crate::circuit_translation::binary_digits_target::BinaryDigitsTarget;
use crate::circuit_translation::sha256_translator::Sha256Translator;


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

    pub fn unpack(self) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>) {
        (self.builder.build::<C>(), self.witness_target_map)
    }

    pub fn translate_circuit(self: &mut Self, circuit: &Circuit) {
        self._register_public_parameters_from_acir_circuit(circuit);
        for opcode in &circuit.opcodes {
            match opcode {
                Opcode::AssertZero(expr) => {
                    let mut translator = assert_zero_translator::AssertZeroTranslator::new_for(
                        &mut self.builder, &mut self.witness_target_map, &expr);
                    translator.translate();
                }
                Opcode::BrilligCall { id, inputs, outputs, predicate } => {
                    eprintln!("----------Brillig--------");
                    eprintln!("id: {:?}", id);
                    eprintln!("inputs: {:?}", inputs);
                    eprintln!("outputs: {:?}", outputs);
                    eprintln!("predicate: {:?}", predicate);
                }
                Opcode::MemoryInit { block_id, init } => {
                    eprintln!("outputs: {:?}", block_id);
                    eprintln!("predicate: {:?}", init);
                }
                Opcode::MemoryOp { block_id, op, predicate } => {
                    // TODO: check whether we should register if the predicate is false
                    self._register_intermediate_witnesses_for_memory_op(&op);
                }
                Opcode::BlackBoxFuncCall(func_call) => {
                    eprintln!("{:?}", func_call);
                    match func_call {
                        opcodes::BlackBoxFuncCall::RANGE { input } => {
                            eprintln!("{:?}", input);
                            let long_max_bits = input.num_bits.clone() as usize;
                            assert!(long_max_bits <= 32,
                                    "Range checks with more than 32 bits are not allowed yet while using Plonky2 prover");
                            let witness = input.witness;
                            let target = self._get_or_create_target_for_witness(witness);
                            self.builder.range_check(target, long_max_bits)
                        }
                        opcodes::BlackBoxFuncCall::AND { lhs, rhs, output } => {
                            self._extend_circuit_with_operation(lhs, rhs, output, Self::and);
                        }
                        opcodes::BlackBoxFuncCall::XOR { lhs, rhs, output } => {
                            self._extend_circuit_with_operation(lhs, rhs, output, Self::xor);
                        }
                        opcodes::BlackBoxFuncCall::SHA256 { inputs, outputs } => {
                            self._extend_circuit_with_sha256_operation(inputs, outputs);
                        }
                        blackbox_func => {
                            panic!("Blackbox func not supported yet: {:?}", blackbox_func);
                        }
                    };
                }

                opcode => {
                    panic!("Opcode not supported yet: {:?}", opcode);
                }
            }
        }
    }

    fn _extend_circuit_with_sha256_operation(&mut self, inputs: &Vec<FunctionInput>, outputs: &Box<[Witness; 32]>) {
        let mut translator = Sha256Translator::new_for(self, inputs, outputs);
        translator.translate();
    }

    fn _extend_circuit_with_operation(self: &mut Self, lhs: &FunctionInput, rhs: &FunctionInput,
                                      output: &Witness, operation: fn(&mut Self, BinaryDigitsTarget, BinaryDigitsTarget) -> BinaryDigitsTarget) {
        assert_eq!(lhs.num_bits, rhs.num_bits);
        let binary_digits = lhs.num_bits as usize;
        let lhs_binary_target = self.binary_number_target_for_witness(lhs.witness, binary_digits);
        let rhs_binary_target = self.binary_number_target_for_witness(rhs.witness, binary_digits);

        let output_binary_target = operation(self, lhs_binary_target, rhs_binary_target);

        let output_target = self.convert_binary_number_to_number(output_binary_target);
        self.witness_target_map.insert(*output, output_target);
    }

    pub fn binary_number_target_for_witness(&mut self, w: Witness, digits: usize) -> BinaryDigitsTarget {
        let target = self._get_or_create_target_for_witness(w);
        self.convert_number_to_binary_number(target, digits)
    }

    pub fn binary_number_target_for_constant(&mut self, c: usize, digits: usize) -> BinaryDigitsTarget {
        let bits = (0..digits).map(|i| self._constant_bool_target_for_bit(c, i)).collect();
        BinaryDigitsTarget { bits }
    }

    fn convert_number_to_binary_number(&mut self, a: Target, digits: usize) -> BinaryDigitsTarget {
        BinaryDigitsTarget {
            bits: self.builder.split_le(a, digits).into_iter().rev().collect(),
        }
    }

    fn convert_binary_number_to_number(&mut self, a: BinaryDigitsTarget) -> Target {
        self.builder.le_sum(a.bits.into_iter().rev())
    }

    fn zeroes(&mut self, digits: usize) -> Vec<BoolTarget> {
        vec![self._bool_target_false(); digits]
    }

    fn _constant_bool_target_for_bit(&mut self, n: usize, i: usize) -> BoolTarget {
        let cond = (n & (1 << i)) == 0;
        self.builder.constant_bool(cond)
    }

    fn _bool_target_false(&mut self) -> BoolTarget {
        self.builder._false()
    }

    fn _register_public_parameters_from_acir_circuit(self: &mut Self, circuit: &Circuit) {
        let public_parameters_as_list: Vec<Witness> = circuit.public_parameters.0.iter().cloned().collect();
        for public_parameter_witness in public_parameters_as_list {
            self._register_new_public_input_from_witness(public_parameter_witness);
        }
    }

    fn _register_new_public_input_from_witness(self: &mut Self, public_input_witness: Witness) -> Target {
        let public_input_target = self.builder.add_virtual_target();
        self.builder.register_public_input(public_input_target);
        self.witness_target_map.insert(public_input_witness, public_input_target);
        public_input_target
    }

    fn _register_intermediate_witnesses_for_memory_op(self: &mut Self, op: &MemOp) {
        let at = &op.index.linear_combinations[0].1;
        self._get_or_create_target_for_witness(*at);

        let value = &op.value.linear_combinations[0].1;
        self._get_or_create_target_for_witness(*value);
    }

    fn _get_or_create_target_for_witness(self: &mut Self, witness: Witness) -> Target {
        match self.witness_target_map.get(&witness) {
            Some(target) => *target,
            None => {
                let target = self.builder.add_virtual_target();
                self.witness_target_map.insert(witness, target);
                target
            }
        }
    }

    fn xor(&mut self, b1: BinaryDigitsTarget, b2: BinaryDigitsTarget) -> BinaryDigitsTarget {
        self.apply_bitwise_to_binary_digits_target(b1, b2, Self::bit_xor)
    }

    fn or(&mut self, b1: BinaryDigitsTarget, b2: BinaryDigitsTarget) -> BinaryDigitsTarget {
        self.apply_bitwise_to_binary_digits_target(b1, b2, Self::bit_or)
    }

    fn and(&mut self, b1: BinaryDigitsTarget, b2: BinaryDigitsTarget) -> BinaryDigitsTarget {
        self.apply_bitwise_to_binary_digits_target(b1, b2, Self::bit_and)
    }

    fn add(&mut self, b1: &BinaryDigitsTarget, b2: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let partial_sum = self.apply_bitwise_and_output_bool_targets(&b1, &b2, Self::bit_xor);
        let partial_carries = self.apply_bitwise_and_output_bool_targets(&b1, &b2, Self::bit_and);

        let mut carry_in = self._bool_target_false();

        let sum = (0..b1.number_of_digits()).map(|idx_bit| {
            let sum_with_carry_in = self.bit_xor(partial_sum[idx_bit], carry_in);
            let carry_out = self.bit_or(partial_carries[idx_bit], carry_in);

            carry_in = carry_out; // The new carry_in is the current carry_out
            sum_with_carry_in
        }).collect();

        BinaryDigitsTarget { bits: sum }
    }

    fn apply_bitwise_to_binary_digits_target(&mut self, b1: BinaryDigitsTarget, b2: BinaryDigitsTarget,
                                             op: fn(&mut CircuitBuilderFromAcirToPlonky2, BoolTarget, BoolTarget) -> BoolTarget) -> BinaryDigitsTarget {
        BinaryDigitsTarget { bits: self.apply_bitwise_and_output_bool_targets(&b1, &b2, op) }
    }

    fn apply_bitwise_and_output_bool_targets(
        &mut self, b1: &BinaryDigitsTarget, b2: &BinaryDigitsTarget,
        op: fn(&mut CircuitBuilderFromAcirToPlonky2, BoolTarget, BoolTarget) -> BoolTarget
    ) -> Vec<BoolTarget> {
        b1.bits
            .iter()
            .zip(b2.bits.iter())
            .map(|(bit1, bit2)| op(self, *bit1, *bit2))
            .collect()
    }

    fn bit_and(&mut self, b1: BoolTarget, b2: BoolTarget) -> BoolTarget {
        self.builder.and(b1, b2)
    }

    fn bit_or(&mut self, b1: BoolTarget, b2: BoolTarget) -> BoolTarget { self.builder.or(b1, b2) }

    fn bit_xor(&mut self, b1: BoolTarget, b2: BoolTarget) -> BoolTarget {
        // a xor b = (a or b) and (not (a and b))
        let b1_or_b2 = self.builder.or(b1, b2);
        let b1_and_b2 = self.builder.and(b1, b2);
        let not_b1_and_b2 = self.builder.not(b1_and_b2);
        self.builder.and(b1_or_b2, not_b1_and_b2)
    }

    pub fn shift_right(&mut self, target: &BinaryDigitsTarget, times: usize) -> BinaryDigitsTarget {
        let mut new_bits = Vec::new();
        // Fill zero bits
        for _ in 0..times {
            new_bits.push(BoolTarget::new_unsafe(
                self.builder.constant(F::from_canonical_u8(0)),
            ));
        }

        for i in times..8 {
            let new_bool_target = self.builder.add_virtual_bool_target_safe();
            self.builder.connect(target.bits[i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }
        BinaryDigitsTarget { bits: new_bits }
    }

    pub fn rotate_right(&mut self, target: &BinaryDigitsTarget, times: usize) -> BinaryDigitsTarget {
        let mut new_bits = Vec::new();
        // Wrap bits around
        for i in 0..times {
            let new_bool_target = self.builder.add_virtual_bool_target_safe();
            self.builder.connect(target.bits[target.number_of_digits() + i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }

        for i in times..8 {
            let new_bool_target = self.builder.add_virtual_bool_target_safe();
            self.builder.connect(target.bits[i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }
        BinaryDigitsTarget { bits: new_bits }
    }
}
