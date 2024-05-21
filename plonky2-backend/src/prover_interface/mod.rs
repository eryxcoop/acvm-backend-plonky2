use std::collections::HashMap;
use acir::circuit::{Circuit, Program};
use acir::FieldElement;
use acir::native_types::{Witness, WitnessStack};
use num_bigint::BigUint;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig};
use crate::circuit_translation;

const D: usize = 2;
type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

pub struct ProverInterface;

impl ProverInterface {
    pub fn write_proof_in_standard_output(&self, acir_program: Program, mut witness_stack: WitnessStack) {
        let functions = acir_program.functions;
        let circuit = &functions[0];

        let (circuit_data, witness_target_map) =
            self.generate_plonky2_circuit_from_acir_circuit(circuit);
        let proof = self.generate_plonky2_proof_using_witness_values(
            witness_stack, &witness_target_map, &circuit_data);
        println!("{:?}", proof);
    }

    pub fn generate_plonky2_circuit_from_acir_circuit(&self, circuit: &Circuit) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>) {
        let mut translator = circuit_translation::CircuitBuilderFromAcirToPlonky2::new();
        translator.translate_circuit(circuit);
        let circuit_translation::CircuitBuilderFromAcirToPlonky2 { builder, witness_target_map } = translator;
        (builder.build::<C>(), witness_target_map)
    }

    fn _field_element_to_goldilocks_field(&self, fe: &FieldElement) -> F {
        let fe_as_big_uint = BigUint::from_bytes_be(&fe.to_be_bytes() as &[u8]);
        F::from_noncanonical_biguint(fe_as_big_uint)
    }

    pub fn generate_plonky2_proof_using_witness_values(&self, mut witness_stack: WitnessStack,
                                                   witness_target_map: &HashMap<Witness, Target>,
                                                   circuit_data: &CircuitData<F, C, 2>) -> Vec<u8> {
        let witnesses = self._extract_witnesses(&mut witness_stack, witness_target_map);
        let verifier_data_digest = &circuit_data.verifier_only.circuit_digest;
        let common = &circuit_data.common;
        let proof = circuit_data.prove(witnesses);
        proof.unwrap().compress(verifier_data_digest, common).unwrap().to_bytes()
    }

    fn _extract_witnesses(&self, witness_stack: &mut WitnessStack, witness_target_map: &HashMap<Witness, Target>) -> PartialWitness<GoldilocksField> {
        let mut witnesses = PartialWitness::<F>::new();
        let mut witness_map = witness_stack.pop().unwrap().witness;
        for (witness, value) in witness_map.into_iter() {
            let plonky2_target = witness_target_map.get(&witness).unwrap();
            witnesses.set_target(*plonky2_target, self._field_element_to_goldilocks_field(&value));
        }
        witnesses
    }
}