use super::*;
use acir::circuit::opcodes::FunctionInput;
use acir::circuit::{opcodes, ExpressionWidth, PublicInputs};
use std::collections::BTreeSet;

pub fn x_equals_0_opcode(public_input_witness: Witness) -> Opcode {
    Opcode::AssertZero(Expression {
        mul_terms: Vec::new(),
        linear_combinations: vec![(FieldElement::one(), public_input_witness)],
        q_c: FieldElement::zero(),
    })
}

pub fn circuit_with_single_opcode(
    only_expr: Opcode,
    public_input_witnesses: Vec<Witness>,
) -> Circuit {
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
    Opcode::AssertZero(Expression {
        mul_terms: Vec::new(),
        linear_combinations: vec![(FieldElement::one(), public_input_witness)],
        q_c: -FieldElement::from_hex("0x04").unwrap(),
    })
}

pub fn x_times_3_equals_12_opcode(public_input_witness: Witness) -> Opcode {
    Opcode::AssertZero(Expression {
        mul_terms: Vec::new(),
        linear_combinations: vec![(
            FieldElement::from_hex("0x03").unwrap(),
            public_input_witness,
        )],
        q_c: -FieldElement::from_hex("0x0C").unwrap(),
    })
}

pub fn x_times_3_plus_y_times_4_equals_constant(
    first_public_input_witness: Witness,
    second_public_input_witness: Witness,
) -> Opcode {
    Opcode::AssertZero(Expression {
        mul_terms: vec![],
        linear_combinations: vec![
            (
                FieldElement::from_hex("0x03").unwrap(),
                first_public_input_witness,
            ),
            (
                FieldElement::from_hex("0x09").unwrap(),
                second_public_input_witness,
            ),
        ],
        q_c: -FieldElement::from_hex("0x0c").unwrap(),
    })
}

pub fn multiple_linear_combinations_opcode(public_inputs: &Vec<Witness>) -> Opcode {
    Opcode::AssertZero(Expression {
        mul_terms: vec![],
        linear_combinations: public_inputs
            .iter()
            .map(|a_witness| (FieldElement::from_hex("0x03").unwrap(), *a_witness))
            .rev()
            .collect(),
        q_c: -FieldElement::from_hex("0x0c").unwrap(),
    })
}

pub fn two_times_x_times_x_opcode(public_input: Witness) -> Opcode {
    Opcode::AssertZero(Expression {
        mul_terms: vec![(
            FieldElement::from_hex("0x02").unwrap(),
            public_input,
            public_input,
        )],
        linear_combinations: vec![],
        q_c: -FieldElement::from_hex("0x20").unwrap(),
    })
}

pub fn two_times_x_times_y_opcode(public_input_1: Witness, public_input_2: Witness) -> Opcode {
    Opcode::AssertZero(Expression {
        mul_terms: vec![(
            FieldElement::from_hex("0x02").unwrap(),
            public_input_1,
            public_input_2,
        )],
        linear_combinations: vec![],
        q_c: -FieldElement::from_hex("0x28").unwrap(),
    })
}

pub fn multiple_cuadratic_terms_opcode(public_inputs: &Vec<Witness>) -> Opcode {
    Opcode::AssertZero(Expression {
        mul_terms: vec![
            (
                FieldElement::from_hex("0x02").unwrap(),
                public_inputs[0],
                public_inputs[0],
            ),
            (
                FieldElement::from_hex("0x03").unwrap(),
                public_inputs[0],
                public_inputs[1],
            ),
            (
                FieldElement::from_hex("0x04").unwrap(),
                public_inputs[1],
                public_inputs[2],
            ),
            (
                FieldElement::from_hex("0x05").unwrap(),
                public_inputs[2],
                public_inputs[3],
            ),
            (
                FieldElement::from_hex("0x06").unwrap(),
                public_inputs[3],
                public_inputs[3],
            ),
            (
                FieldElement::from_hex("0x07").unwrap(),
                public_inputs[1],
                public_inputs[1],
            ),
        ],
        linear_combinations: vec![],
        q_c: -FieldElement::from_hex("0x6c").unwrap(),
    })
}

