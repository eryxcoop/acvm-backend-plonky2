use super::*;
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
use std::sync::Arc;
// Generics
pub use acir_field::AcirField;
pub use acir_field::FieldElement;
use plonky2::field::types::Field;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::circuit_data::CircuitData;

mod binary_digits_target;
mod memory_translator;
mod sha256_translator;

use binary_digits_target::BinaryDigitsTarget;
use memory_translator::MemoryOperationsTranslator;
use plonky2::gates::lookup_table::LookupTable;
use sha256_translator::Sha256CompressionTranslator;

#[cfg(test)]
mod tests;

pub mod assert_zero_translator;

type CB = CircuitBuilder<F, D>;

/// The FieldElement is imported from the Noir library, but for this backend to work the
/// GoldilocksField should be used (and the witnesses generated accordingly).

pub type Opcode = GenericOpcode<FieldElement>;
pub type Circuit = GenericCircuit<FieldElement>;
pub type Program = GenericProgram<FieldElement>;
pub type Expression = GenericExpression<FieldElement>;
pub type MemOp = GenericMemOp<FieldElement>;
pub type WitnessStack = GenericWitnessStack<FieldElement>;

/// This is the most important part of the backend. The CircuitBuilderFromAcirToPlonky2 translates
/// the ACIR Circuit into an equivalent Plonky2 circuit. Besides the Plonky2 circuit, the output
/// contains a mapping from ACIR Witnesses to Plonky2 Targets, which is not only for internal use
/// but for assigning values to the targets when generating the proof.
///
/// The opcodes suported are: AssertZero, MemoryInit, MemoryOp, BrilligCall, Directive(ToLeRadix),
/// and the BlackboxFunctions: Range, And, Xor, SHA256Compression.
///
/// Internally it uses a Plonky2 CircuitBuilder for generating the circuit, a mapping of memory
/// blocks for the memory operations and the witness to targets mapping to retain the information
/// about which target is which.

pub struct CircuitBuilderFromAcirToPlonky2 {
    pub builder: CB,
    pub witness_target_map: HashMap<Witness, Target>,
    pub memory_blocks: HashMap<BlockId, (Vec<Target>, usize)>,
    pub u8_range_table_index: Option<usize>,
    pub u8_xor_table_index: Option<usize>,
}

impl CircuitBuilderFromAcirToPlonky2 {
    pub fn new() -> Self {
        let config = CircuitConfig::standard_recursion_config();
        let builder = CB::new(config);
        let witness_target_map: HashMap<Witness, Target> = HashMap::new();
        let memory_blocks: HashMap<BlockId, (Vec<Target>, usize)> = HashMap::new();
        Self {
            builder,
            witness_target_map,
            memory_blocks,
            u8_range_table_index: None,
            u8_xor_table_index: None,
        }
    }

