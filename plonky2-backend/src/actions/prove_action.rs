use std::collections::HashMap;

use num_bigint::BigUint;

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::proof::ProofWithPublicInputs;

use crate::circuit_translation;
use crate::circuit_translation::*;
use crate::noir_and_plonky2_serialization::*;
use super::*;

pub struct ProveAction {
    pub acir_program_json_path: String,
    pub witness_stack_zip_path: String,
    pub resulting_proof_file_path: String,
}

impl ProveAction {
    pub fn run(&self) {
        let acir_program: Program =
            deserialize_program_within_file_path(&self.acir_program_json_path);
        let witness_stack: WitnessStack =
            deserialize_witnesses_within_file_path(self.witness_stack_zip_path.clone());

        let circuit = &acir_program.functions[0];
        let (circuit_data, witness_target_map) =
            self.generate_plonky2_circuit_from_acir_circuit(circuit);
        let proof = self.generate_serialized_plonky2_proof(
            witness_stack,
            &witness_target_map,
            &circuit_data,
        );

        self._write_proof_into_file(proof, &self.resulting_proof_file_path);
    }

    fn _write_proof_into_file(&self, proof: Vec<u8>, proof_path: &String) {
        write_bytes_to_file_path(proof, proof_path)
    }

    pub fn generate_plonky2_circuit_from_acir_circuit(
        &self,
        circuit: &Circuit,
    ) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>) {
        let mut translator = circuit_translation::CircuitBuilderFromAcirToPlonky2::new();
        translator.translate_circuit(circuit);
        translator.unpack()
    }

    fn _field_element_to_goldilocks_field(&self, fe: &FieldElement) -> F {
        let fe_as_big_uint = BigUint::from_bytes_be(&fe.to_be_bytes() as &[u8]);
        F::from_noncanonical_biguint(fe_as_big_uint)
    }

    fn generate_serialized_plonky2_proof(
        &self,
        mut witness_stack: WitnessStack,
        witness_target_map: &HashMap<Witness, Target>,
        circuit_data: &CircuitData<F, C, 2>,
    ) -> Vec<u8> {
        let proof = self.generate_plonky2_proof_from_witness_stack(
            &mut witness_stack,
            witness_target_map,
            circuit_data,
        );
        let verifier_data_digest = &circuit_data.verifier_only.circuit_digest;
        let common = &circuit_data.common;
        let compressed_proof = proof.compress(verifier_data_digest, common).unwrap();
        compressed_proof.to_bytes()
    }

    pub fn generate_plonky2_proof_from_witness_stack(
        &self,
        mut witness_stack: &mut WitnessStack,
        witness_target_map: &HashMap<Witness, Target>,
        circuit_data: &CircuitData<GoldilocksField, C, 2>,
    ) -> ProofWithPublicInputs<GoldilocksField, C, 2> {
        let witnesses = self._extract_witnesses(&mut witness_stack, witness_target_map);
        self.generate_plonky2_proof_from_partial_witnesses(circuit_data, witnesses)
    }

    pub fn generate_plonky2_proof_from_partial_witnesses(
        &self,
        circuit_data: &CircuitData<GoldilocksField, C, 2>,
        witnesses: PartialWitness<GoldilocksField>,
    ) -> ProofWithPublicInputs<GoldilocksField, C, 2> {
        circuit_data.prove(witnesses).unwrap()
    }

    fn _extract_witnesses(
        &self,
        witness_stack: &mut WitnessStack,
        witness_target_map: &HashMap<Witness, Target>,
    ) -> PartialWitness<GoldilocksField> {
        let mut witnesses = PartialWitness::<F>::new();
        let witness_map = witness_stack.pop().unwrap().witness;
        for (witness, value) in witness_map.into_iter() {
            let plonky2_target = witness_target_map.get(&witness).unwrap();
            witnesses.set_target(
                *plonky2_target,
                self._field_element_to_goldilocks_field(&value),
            );
        }
        witnesses
    }
}
