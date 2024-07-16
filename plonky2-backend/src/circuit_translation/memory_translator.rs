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
        Self { builder, witness_target_map, memory_blocks }
    }

    pub fn translate_memory_init(
        &mut self,
        init: &Vec<Witness>,
        block_id: &BlockId,
    ) {
        let vector_targets = init
            .into_iter()
            .map(|w| self._get_or_create_target_for_witness(*w))
            .collect();
        self.memory_blocks.insert(*block_id, vector_targets);
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
