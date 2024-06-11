mod tests;
pub mod assert_zero_translator;
mod targets;

use std::cmp::max;
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
use crate::circuit_translation::targets::BinaryDigitsTarget;


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

    pub fn unpack(self) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>){
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
                            self._extend_circuit_with_bitwise_operation(lhs, rhs, output, Self::and);
                        }
                        opcodes::BlackBoxFuncCall::XOR { lhs, rhs, output } => {
                            self._extend_circuit_with_bitwise_operation(lhs, rhs, output, Self::xor);
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

   fn _extend_circuit_with_sha256_operation(&self, inputs: &Vec<FunctionInput>, outputs: &Box<[Witness; 32]>) {
//         h = ['0x6a09e667', '0xbb67ae85', '0x3c6ef372', '0xa54ff53a', '0x510e527f', '0x9b05688c', '0x1f83d9ab', '0x5be0cd19']
//         k = ['0x428a2f98', '0x71374491', '0xb5c0fbcf', '0xe9b5dba5', '0x3956c25b', '0x59f111f1', '0x923f82a4','0xab1c5ed5', '0xd807aa98', '0x12835b01', '0x243185be', '0x550c7dc3', '0x72be5d74', '0x80deb1fe','0x9bdc06a7', '0xc19bf174', '0xe49b69c1', '0xefbe4786', '0x0fc19dc6', '0x240ca1cc', '0x2de92c6f','0x4a7484aa', '0x5cb0a9dc', '0x76f988da', '0x983e5152', '0xa831c66d', '0xb00327c8', '0xbf597fc7','0xc6e00bf3', '0xd5a79147', '0x06ca6351', '0x14292967', '0x27b70a85', '0x2e1b2138', '0x4d2c6dfc','0x53380d13', '0x650a7354', '0x766a0abb', '0x81c2c92e', '0x92722c85', '0xa2bfe8a1', '0xa81a664b','0xc24b8b70', '0xc76c51a3', '0xd192e819', '0xd6990624', '0xf40e3585', '0x106aa070', '0x19a4c116','0x1e376c08', '0x2748774c', '0x34b0bcb5', '0x391c0cb3', '0x4ed8aa4a', '0x5b9cca4f', '0x682e6ff3','0x748f82ee', '0x78a5636f', '0x84c87814', '0x8cc70208', '0x90befffa', '0xa4506ceb', '0xbef9a3f7','0xc67178f2']

    }

    fn _extend_circuit_with_bitwise_operation(self: &mut Self, lhs: &FunctionInput, rhs: &FunctionInput,
                                              output: &Witness, operation: fn(&mut Self, BoolTarget, BoolTarget) -> BoolTarget) {
        assert_eq!(lhs.num_bits, rhs.num_bits);
        let binary_digits = lhs.num_bits as usize;
        let lhs_binary_target = self._binary_number_target_for_witness(lhs.witness, binary_digits);
        let rhs_binary_target = self._binary_number_target_for_witness(rhs.witness, binary_digits);

        let output_binary_target = self._translate_bitwise_operation(
            lhs_binary_target, rhs_binary_target, operation);

        let output_target = self.convert_binary_number_to_number(output_binary_target);
        self.witness_target_map.insert(*output, output_target);
    }

    fn _binary_number_target_for_witness(self: &mut Self, w: Witness, digits: usize) -> BinaryDigitsTarget {
        let target = self._get_or_create_target_for_witness(w);
        self.convert_number_to_binary_number(target, digits)
    }

    fn convert_number_to_binary_number(&mut self, a: Target, digits: usize) -> BinaryDigitsTarget {
        BinaryDigitsTarget {
            bits: self.builder.split_le(a, digits).into_iter().rev().collect(),
        }
    }

    fn convert_binary_number_to_number(&mut self, a: BinaryDigitsTarget) -> Target {
        self.builder.le_sum(a.bits.into_iter().rev())
    }

    fn _translate_bitwise_operation(self: &mut Self, lhs: BinaryDigitsTarget, rhs: BinaryDigitsTarget,
                                    operation: fn(&mut Self, BoolTarget, BoolTarget) -> BoolTarget) -> BinaryDigitsTarget {
        BinaryDigitsTarget {
            bits: lhs
                .bits.iter().zip(rhs.bits.iter())
                .map(|(x, y)| operation(self, *x, *y)).collect(),
        }
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

    fn and(&mut self, b1: BoolTarget, b2: BoolTarget) -> BoolTarget {
        self.builder.and(b1, b2)
    }

    fn xor(&mut self, b1: BoolTarget, b2: BoolTarget) -> BoolTarget {
        // a xor b = (a or b) and (not (a and b))
        let b1_or_b2 = self.builder.or(b1, b2);
        let b1_and_b2 = self.builder.and(b1, b2);
        let not_b1_and_b2 = self.builder.not(b1_and_b2);
        self.builder.and(b1_or_b2, not_b1_and_b2)
    }
}

