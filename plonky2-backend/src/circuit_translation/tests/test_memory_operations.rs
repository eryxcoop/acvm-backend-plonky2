use super::*;
use crate::circuit_translation::tests::factories::utils::*;
use acir::circuit::opcodes::BlockId;
use acir::circuit::opcodes::BlockType::Memory;
use acir::circuit::{ExpressionWidth, PublicInputs};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use std::collections::BTreeSet;

#[test]
fn test_plonky2_backend_can_translate_a_read_memory_operation() {
    /*fn main(mut x: pub [Field; 2], y: pub Field){
        assert(x[0] == x[y]);
    }*/
    //Given
    let array_input_witnesses = [Witness(0), Witness(1)];
    let index_input_witness = Witness(2);
    let circuit = _memory_simple_read_circuit(array_input_witnesses.to_vec(), index_input_witness);

    //When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let zero = F::from_canonical_u64(0);
    let one = F::from_canonical_u64(1);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![
            (array_input_witnesses[0], zero),
            (array_input_witnesses[1], zero),
            (index_input_witness, one),
            (Witness(3), zero),
        ],
        &witness_target_map,
        &circuit_data,
    );
    assert!(circuit_data.verify(proof).is_ok());
}

#[test]
fn test_plonky2_backend_can_translate_a_program_with_basic_memory_write() {
    /*fn main(mut x: pub [Field; 2], y: pub Field, v: pub Field){
        x[y] = v;
        assert(x[0] == 1);
        assert(x[1] == 11);
    }*/

    //Given
    let array_input_witnesses = vec![Witness(0), Witness(1)];
    let index_input_witness = Witness(2);
    let value_input_witness = Witness(3);
    let circuit = _memory_simple_write_circuit(
        array_input_witnesses.clone(),
        index_input_witness,
        value_input_witness,
    );

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let zero = F::from_canonical_u64(0);
    let one = F::from_canonical_u64(1);
    let ten = F::from_canonical_u64(10);
    let eleven = F::from_canonical_u64(11);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![
            (array_input_witnesses[0], ten),
            (array_input_witnesses[1], eleven),
            (index_input_witness, zero),
            (value_input_witness, one),
            (Witness(4), zero),
            (Witness(5), one),
            (Witness(6), one),
            (Witness(7), eleven),
        ],
        &witness_target_map,
        &circuit_data,
    );
    assert!(circuit_data.verify(proof).is_ok());
}

#[test]
fn test_backend_supports_creation_of_memory_blocks_with_irregular_size() {
    // Irregular means that is not a power of 2.
    // Apparently the way plonky2 handles random access requires a vector with a length of power of 2
    // The solution is appending dummy targets to fill the necessary spaces when reading a block

    // fn main(mut v: pub [Field; 3], idx: pub Field){
    //     assert(v[idx] == 5);
    // }

    //Given
    let array_input_witnesses = vec![Witness(0), Witness(1), Witness(2)];
    let index_input_witness = Witness(3);
    let circuit =
        _read_memory_of_length_3_circuit(array_input_witnesses.clone(), index_input_witness);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let zero = F::from_canonical_u64(0);
    let five = F::from_canonical_u64(5);
    let ten = F::from_canonical_u64(10);
    let eleven = F::from_canonical_u64(11);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![
            (array_input_witnesses[0], five),
            (array_input_witnesses[1], ten),
            (array_input_witnesses[2], eleven),
            (index_input_witness, zero),
            (Witness(4), five),
        ],
        &witness_target_map,
        &circuit_data,
    );
    assert!(circuit_data.verify(proof).is_ok());
}

// Test less or equal

#[test]
#[ignore]
fn test_brute_force_range_checks_up_to_17() {
    let max = 17;
    for (max_value, target_value) in _generate_valid_combinations(max) {
        println!("(max: {}, target: {})", max_value, target_value);
        assert_target_is_less_or_equal(max_value, target_value);
    }
    use std::panic::{catch_unwind, AssertUnwindSafe};
    for (max_value, target_value) in _generate_invalid_combinations(max) {
        println!("(max: {}, target: {})", max_value, target_value);
        let result = catch_unwind(AssertUnwindSafe(|| {
            assert_target_is_less_or_equal(max_value, target_value);
        }));
        assert!(result.is_err(), "Expected panic");
    }
}

