use crate::actions::prove_action::ProveAction;
use super::*;

pub fn generate_plonky2_circuit_from_acir_circuit(circuit: &Circuit) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>) {
    let prove_action = ProveAction.initialize_empty();
    prove_action.generate_plonky2_circuit_from_acir_circuit(circuit)
}

pub fn generate_plonky2_proof_using_witness_values(witness_assignment: Vec<(Witness, F)>,
                                               witness_target_map: &HashMap<Witness, Target>,
                                               circuit_data: &CircuitData<F, C, 2>) -> ProofWithPublicInputs<GoldilocksField, C, 2> {
    let mut witnesses = PartialWitness::<F>::new();
    for (witness, value) in witness_assignment {
        let plonky2_target = witness_target_map.get(&witness).unwrap();
        witnesses.set_target(*plonky2_target, value);
    }
    let prove_action = ProveAction.initialize_empty();
    prove_action.generate_plonky2_proof_from_partial_witnesses(circuit_data, witnesses)
}
