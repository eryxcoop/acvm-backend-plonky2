use super::*;
use crate::circuit_translation;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::witness::PartialWitness;
use plonky2::iop::witness::WitnessWrite;
use plonky2::plonk::proof::ProofWithPublicInputs;

pub fn generate_plonky2_circuit_from_acir_circuit(
    circuit: &Circuit,
) -> (CircuitData<F, C, 2>, HashMap<Witness, Target>) {
    let mut translator = circuit_translation::CircuitBuilderFromAcirToPlonky2::new();
    translator.translate_circuit(circuit);
    translator.unpack()
}

pub fn generate_plonky2_proof_using_witness_values(
    witness_assignment: Vec<(Witness, F)>,
    witness_target_map: &HashMap<Witness, Target>,
    circuit_data: &CircuitData<F, C, 2>,
) -> ProofWithPublicInputs<GoldilocksField, C, 2> {
    let mut witnesses = PartialWitness::<F>::new();
    for (witness, value) in witness_assignment {
        let plonky2_target = witness_target_map.get(&witness).unwrap();
        witnesses.set_target(*plonky2_target, value);
    }
    circuit_data.prove(witnesses).unwrap()
}

pub fn check_linked_output_targets_property(
    circuit: &Circuit,
    witness_target_map: &HashMap<Witness, Target>,
) {
    // We must make sure that all targets linked to output witness exist and are actual Wires
    // (instead of VirtualTargets). Otherwise, it means that te circuit is not doing what we
    // expect and might fall into false positive tests.
    for witness_index in circuit.return_values.indices() {
        match witness_target_map.get(&Witness(witness_index)) {
            Some(target) => match target {
                Target::VirtualTarget { index: _ } => {
                    panic!("{}", format!("Target corresponding to witness {} is not linked to the circuit", witness_index))
                }
                Target::Wire(_wire) => {}
            },
            None => panic!("An output witness has not an associated target"),
        }
    }
}