pub fn multiple_cuadratic_terms_and_linear_combinations_opcode(
    public_inputs: &Vec<Witness>,
) -> Opcode {
    Opcode::AssertZero(Expression {
        mul_terms: vec![
            (
                FieldElement::from_hex("0x02").unwrap(),
                public_inputs[0],
                public_inputs[0],
            ),
            (
                FieldElement::from_hex("0x03").unwrap(),
                public_inputs[0],
                public_inputs[1],
            ),
            (
                FieldElement::from_hex("0x04").unwrap(),
                public_inputs[1],
                public_inputs[2],
            ),
            (
                FieldElement::from_hex("0x05").unwrap(),
                public_inputs[2],
                public_inputs[3],
            ),
            (
                FieldElement::from_hex("0x06").unwrap(),
                public_inputs[3],
                public_inputs[3],
            ),
            (
                FieldElement::from_hex("0x07").unwrap(),
                public_inputs[1],
                public_inputs[1],
            ),
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

pub fn black_box_range_opcode(public_input: Witness, max_bits: u32) -> Opcode {
    let input = FunctionInput {
        witness: public_input,
        num_bits: max_bits,
    };
    Opcode::BlackBoxFuncCall(opcodes::BlackBoxFuncCall::RANGE { input })
}

pub fn circuit_with_a_public_input_and_two_assert_zero_operands(
    public_input_witness: Witness,
    intermediate_witness: Witness,
) -> Circuit {
    Circuit {
        current_witness_index: 0,
        expression_width: ExpressionWidth::Unbounded,
        opcodes: vec![
            Opcode::AssertZero(Expression {
                mul_terms: vec![],
                linear_combinations: vec![
                    (FieldElement::one(), public_input_witness),
                    (-FieldElement::one(), intermediate_witness),
                ],
                q_c: FieldElement::from_hex("0x04").unwrap(),
            }),
            Opcode::AssertZero(Expression {
                mul_terms: vec![(
                    FieldElement::one(),
                    intermediate_witness,
                    intermediate_witness,
                )],
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

pub fn bitwise_and_circuit(
    input_1: Witness,
    input_2: Witness,
    output: Witness,
    bit_size: u32,
) -> Circuit {
    // BLACKBOX::RANGE [(_0, num_bits: max_bits)] [ ]
    // BLACKBOX::RANGE [(_1, num_bits: max_bits)] [ ]
    // BLACKBOX::AND [(_0, num_bits: max_bits), (_1, num_bits: max_bits)] [ _2]

    _circuit_with_bitwise_operation(
        input_1,
        input_2,
        _bitwise_and_acir_opcode(output, input_1, input_2, bit_size),
        bit_size,
    )
}

pub fn bitwise_xor_circuit(
    input_1: Witness,
    input_2: Witness,
    output: Witness,
    bit_size: u32,
) -> Circuit {
    // BLACKBOX::RANGE [(_0, num_bits: max_bits)] [ ]
    // BLACKBOX::RANGE [(_1, num_bits: max_bits)] [ ]
    // BLACKBOX::XOR [(_0, num_bits: max_bits), (_1, num_bits: max_bits)] [ _2]

    _circuit_with_bitwise_operation(
        input_1,
        input_2,
        _bitwise_xor_acir_opcode(output, input_1, input_2, bit_size),
        bit_size,
    )
}

fn _circuit_with_bitwise_operation(
    input_1: Witness,
    input_2: Witness,
    opcode: Opcode,
    bit_size: u32,
) -> Circuit {
    // BLACKBOX::RANGE [(_0, num_bits: max_bits)] [ ]
    // BLACKBOX::RANGE [(_1, num_bits: max_bits)] [ ]
    // BLACKBOX::OPCODE [(_0, num_bits: max_bits), (_1, num_bits: max_bits)] [ _2]

    Circuit {
        current_witness_index: 0,
        expression_width: ExpressionWidth::Unbounded,
        opcodes: vec![
            black_box_range_opcode(input_1, bit_size),
            black_box_range_opcode(input_2, bit_size),
            opcode,
        ],
        private_parameters: BTreeSet::new(),
        public_parameters: PublicInputs(BTreeSet::from_iter(vec![input_1, input_2])),
        return_values: PublicInputs(BTreeSet::from_iter([Witness(2)])),
        assert_messages: Default::default(),
        recursive: false,
    }
}

fn _bitwise_and_acir_opcode(
    output: Witness,
    input_1: Witness,
    input_2: Witness,
    bit_size: u32,
) -> Opcode {
    let and_lhs = FunctionInput {
        witness: input_1,
        num_bits: bit_size,
    };
    let and_rhs = FunctionInput {
        witness: input_2,
        num_bits: bit_size,
    };

    Opcode::BlackBoxFuncCall(opcodes::BlackBoxFuncCall::AND {
        lhs: and_lhs,
        rhs: and_rhs,
        output,
    })
}

fn _bitwise_xor_acir_opcode(
    output: Witness,
    input_1: Witness,
    input_2: Witness,
    max_bits: u32,
) -> Opcode {
    let and_lhs = FunctionInput {
        witness: input_1,
        num_bits: max_bits,
    };
    let and_rhs = FunctionInput {
        witness: input_2,
        num_bits: max_bits,
    };

    Opcode::BlackBoxFuncCall(opcodes::BlackBoxFuncCall::XOR {
        lhs: and_lhs,
        rhs: and_rhs,
        output,
    })
}
