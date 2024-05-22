use super::*;

// This is a planned refactor, currently this functionality is implemented in the
// CircuitBuilderFromAcirToPlonky2 struct
struct AssertZeroTranslator {
    builder: CB,
    witness_target_map: HashMap<Witness, Target>,
    expr: Expression,
}

impl AssertZeroTranslator {

}