use super::*;

/// This module performs memory operations such as creating blocks of memory, reading in a specific
/// position known in prove time and write a value in a position both known at prove time. The
/// length of the blocks is fixed and known in circuit building time.
///
/// The desired length of a memory block may not coincide with the length of the associated vector
/// of targets representation. This is because we must append some zeroes at the end for making the
/// length a power of 2, therefore the memory_blocks collaborator must hold the length of each
/// memory block.
pub struct MemoryOperationsTranslator<'a> {
    builder: &'a mut CircuitBuilder<F, D>,
    witness_target_map: &'a mut HashMap<Witness, Target>,
    memory_blocks: &'a mut HashMap<BlockId, (Vec<Target>, usize)>,
}

impl<'a> MemoryOperationsTranslator<'a> {
    pub fn new_for(
        builder: &'a mut CircuitBuilder<F, D>,
        witness_target_map: &'a mut HashMap<Witness, Target>,
        memory_blocks: &'a mut HashMap<BlockId, (Vec<Target>, usize)>,
    ) -> Self {
        Self {
            builder,
            witness_target_map,
            memory_blocks,
        }
    }

    pub fn translate_memory_op(&mut self, block_id: &BlockId, op: &MemOp) {
        self._register_intermediate_witnesses_for_memory_op(&op);

        let witness_index_to_access = op.index.to_witness().unwrap();
        let target_index_to_access =
            self._get_or_create_target_for_witness(witness_index_to_access);
        MemoryOperationsTranslator::add_restrictions_to_assert_target_is_less_or_equal_to(
            self.memory_blocks.get(block_id).unwrap().1 - 1,
            target_index_to_access,
            &mut self.builder,
        );

        let is_memory_read = op.clone().operation.to_const().unwrap().is_zero();
        let is_memory_write = op.clone().operation.to_const().unwrap().is_one();
        if is_memory_read {
            self._translate_memory_read(block_id, op);
        } else if is_memory_write {
            self._translate_memory_write(&block_id, op);
        } else {
            panic!("Backend encountered unknown memory operation code (nor 0 or 1)");
        }
    }

    /// We use this algorithm to validate in-range access and restrain access to one of the padded
    /// positions in the target vector.
    pub fn add_restrictions_to_assert_target_is_less_or_equal_to(
        max_allowed_value: usize,
        target_index: Target,
        builder: &mut CB,
    ) {
        let binary_representation: Vec<u8> = format!("{:b}", max_allowed_value)
            .chars()
            .map(|c| c.to_digit(2).unwrap() as u8)
            .collect();

        let binary_target_index = BinaryDigitsTarget {
            bits: builder
                .split_le(target_index, binary_representation.len())
                .into_iter()
                .rev()
                .collect(),
        };

        let mut acc_target = builder.one();
        for i in 0..binary_target_index.number_of_digits() {
            if binary_representation[i] == 0 {
                let aux = builder.mul(binary_target_index.bits[i].target, acc_target);
                builder.assert_zero(aux);
            } else if binary_representation[i] == 1 {
                let new_acc_target = builder.mul(acc_target, binary_target_index.bits[i].target);
                acc_target = new_acc_target;
            }
        }
    }

    /// The problem is that we cannot know which target is going to be replaced in circuit-building
    /// time. The solution, replacing all the targets, connecting all the values except for the
    /// one modified. To know what position is being modified, we use the Plonky2 EqualGate.
    /// The key is that the circuit has to be symmetrical for all possible values.
    fn _translate_memory_write(&mut self, block_id: &BlockId, op: &MemOp) {
        let witness_idx_to_write = op.index.to_witness().unwrap();
        let target_idx_to_write = self._get_or_create_target_for_witness(witness_idx_to_write);
        let witness_holding_new_value = op.value.to_witness().unwrap();
        let target_holding_new_value =
            self._get_or_create_target_for_witness(witness_holding_new_value);

        let memory_block_length = (&self.memory_blocks[block_id].0).len();
        for position in 0..memory_block_length {
            let target_with_position = self.builder.constant(F::from_canonical_usize(position));
            let is_current_position_being_modified = self
                .builder
                .is_equal(target_idx_to_write, target_with_position);

            let current_target_in_position = self.memory_blocks[block_id].0[position];
            let new_target_in_array = self.builder._if(
                is_current_position_being_modified,
                target_holding_new_value,
                current_target_in_position,
            );

            self.memory_blocks.get_mut(block_id).unwrap().0[position] = new_target_in_array;
        }
    }

    /// For this Plonky2 uses the RandomAccessGate
    fn _translate_memory_read(&mut self, block_id: &BlockId, op: &MemOp) {
        let witness_idx_to_read = op.index.to_witness().unwrap();
        let target_idx_to_read = self._get_or_create_target_for_witness(witness_idx_to_read);
        let witness_to_save_result = op.value.to_witness().unwrap();
        let block_of_memory = self.memory_blocks[block_id].0.clone();
        let target_to_save_result = self
            .builder
            .random_access(target_idx_to_read, block_of_memory);
        self.witness_target_map
            .insert(witness_to_save_result, target_to_save_result);
    }

    /// Creates a new block of memory with the associated id
    pub fn translate_memory_init(&mut self, init: &Vec<Witness>, block_id: &BlockId) {
        let mut vector_targets: Vec<Target> = init
            .into_iter()
            .map(|w| self._get_or_create_target_for_witness(*w))
            .collect();
        let real_memory_block_size = vector_targets.len();
        self._extend_block_with_zeroes_to_have_a_power_of_two_length(&mut vector_targets);
        self.memory_blocks
            .insert(*block_id, (vector_targets, real_memory_block_size));
    }

    /// This is necessary because plonky2 can only perform a random_access operation
    /// on vectors with a length that is a power of two.
    fn _extend_block_with_zeroes_to_have_a_power_of_two_length(
        &mut self,
        vector_targets: &mut Vec<Target>,
    ) {
        let length_of_block = vector_targets.len();
        let targets_to_add = (length_of_block as u32)
            .checked_next_power_of_two()
            .unwrap_or(0)
            - length_of_block as u32;
        vector_targets.extend((0..targets_to_add).into_iter().map(|_| self.builder.zero()));
    }

    fn _register_intermediate_witnesses_for_memory_op(self: &mut Self, op: &MemOp) {
        let at = &op.index.linear_combinations[0].1;
        self._get_or_create_target_for_witness(*at);

        let value = &op.value.linear_combinations[0].1;
        self._get_or_create_target_for_witness(*value);
    }

    fn _get_or_create_target_for_witness(&mut self, witness: Witness) -> Target {
        match self.witness_target_map.get(&witness) {
            Some(target) => *target,
            None => {
                let target = self.builder.add_virtual_target();
                self.witness_target_map.insert(witness, target);
                target
            }
        }
    }
}
