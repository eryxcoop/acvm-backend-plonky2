use acir::circuit::opcodes;
use acir::circuit::opcodes::MemOp as GenericMemOp;
use acir::circuit::opcodes::{BlockId, FunctionInput};
use acir::circuit::Circuit as GenericCircuit;
use acir::circuit::Opcode as GenericOpcode;
use acir::circuit::Program as GenericProgram;
use acir::native_types::Expression as GenericExpression;
pub use acir::native_types::Witness;
use acir::native_types::WitnessStack as GenericWitnessStack;
use num_bigint::BigUint;
use std::collections::HashMap;

// Generics
pub use acir_field::AcirField;
pub use acir_field::FieldElement;
use plonky2::field::types::Field;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig};

mod binary_digits_target;
mod sha256_translator;

use binary_digits_target::BinaryDigitsTarget;
use sha256_translator::Sha256CompressionTranslator;

#[cfg(test)]
mod tests;

pub mod assert_zero_translator;

const D: usize = 2;

type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type CB = CircuitBuilder<F, D>;

pub type Opcode = GenericOpcode<FieldElement>;
pub type Circuit = GenericCircuit<FieldElement>;
pub type Program = GenericProgram<FieldElement>;
pub type Expression = GenericExpression<FieldElement>;
pub type MemOp = GenericMemOp<FieldElement>;
pub type WitnessStack = GenericWitnessStack<FieldElement>;

pub struct CircuitBuilderFromAcirToPlonky2 {
    pub builder: CB,
    pub witness_target_map: HashMap<Witness, Target>,
    pub memory_blocks: HashMap<BlockId, Vec<Target>>,
}

