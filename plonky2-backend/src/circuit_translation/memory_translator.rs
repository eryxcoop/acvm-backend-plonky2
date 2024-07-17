use super::*;

pub struct MemoryOperationsTranslator<'a> {
    builder: &'a mut CircuitBuilder<F, D>,
    witness_target_map: &'a mut HashMap<Witness, Target>,
    memory_blocks: &'a mut HashMap<BlockId, Vec<Target>>,
}

impl<'a> MemoryOperationsTranslator<'a> {
    pub fn new_for(
        builder: &'a mut CircuitBuilder<F, D>,
        witness_target_map: &'a mut HashMap<Witness, Target>,
        memory_blocks: &'a mut HashMap<BlockId, Vec<Target>>,
    ) -> Self {
        Self {
            builder,
            witness_target_map,
            memory_blocks,
        }
    }

    pub fn translate_memory_op(&mut self, block_id: &BlockId, op: &MemOp) {
        self._register_intermediate_witnesses_for_memory_op(&op);
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

    fn _translate_memory_write(&mut self, block_id: &&BlockId, op: &MemOp) {
        let witness_idx_to_write = op.index.to_witness().unwrap();
        let target_idx_to_write = self._get_or_create_target_for_witness(witness_idx_to_write);
        let witness_holding_new_value = op.value.to_witness().unwrap();
        let target_holding_new_value =
            self._get_or_create_target_for_witness(witness_holding_new_value);

        let memory_block_length = (&self.memory_blocks[block_id]).len();
        for position in 0..memory_block_length {
            let target_with_position = self.builder.constant(F::from_canonical_usize(position));
            let is_current_position_being_modified = self
                .builder
                .is_equal(target_idx_to_write, target_with_position);

            let current_target_in_position = self.memory_blocks[block_id][position];
            let new_target_in_array = self.builder._if(
                is_current_position_being_modified,
                target_holding_new_value,
                current_target_in_position,
            );

            self.memory_blocks.get_mut(block_id).unwrap()[position] = new_target_in_array;
        }
    }

    fn _translate_memory_read(&mut self, block_id: &BlockId, op: &MemOp) {
        let witness_idx_to_read = op.index.to_witness().unwrap();
        let target_idx_to_read = self._get_or_create_target_for_witness(witness_idx_to_read);
        let witness_to_save_result = op.value.to_witness().unwrap();
        let block_of_memory = self.memory_blocks[block_id].clone();
        let target_to_save_result = self
            .builder
            .random_access(target_idx_to_read, block_of_memory);
        self.witness_target_map
            .insert(witness_to_save_result, target_to_save_result);
    }

    pub fn translate_memory_init(&mut self, init: &Vec<Witness>, block_id: &BlockId) {
        let mut vector_targets: Vec<Target> = init
            .into_iter()
            .map(|w| self._get_or_create_target_for_witness(*w))
            .collect();
        self._extend_block_with_zeroes_to_have_a_power_of_two_length(&mut vector_targets);
        self.memory_blocks.insert(*block_id, vector_targets);
    }

    /// This is necessary because plonky2 can only perform a random_access operation
    /// on vectors with a length that is a power of two.
    /// "Index out of bounds" accesses shouldn't happen because Noir won't allow them.
    fn _extend_block_with_zeroes_to_have_a_power_of_two_length(&mut self, vector_targets: &mut Vec<Target>) {
        let length_of_block = vector_targets.len();
        let targets_to_add = (length_of_block as u32).checked_next_power_of_two().unwrap_or(0) - length_of_block as u32;
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
