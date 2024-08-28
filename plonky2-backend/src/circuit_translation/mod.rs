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
use sha256_translator::Sha256CompressionTranslator;
use crate::circuit_translation::ecdsa_secp256k1_translator::EcdsaSecp256k1Translator;

#[cfg(test)]
mod tests;

pub mod assert_zero_translator;
mod ecdsa_secp256k1_translator;

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
                            assert!(long_max_bits <= 33,
                                    "Range checks with more than 33 bits are not allowed yet while using Plonky2 prover");
                            let witness = input.witness;
                            let target = self._get_or_create_target_for_witness(witness);
                            self.builder.range_check(target, long_max_bits)
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
                            self._extend_circuit_with_bitwise_operation(
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
                        opcodes::BlackBoxFuncCall::EcdsaSecp256k1 {
                            public_key_x,
                            public_key_y,
                            signature,
                            hashed_message,
                            output,
                        } => {
                            self._extend_circuit_with_ecdsa_secp256k1_operation(
                                public_key_x,
                                public_key_y,
                                signature,
                                hashed_message,
                                *output
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

    fn _extend_circuit_with_ecdsa_secp256k1_operation(
        &mut self,
        public_key_x: &Box<[FunctionInput; 32]>,
        public_key_y: &Box<[FunctionInput; 32]>,
        signature: &Box<[FunctionInput; 64]>,
        hashed_message: &Box<[FunctionInput; 32]>,
        output: Witness,
    ) {
        let mut ecdsa_secp256k1_translator =
            EcdsaSecp256k1Translator::new_for(self, hashed_message, public_key_x, public_key_y, signature, output);
        ecdsa_secp256k1_translator.translate();
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

    pub fn target_for_witness(&mut self, w: Witness) -> Target {
        self._get_or_create_target_for_witness(w)
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