impl CircuitBuilderFromAcirToPlonky2 {
    pub fn new() -> Self {
        let config = CircuitConfig::standard_recursion_config();
        let builder = CB::new(config);
        let witness_target_map: HashMap<Witness, Target> = HashMap::new();
        let memory_blocks: HashMap<BlockId, Vec<Target>> = HashMap::new();
        Self {
            builder,
            witness_target_map,
            memory_blocks,
        }
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
                        &mut self.builder,
                        &mut self.witness_target_map,
                        &expr,
                    );
                    translator.translate();
                }
                Opcode::BrilligCall {
                    id: _,
                    inputs: _,
                    outputs: _,
                    predicate: _,
                } => {}
                Opcode::Directive(_directive) => {}
                Opcode::MemoryInit {
                    block_id,
                    init,
                    block_type: _,
                } => {
                    let vector_targets = init
                        .into_iter()
                        .map(|w| self._get_or_create_target_for_witness(*w))
                        .collect();
                    self.memory_blocks.insert(*block_id, vector_targets);
                }
                Opcode::MemoryOp {
                    block_id,
                    op,
                    predicate: _,
                } => {
                    // TODO: check whether we should register if the predicate is false
                    self._register_intermediate_witnesses_for_memory_op(&op);
                    let is_memory_read = op.clone().operation.to_const().unwrap().is_zero();
                    let is_memory_write = op.clone().operation.to_const().unwrap().is_one();
                    if is_memory_read {
                        let witness_idx_to_read = op.index.to_witness().unwrap();
                        let target_idx_to_read =
                            self._get_or_create_target_for_witness(witness_idx_to_read);
                        let witness_to_save_result = op.value.to_witness().unwrap();
                        let target_to_save_result = self.builder.random_access(
                            target_idx_to_read,
                            self.memory_blocks[block_id].clone(),
                        );
                        self.witness_target_map
                            .insert(witness_to_save_result, target_to_save_result);
                    } else if is_memory_write {
                        let witness_idx_to_write = op.index.to_witness().unwrap();
                        let target_idx_to_write =
                            self._get_or_create_target_for_witness(witness_idx_to_write);
                        let witness_holding_new_value = op.value.to_witness().unwrap();
                        let target_holding_new_value =
                            self._get_or_create_target_for_witness(witness_holding_new_value);

                        let memory_block_length = (&self.memory_blocks[block_id]).len();
                        for position in 0..memory_block_length {
                            let target_with_position =
                                self.builder.constant(F::from_canonical_usize(position));
                            let is_current_position_being_modified = self
                                .builder
                                .is_equal(target_idx_to_write, target_with_position)
                                .target;
                            let constant_one = self.builder.constant(F::from_canonical_usize(1));
                            let is_current_position_being_kept = self
                                .builder
                                .sub(constant_one, is_current_position_being_modified);

                            let new_target_in_array = self.builder.add_virtual_target();
                            let current_target_in_position =
                                self.memory_blocks[block_id][position];

                            // Case where the current position is being modified
                            self.builder.conditional_assert_eq(
                                is_current_position_being_modified,
                                target_holding_new_value,
                                new_target_in_array,
                            );
                            // Case where we want to keep the current value in the array
                            self.builder.conditional_assert_eq(
                                is_current_position_being_kept,
                                current_target_in_position,
                                new_target_in_array,
                            );
                            self.memory_blocks.get_mut(block_id).unwrap()[position] = new_target_in_array;
                        }
                    } else {
                        panic!("Backend encountered unknown memory operation code (nor 0 or 1)");
                    }
                }
                Opcode::BlackBoxFuncCall(func_call) => {
                    match func_call {
                        opcodes::BlackBoxFuncCall::RANGE { input } => {
                            let long_max_bits = input.num_bits.clone() as usize;
                            assert!(long_max_bits <= 33,
                                    "Range checks with more than 33 bits are not allowed yet while using Plonky2 prover");
                            let witness = input.witness;
                            let target = self._get_or_create_target_for_witness(witness);
                            self.builder.range_check(target, long_max_bits)
                        }
                        opcodes::BlackBoxFuncCall::AND { lhs, rhs, output } => {
                            self._extend_circuit_with_operation(
                                lhs,
                                rhs,
                                output,
                                BinaryDigitsTarget::and,
                            );
                        }
                        opcodes::BlackBoxFuncCall::XOR { lhs, rhs, output } => {
                            self._extend_circuit_with_operation(
                                lhs,
                                rhs,
                                output,
                                BinaryDigitsTarget::xor,
                            );
                        }
                        opcodes::BlackBoxFuncCall::Sha256Compression {
                            inputs,
                            hash_values,
                            outputs,
                        } => {
                            self._extend_circuit_with_sha256_compression_operation(
                                inputs,
                                hash_values,
                                outputs,
                            );
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

    fn _extend_circuit_with_sha256_compression_operation(
        &mut self,
        inputs: &Box<[FunctionInput; 16]>,
        hash_values: &Box<[FunctionInput; 8]>,
        outputs: &Box<[Witness; 8]>,
    ) {
        let mut sha256_compression_translator =
            Sha256CompressionTranslator::new_for(self, inputs, hash_values, outputs);
        sha256_compression_translator.translate();
    }

    fn _extend_circuit_with_operation(
        self: &mut Self,
        lhs: &FunctionInput,
        rhs: &FunctionInput,
        output: &Witness,
        operation: fn(BinaryDigitsTarget, BinaryDigitsTarget, &mut CB) -> BinaryDigitsTarget,
    ) {
        assert_eq!(lhs.num_bits, rhs.num_bits);
        let binary_digits = lhs.num_bits as usize;
        let lhs_binary_target = self.binary_number_target_for_witness(lhs.witness, binary_digits);
        let rhs_binary_target = self.binary_number_target_for_witness(rhs.witness, binary_digits);

        let output_binary_target =
            operation(lhs_binary_target, rhs_binary_target, &mut self.builder);

        let output_target = self.convert_binary_number_to_number(output_binary_target);
        self.witness_target_map.insert(*output, output_target);
    }

    pub fn binary_number_target_for_witness(
        &mut self,
        w: Witness,
        digits: usize,
    ) -> BinaryDigitsTarget {
        let target = self._get_or_create_target_for_witness(w);
        self.convert_number_to_binary_number(target, digits)
    }

    pub fn binary_number_target_for_constant(
        &mut self,
        constant: usize,
        digits: usize,
    ) -> BinaryDigitsTarget {
        let bit_targets = (0..digits)
            .map(|bit_position| self._constant_bool_target_for_bit(constant, bit_position))
            .collect();
        BinaryDigitsTarget { bits: bit_targets }
    }

    fn convert_number_to_binary_number(
        &mut self,
        number_target: Target,
        digits: usize,
    ) -> BinaryDigitsTarget {
        BinaryDigitsTarget {
            bits: self
                .builder
                .split_le(number_target, digits)
                .into_iter()
                .rev()
                .collect(),
        }
    }

    fn convert_binary_number_to_number(&mut self, a: BinaryDigitsTarget) -> Target {
        self.builder.le_sum(a.bits.into_iter().rev())
    }

    fn zeroes(&mut self, digits: usize) -> Vec<BoolTarget> {
        vec![self._bool_target_false(); digits]
    }

    fn _constant_bool_target_for_bit(
        &mut self,
        constant_value: usize,
        bit_position: usize,
    ) -> BoolTarget {
        let cond = (constant_value & (1 << bit_position)) == 1;
        self.builder.constant_bool(cond)
    }

    fn _bool_target_false(&mut self) -> BoolTarget {
        self.builder._false()
    }

    fn _register_public_parameters_from_acir_circuit(self: &mut Self, circuit: &Circuit) {
        let public_parameters_as_list: Vec<Witness> =
            circuit.public_parameters.0.iter().cloned().collect();
        for public_parameter_witness in public_parameters_as_list {
            self._register_new_public_input_from_witness(public_parameter_witness);
        }
    }

    fn _register_new_public_input_from_witness(
        self: &mut Self,
        public_input_witness: Witness,
    ) -> Target {
        let public_input_target = self.builder.add_virtual_target();
        self.builder.register_public_input(public_input_target);
        self.witness_target_map
            .insert(public_input_witness, public_input_target);
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
}
