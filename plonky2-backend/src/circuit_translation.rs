use std::collections::{BTreeSet, HashMap};
use acir::circuit::{Circuit, ExpressionWidth, PublicInputs};
use acir::circuit::Opcode::AssertZero;
use acir::FieldElement;
use acir::native_types::{Expression, Witness};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::{Field, Field64};
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig};
use num_bigint::BigUint;

const D: usize = 2;
type C = KeccakGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;
type CB = CircuitBuilder::<F, D>;

fn translate_assert_zero(builder: &mut CB, expression: &Expression, witness_target_map: &mut HashMap<Witness, Target>) {
    assert_eq!(0, expression.mul_terms.len());
    assert_eq!(FieldElement::zero(), expression.q_c);
    let linear_combinations = &expression.linear_combinations;
    assert_eq!(1, linear_combinations.len());
    let (field_element, witness) = &linear_combinations[0];

    let target = if let Some(target) = witness_target_map.get(witness) {
        *target
    } else {
        let new_target = builder.add_virtual_target();
        witness_target_map.insert(*witness, new_target);
        new_target
    };

    let mut g_field_element= GoldilocksField::default();
    g_field_element.add_one();

    let result_target = builder.mul_const(g_field_element, target);
    builder.assert_zero(result_target);
}


fn generate_plonky2_circuit(circuit: &Circuit) -> CircuitData<GoldilocksField, C, 2>{
    let config = CircuitConfig {
        zero_knowledge: true,
        ..CircuitConfig::standard_ecc_config()
    };
    let mut builder = CB::new(config);
    let mut witness_target_map: HashMap<Witness, Target> = HashMap::new();

    let public_input_1 = circuit.public_parameters.0.first().unwrap();
    let target_1 = builder.add_virtual_target();

    witness_target_map.insert(*public_input_1, target_1);

    for opcode in &circuit.opcodes {
        match opcode {
            AssertZero(expr) => translate_assert_zero(&mut builder, &expr, &mut witness_target_map),
            _ => { () }
        }
    }

    builder.build::<C>()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xxxxx(){
        let only_expr = Expression {mul_terms: Vec::new(),
                                    linear_combinations: vec![(FieldElement::one(), Witness(0))],
                                    q_c: FieldElement::zero() };

        let circuit = Circuit {
            current_witness_index: 0,
            expression_width: ExpressionWidth::Unbounded,
            opcodes: Vec::from([AssertZero(only_expr)]),
            private_parameters: BTreeSet::new(),
            public_parameters: PublicInputs(BTreeSet::from_iter(vec![Witness(0)])),
            return_values: PublicInputs(BTreeSet::new()),
            assert_messages: Default::default(),
            recursive: false,
        };

        let xxxx = generate_plonky2_circuit(&circuit);

    }
}