fn _generate_valid_combinations(max: u32) -> Vec<(usize, usize)> {
    let mut test_values: Vec<(usize, usize)> = Vec::new();
    for max_value in 0..max {
        for target_value in 0..max_value + 1 {
            test_values.push((max_value as usize, target_value as usize));
        }
    }
    test_values
}

fn _generate_invalid_combinations(max: u32) -> Vec<(usize, usize)> {
    let mut test_values: Vec<(usize, usize)> = Vec::new();
    for max_value in 0..max {
        for target_value in (max_value + 1)..max {
            test_values.push((max_value as usize, target_value as usize));
        }
    }
    test_values
}

fn assert_target_is_less_or_equal(max_allowed_value: usize, actual_target_value: usize) {
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let public_input_target = circuit_builder.add_virtual_public_input();
    MemoryOperationsTranslator::add_restrictions_to_assert_target_is_less_or_equal_to(
        max_allowed_value,
        public_input_target,
        &mut circuit_builder,
    );

    let mut partial_witnesses = PartialWitness::<F>::new();
    partial_witnesses.set_target(
        public_input_target,
        F::from_canonical_usize(actual_target_value),
    );

    let circuit_data = circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}

// ------------ CIRCUITS ------------ //

fn _read_memory_of_length_3_circuit(
    array_positions_input_witnesses: Vec<Witness>,
    index_input_witness: Witness,
) -> Circuit {
    // public parameters indices : [0, 1, 2, 3]
    // return value indices : []
    // INIT (id: 0, len: 3)
    // MEM (id: 0, read at: x3, value: x4)
    // EXPR [ (1, _4) -5 ]
    Circuit {
        current_witness_index: 0,
        expression_width: ExpressionWidth::Unbounded,
        opcodes: vec![
            Opcode::MemoryInit {
                block_id: BlockId(0),
                init: array_positions_input_witnesses.clone(),
                block_type: Memory,
            },
            Opcode::MemoryOp {
                block_id: BlockId(0),
                op: MemOp {
                    operation: expression_read(),
                    index: expression_witness(index_input_witness),
                    value: expression_witness(Witness(4)),
                },
                predicate: None,
            },
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![(FieldElement::one(), Witness(4))],
                q_c: -FieldElement::from_hex("0x05").unwrap(),
            }),
        ],
        private_parameters: BTreeSet::new(),
        public_parameters: PublicInputs(BTreeSet::from_iter(vec![
            array_positions_input_witnesses[0],
            array_positions_input_witnesses[1],
            array_positions_input_witnesses[2],
            index_input_witness,
        ])),
        return_values: PublicInputs(BTreeSet::new()),
        assert_messages: Default::default(),
        recursive: false,
    }
}

fn _memory_simple_read_circuit(
    array_positions_input_witnesses: Vec<Witness>,
    index_input_witness: Witness,
) -> Circuit {
    // INIT (id: 0, len: 2)
    // MEM (id: 0, read at: x2, value: x3)
    // EXPR [ (1, _0) (-1, _3) 0 ]
    Circuit {
        current_witness_index: 0,
        expression_width: ExpressionWidth::Unbounded,
        opcodes: vec![
            Opcode::MemoryInit {
                block_id: BlockId(0),
                init: array_positions_input_witnesses.clone(),
                block_type: Memory,
            },
            Opcode::MemoryOp {
                block_id: BlockId(0),
                op: MemOp {
                    operation: expression_read(),
                    index: expression_witness(index_input_witness),
                    value: expression_witness(Witness(3)),
                },
                predicate: None,
            },
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![
                    (
                        FieldElement::one(),
                        array_positions_input_witnesses[0].clone(),
                    ),
                    (-FieldElement::one(), Witness(3)),
                ],
                q_c: FieldElement::zero(),
            }),
        ],
        private_parameters: BTreeSet::new(),
        public_parameters: PublicInputs(BTreeSet::from_iter(vec![
            array_positions_input_witnesses[0],
            array_positions_input_witnesses[1],
            index_input_witness,
        ])),
        return_values: PublicInputs(BTreeSet::new()),
        assert_messages: Default::default(),
        recursive: false,
    }
}