    pub fn unpack(self) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>) {
        (self.builder.build::<C>(), self.witness_target_map)
    }

    /// Main function of the module. It sequentially parses the ACIR opcodes, applying changes
    /// in the CircuitBuilder accordingly.
    pub fn translate_circuit(self: &mut Self, circuit: &Circuit) {
        self._register_witnesses_from_acir_circuit(circuit);
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
                } => {} // The brillig call is ignored since it has no impact in the circuit
                Opcode::Directive(_directive) => {} // The same happens with the Directive
                Opcode::MemoryInit {
                    block_id,
                    init,
                    block_type: _,
                } => {
                    MemoryOperationsTranslator::new_for(
                        &mut self.builder,
                        &mut self.witness_target_map,
                        &mut self.memory_blocks,
                    )
                    .translate_memory_init(init, block_id);
                }
                Opcode::MemoryOp {
                    block_id,
                    op,
                    predicate: _,
                } => {
                    MemoryOperationsTranslator::new_for(
                        &mut self.builder,
                        &mut self.witness_target_map,
                        &mut self.memory_blocks,
                    )
                    .translate_memory_op(block_id, op);
                }
                Opcode::BlackBoxFuncCall(func_call) => {
                    match func_call {
                        opcodes::BlackBoxFuncCall::RANGE { input } => {
                            let long_max_bits = input.num_bits.clone() as usize;
                            let witness = input.witness;
                            let target = self._get_or_create_target_for_witness(witness);

                            if long_max_bits == 8 {
                                match self.u8_range_table_index {
                                    Some(_index) => {}
                                    None => {
                                        let table: LookupTable =
                                            Arc::new((0..256u16).zip((0..256u16)).collect());
                                        let u8_range_table_index =
                                            self.builder.add_lookup_table_from_pairs(table);
                                        self.u8_range_table_index = Some(u8_range_table_index);
                                    }
                                }
                                self.builder.add_lookup_from_index(
                                    target,
                                    self.u8_range_table_index.unwrap(),
                                );
                            } else {
                                assert!(long_max_bits <= 33,
                                        "Range checks with more than 33 bits are not allowed yet while using Plonky2 prover");
                                self.builder.range_check(target, long_max_bits)
                            }
                        }
                        opcodes::BlackBoxFuncCall::AND { lhs, rhs, output } => {
                            self._extend_circuit_with_bitwise_operation(
                                lhs,
                                rhs,
                                output,
                                BinaryDigitsTarget::and,
                            );
                        }
                        opcodes::BlackBoxFuncCall::XOR { lhs, rhs, output } => {
                            if lhs.num_bits == 8 {
                                let target_left =
                                    self._get_or_create_target_for_witness(lhs.witness);
                                let target_right =
                                    self._get_or_create_target_for_witness(rhs.witness);
                                let target_256 = self.builder.constant(F::from_canonical_u32(256));
                                let target_index_lookup =
                                    self.builder.mul_add(target_left, target_256, target_right);
                                match self.u8_xor_table_index {
                                    Some(_index) => {}
                                    None => {
                                        let supported_indexes: Vec<u16> = (0..65535).collect();
                                        let supported_indexes: &[u16] = &supported_indexes;
                                        let u8_xor_table_index =
                                            self.builder.add_lookup_table_from_fn(
                                                Self::_xor_to_compressed_value,
                                                supported_indexes,
                                            );
                                        self.u8_xor_table_index = Some(u8_xor_table_index);
                                    }
                                }
                                let output_target = self.builder.add_lookup_from_index(
                                    target_index_lookup,
                                    self.u8_xor_table_index.unwrap(),
                                );
                                self.witness_target_map.insert(*output, output_target);
                            } else {
                                self._extend_circuit_with_bitwise_operation(
                                    lhs,
                                    rhs,
                                    output,
                                    BinaryDigitsTarget::xor,
                                );
                            }
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

    fn _xor_to_compressed_value(compressed_value: u16) -> u16 {
        let a = compressed_value / 256;
        let b = compressed_value % 256;
        a ^ b
    }

    fn _extend_circuit_with_bitwise_operation(
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
            .rev()
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

    fn _constant_bool_target_for_bit(
        &mut self,
        constant_value: usize,
        bit_position: usize,
    ) -> BoolTarget {
        let cond = (constant_value & (1 << bit_position)) != 0;
        self.builder.constant_bool(cond)
    }

    fn _register_witnesses_from_acir_circuit(self: &mut Self, circuit: &Circuit) {
        // Public parameters
        let public_parameters_as_list: Vec<Witness> =
            circuit.public_parameters.0.iter().cloned().collect();
        for public_parameter_witness in public_parameters_as_list {
            self._register_new_public_input_from_witness(public_parameter_witness);
        }
        // Private parameters
        let private_parameters_as_list: Vec<Witness> =
            circuit.private_parameters.iter().cloned().collect();
        for private_parameter_witness in private_parameters_as_list {
            self._register_new_private_input_from_witness(private_parameter_witness);
        }
    }

    fn _register_new_public_input_from_witness(self: &mut Self, public_input_witness: Witness) {
        let public_input_target = self.builder.add_virtual_target();
        self.builder.register_public_input(public_input_target);
        self.witness_target_map
            .insert(public_input_witness, public_input_target);
    }

    fn _register_new_private_input_from_witness(self: &mut Self, private_input_witness: Witness) {
        self._get_or_create_target_for_witness(private_input_witness);
    }

    /// This method is key. The ACIR Opcodes talk about witnesses, while Plonky2 operates with
    /// targets, so when we want to know which target corresponds to a certain witness we might
    /// encounter that there isn't a target yet in the builder for the witness, so we need to
    /// create it.
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
