use plonky2::field::types::Field;
use plonky2::iop::target::BoolTarget;
use crate::circuit_translation::{CB, F};

#[derive(Clone, Debug)]
pub struct BinaryDigitsTarget {
    pub bits: Vec<BoolTarget>,
}

impl BinaryDigitsTarget {
    pub(crate) fn number_of_digits(&self) -> usize {
        self.bits.len()
    }

    pub fn rotate_right(
        binary_target: &BinaryDigitsTarget,
        times: usize,
        builder: &mut CB
    ) -> BinaryDigitsTarget {
        let mut new_bits = Vec::new();
        // Wrap bits around
        for i in 0..times {
            let new_bool_target = builder.add_virtual_bool_target_safe();
            builder.connect(
                binary_target.bits[binary_target.number_of_digits() + i - times].target,
                new_bool_target.target,
            );
            new_bits.push(new_bool_target);
        }

        for i in times..binary_target.number_of_digits() {
            let new_bool_target = builder.add_virtual_bool_target_safe();
            builder
                .connect(binary_target.bits[i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }
        BinaryDigitsTarget { bits: new_bits }
    }

    pub fn shift_right(target: &BinaryDigitsTarget, times: usize, builder: &mut CB) -> BinaryDigitsTarget {
        let mut new_bits = Vec::new();
        // Fill zero bits
        for _ in 0..times {
            new_bits.push(BoolTarget::new_unsafe(
                builder.constant(F::from_canonical_u8(0)),
            ));
        }

        for i in times..target.number_of_digits() {
            let new_bool_target = builder.add_virtual_bool_target_safe();
            builder
                .connect(target.bits[i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }
        BinaryDigitsTarget { bits: new_bits }
    }
}