fn _memory_simple_write_circuit(
    array_input_witnesses: Vec<Witness>,
    index_input_witness: Witness,
    value_input_witness: Witness,
) -> Circuit {
    /*
    INIT (id: 0, len: 2)
    MEM (id: 0, write x3 at: x2)
    EXPR [ (-1, _4) 0 ]
    MEM (id: 0, read at: x4, value: x5)
    EXPR [ (1, _5) -1 ]
    EXPR [ (-1, _6) 1 ]
    MEM (id: 0, read at: x6, value: x7)
    EXPR [ (1, _7) -11 ]
    */
    Circuit {
        current_witness_index: 0,
        expression_width: ExpressionWidth::Unbounded,
        opcodes: vec![
            Opcode::MemoryInit {
                block_id: BlockId(0),
                init: array_input_witnesses.clone(),
                block_type: Memory,
            },
            Opcode::MemoryOp {
                block_id: BlockId(0),
                op: MemOp {
                    operation: expression_write(),
                    index: expression_witness(index_input_witness),
                    value: expression_witness(value_input_witness),
                },
                predicate: None,
            },
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![(-FieldElement::one(), Witness(4))],
                q_c: FieldElement::zero(),
            }),
            Opcode::MemoryOp {
                block_id: BlockId(0),
                op: MemOp {
                    operation: expression_read(),
                    index: expression_witness(Witness(4)),
                    value: expression_witness(Witness(5)),
                },
                predicate: None,
            },
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![(FieldElement::one(), Witness(5))],
                q_c: -FieldElement::one(),
            }),
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![(-FieldElement::one(), Witness(6))],
                q_c: FieldElement::one(),
            }),
            Opcode::MemoryOp {
                block_id: BlockId(0),
                op: MemOp {
                    operation: expression_read(),
                    index: expression_witness(Witness(6)),
                    value: expression_witness(Witness(7)),
                },
                predicate: None,
            },
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![(-FieldElement::one(), Witness(7))],
                q_c: FieldElement::from_hex("0xB").unwrap(),
            }),
        ],
        private_parameters: BTreeSet::new(),
        public_parameters: PublicInputs(BTreeSet::from_iter(vec![
            array_input_witnesses[0],
            array_input_witnesses[1],
            index_input_witness,
            value_input_witness,
        ])),
        return_values: PublicInputs(BTreeSet::new()),
        assert_messages: Default::default(),
        recursive: false,
    }
}

fn expression_write() -> Expression {
    Expression {
        mul_terms: Vec::new(),
        linear_combinations: Vec::new(),
        q_c: FieldElement::one(),
    }
}

fn expression_read() -> Expression {
    Expression {
        mul_terms: Vec::new(),
        linear_combinations: Vec::new(),
        q_c: FieldElement::zero(),
    }
}

fn expression_witness(witness: Witness) -> Expression {
    Expression {
        mul_terms: Vec::new(),
        linear_combinations: vec![(FieldElement::one(), witness)],
        q_c: FieldElement::zero(),
    }
}

#[test]
fn plonky2_is_equal_test_positive() {
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let x = circuit_builder.add_virtual_target();
    let y = circuit_builder.add_virtual_target();

    let mut partial_witnesses = PartialWitness::<F>::new();
    let zero = F::from_canonical_u64(0);
    let one = F::from_canonical_u64(1);
    let is_equal = circuit_builder.is_equal(x, y);

    partial_witnesses.set_target(x, zero);
    partial_witnesses.set_target(y, zero);
    partial_witnesses.set_target(is_equal.target, one);

    let circuit_data = circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}

#[test]
fn plonky2_is_equal_test_negative() {
    let config = CircuitConfig::standard_recursion_config();
    let mut circuit_builder = CB::new(config);

    let x = circuit_builder.add_virtual_target();
    let y = circuit_builder.add_virtual_target();

    let mut partial_witnesses = PartialWitness::<F>::new();
    let one = F::from_canonical_u64(1);
    let zero = F::from_canonical_u64(0);
    let is_equal = circuit_builder.is_equal(x, y);

    partial_witnesses.set_target(x, one);
    partial_witnesses.set_target(y, zero);
    partial_witnesses.set_target(is_equal.target, zero);

    let circuit_data = circuit_builder.build::<C>();
    let proof = circuit_data.prove(partial_witnesses).unwrap();
    assert!(circuit_data.verify(proof).is_ok());
}
