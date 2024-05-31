use acir::circuit::opcodes::BlockId;
use crate::circuit_translation::tests::factories::circuit_factory;
use crate::circuit_translation::tests::factories::utils::*;
use super::*;

#[test]
fn test_plonky2_backend_can_translate_a_program_with_basic_memory_operations(){
    /*fn main(mut pub x: [Field; 1], mut y: Field){
        x[y] = 1;
        assert(x[0] == 1);
    }*/

    //Given
    let array_only_position_input_witness = Witness(0);
    let index_input_witness = Witness(1);
    let circuit = _memory_opcodes_circuit(array_only_position_input_witness, index_input_witness);

    // When
    let (circuit_data, witness_target_map) = generate_plonky2_circuit_from_acir_circuit(&circuit);

    //Then
    let zero = F::from_canonical_u64(0);
    let proof = generate_plonky2_proof_using_witness_values(
        vec![(array_only_position_input_witness, zero),
             (index_input_witness, zero)], &witness_target_map, &circuit_data);
    circuit_data.verify(proof).expect("Verification failed");
}

fn _memory_opcodes_circuit(array_only_position_input_witness: Witness, index_input_witness: Witness) -> Circuit {
    /*
    INIT (id: 0, len: 1)
    EXPR [ (-1, _2) 1 ]
    MEM (id: 0, write x2 at: x1)
    EXPR [ (-1, _3) 0 ]
    MEM (id: 0, read at: x3, value: x4)
    EXPR [ (1, _4) -1 ]
    */
    Circuit {
        current_witness_index: 0,
        expression_width: ExpressionWidth::Unbounded,
        opcodes: vec![
            Opcode::MemoryInit {
                block_id: BlockId(0),
                init: vec![array_only_position_input_witness]
            },
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![(-FieldElement::one(), Witness(2))],
                q_c: FieldElement::one(),
            }),
            Opcode::MemoryOp {
                block_id: BlockId(0),
                op: MemOp {
                    operation: expression_one(),
                    index: expression_witness(index_input_witness),
                    value: expression_witness(Witness(2))
                },
                predicate: None
            },
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![(-FieldElement::one(), Witness(3))],
                q_c: FieldElement::zero(),
            }),
            Opcode::MemoryOp {
                block_id: BlockId(0),
                op: MemOp {
                    operation: expression_zero(),
                    index: expression_witness(Witness(3)),
                    value: expression_witness(Witness(4))
                },
                predicate: None
            },
            Opcode::AssertZero(Expression {
                mul_terms: Vec::new(),
                linear_combinations: vec![(-FieldElement::one(), Witness(4))],
                q_c: FieldElement::one(),
            }),
        ],
        private_parameters: BTreeSet::new(),
        public_parameters: PublicInputs(BTreeSet::from_iter(
            vec![array_only_position_input_witness, index_input_witness])),
        return_values: PublicInputs(BTreeSet::new()),
        assert_messages: Default::default(),
        recursive: false,
    }
}

fn expression_one() -> Expression{
    Expression {
        mul_terms: Vec::new(),
        linear_combinations: Vec::new(),
        q_c: FieldElement::one(),
    }
}

fn expression_zero() -> Expression{
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