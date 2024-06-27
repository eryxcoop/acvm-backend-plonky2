use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::iop::witness::WitnessWrite;
use crate::circuit_translation;
use super::*;

pub fn generate_plonky2_circuit_from_acir_circuit(circuit: &Circuit) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>) {
    let mut translator = circuit_translation::CircuitBuilderFromAcirToPlonky2::new();
    translator.translate_circuit(circuit);
    translator.unpack()
}


pub fn generate_plonky2_proof_using_witness_values(witness_assignment: Vec<(Witness, F)>,
                                               witness_target_map: &HashMap<Witness, Target>,
                                               circuit_data: &CircuitData<F, C, 2>) -> ProofWithPublicInputs<GoldilocksField, C, 2> {
    let mut witnesses = PartialWitness::<F>::new();
    for (witness, value) in witness_assignment {
        let plonky2_target = witness_target_map.get(&witness).unwrap();
        witnesses.set_target(*plonky2_target, value);
    }
    circuit_data.prove(witnesses).unwrap()
}
