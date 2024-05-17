use super::*;

pub fn x_equals_0_opcode(public_input_witness: Witness) -> Opcode {
    AssertZero(Expression {
        mul_terms: Vec::new(),
        linear_combinations: vec![(FieldElement::one(), public_input_witness)],
        q_c: FieldElement::zero(),
    })
}

pub fn circuit_with_single_opcode(only_expr: Opcode, public_input_witnesses: Vec<Witness>) -> Circuit {
    Circuit {
        current_witness_index: 0,
        expression_width: ExpressionWidth::Unbounded,
        opcodes: vec![only_expr],
        private_parameters: BTreeSet::new(),
        public_parameters: PublicInputs(BTreeSet::from_iter(public_input_witnesses)),
        return_values: PublicInputs(BTreeSet::new()),
        assert_messages: Default::default(),
        recursive: false,
    }
}
pub fn x_equals_4_opcode(public_input_witness: Witness) -> Opcode {
    AssertZero(Expression {
        mul_terms: Vec::new(),
        linear_combinations: vec![(FieldElement::one(), public_input_witness)],
        q_c: -FieldElement::from_hex("0x04").unwrap(),
    })
}
pub fn x_times_3_equals_12_opcode(public_input_witness: Witness) -> Opcode {
    AssertZero(Expression {
        mul_terms: Vec::new(),
        linear_combinations: vec![(FieldElement::from_hex("0x03").unwrap(), public_input_witness)],
        q_c: -FieldElement::from_hex("0x0C").unwrap(),
    })
}
pub fn x_times_3_plus_y_times_4_equals_constant(first_public_input_witness: Witness, second_public_input_witness: Witness) -> Opcode {
    AssertZero(Expression {
        mul_terms: vec![],
        linear_combinations: vec![
            (FieldElement::from_hex("0x03").unwrap(), first_public_input_witness),
            (FieldElement::from_hex("0x09").unwrap(), second_public_input_witness),
        ],
        q_c: -FieldElement::from_hex("0x0c").unwrap(),
    })
}
pub fn multiple_linear_combinations_opcode(public_inputs: &Vec<Witness>) -> Opcode {
    AssertZero(Expression {
        mul_terms: vec![],
        linear_combinations: public_inputs.iter().map(|a_witness| (FieldElement::from_hex("0x03").unwrap(), *a_witness)).rev().collect(),
        q_c: -FieldElement::from_hex("0x0c").unwrap(),
    })
}
pub fn two_times_x_times_x_opcode(public_input: Witness) -> Opcode {
    AssertZero(Expression {
        mul_terms: vec![(FieldElement::from_hex("0x02").unwrap(), public_input, public_input)],
        linear_combinations: vec![],
        q_c: -FieldElement::from_hex("0x20").unwrap(),
    })
}
pub fn two_times_x_times_y_opcode(public_input_1: Witness, public_input_2: Witness) -> Opcode {
    AssertZero(Expression {
        mul_terms: vec![(FieldElement::from_hex("0x02").unwrap(), public_input_1, public_input_2)],
        linear_combinations: vec![],
        q_c: -FieldElement::from_hex("0x28").unwrap(),
    })
}
pub fn multiple_cuadratic_terms_opcode(public_inputs: &Vec<Witness>) -> Opcode {
    AssertZero(Expression {
        mul_terms: vec![
            (FieldElement::from_hex("0x02").unwrap(), public_inputs[0], public_inputs[0]),
            (FieldElement::from_hex("0x03").unwrap(), public_inputs[0], public_inputs[1]),
            (FieldElement::from_hex("0x04").unwrap(), public_inputs[1], public_inputs[2]),
            (FieldElement::from_hex("0x05").unwrap(), public_inputs[2], public_inputs[3]),
            (FieldElement::from_hex("0x06").unwrap(), public_inputs[3], public_inputs[3]),
            (FieldElement::from_hex("0x07").unwrap(), public_inputs[1], public_inputs[1]),
        ],
        linear_combinations: vec![],
        q_c: -FieldElement::from_hex("0x6c").unwrap(),
    })
}
pub fn multiple_cuadratic_terms_and_linear_combinations_opcode(public_inputs: &Vec<Witness>) -> Opcode {
    AssertZero(Expression {
        mul_terms: vec![
            (FieldElement::from_hex("0x02").unwrap(), public_inputs[0], public_inputs[0]),
            (FieldElement::from_hex("0x03").unwrap(), public_inputs[0], public_inputs[1]),
            (FieldElement::from_hex("0x04").unwrap(), public_inputs[1], public_inputs[2]),
            (FieldElement::from_hex("0x05").unwrap(), public_inputs[2], public_inputs[3]),
            (FieldElement::from_hex("0x06").unwrap(), public_inputs[3], public_inputs[3]),
            (FieldElement::from_hex("0x07").unwrap(), public_inputs[1], public_inputs[1]),
        ],
        linear_combinations: vec![
            (FieldElement::from_hex("0x01").unwrap(), public_inputs[0]),
            (FieldElement::from_hex("0x02").unwrap(), public_inputs[1]),
            (FieldElement::from_hex("0x03").unwrap(), public_inputs[2]),
            (FieldElement::from_hex("0x04").unwrap(), public_inputs[3]),
        ],
        q_c: -FieldElement::from_hex("0x80").unwrap(),
    })
}
pub fn circuit_with_a_public_input_and_two_assert_zero_operands(public_input_witness: Witness,
                                                            intermediate_witness: Witness) -> Circuit {
    Circuit {
        current_witness_index: 0,
        expression_width: ExpressionWidth::Unbounded,
        opcodes: vec![
            AssertZero(Expression {
                mul_terms: vec![],
                linear_combinations: vec![(FieldElement::one(), public_input_witness), (-FieldElement::one(), intermediate_witness)],
                q_c: FieldElement::from_hex("0x04").unwrap(),
            }),
            AssertZero(Expression {
                mul_terms: vec![(FieldElement::one(), intermediate_witness, intermediate_witness)],
                linear_combinations: vec![],
                q_c: -FieldElement::from_hex("0x19").unwrap(),
            }),
        ],
        private_parameters: BTreeSet::new(),
        public_parameters: PublicInputs(BTreeSet::from_iter(vec![public_input_witness])),
        return_values: PublicInputs(BTreeSet::new()),
        assert_messages: Default::default(),
        recursive: false,
    }
}