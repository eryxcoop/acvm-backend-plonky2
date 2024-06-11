use plonky2::iop::target::BoolTarget;

#[derive(Clone, Debug)]
pub struct BinaryDigitsTarget {
    pub bits: Vec<BoolTarget>,
}

impl BinaryDigitsTarget {
    fn number_of_digits(&self) -> usize {
        self.bits.len()
    }

    fn shift_right(&mut self, times: usize) -> Self {
        let mut new_bits = Vec::new();
        // Fill zero bits
        for _ in 0..times {
            new_bits.push(BoolTarget::new_unsafe(
                self.constant(F::from_canonical_u8(0)),
            ));
        }

        for i in times..8 {
            let new_bool_target = self.add_virtual_bool_target_safe();
            self.connect(self.bits[i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }
        Self { bits: new_bits }
    }

    fn rotate_right(&mut self, times: usize) -> Self {
        let mut new_bits = Vec::new();
        // Wrap bits around
        for i in 0..times {
            let new_bool_target = self.add_virtual_bool_target_safe();
            self.connect(self.bits[self.number_of_digits() + i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }

        for i in times..8 {
            let new_bool_target = self.add_virtual_bool_target_safe();
            self.connect(self.bits[i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }
        Self { bits: new_bits }
    }
}
