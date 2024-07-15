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
        builder: &mut CB,
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
            builder.connect(binary_target.bits[i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }
        BinaryDigitsTarget { bits: new_bits }
    }

    pub fn shift_right(
        target: &BinaryDigitsTarget,
        times: usize,
        builder: &mut CB,
    ) -> BinaryDigitsTarget {
        let mut new_bits = Vec::new();
        // Fill zero bits
        for _ in 0..times {
            new_bits.push(BoolTarget::new_unsafe(
                builder.constant(F::from_canonical_u8(0)),
            ));
        }

        for i in times..target.number_of_digits() {
            let new_bool_target = builder.add_virtual_bool_target_safe();
            builder.connect(target.bits[i - times].target, new_bool_target.target);
            new_bits.push(new_bool_target);
        }
        BinaryDigitsTarget { bits: new_bits }
    }

    pub fn choose(
        chooser: &BinaryDigitsTarget,
        on_true: &BinaryDigitsTarget,
        on_false: &BinaryDigitsTarget,
        builder: &mut CB,
    ) -> BinaryDigitsTarget {
        let bit_pairs_iter = on_true.bits.iter().zip(on_false.bits.iter());

        let chosen_bits = chooser
            .bits
            .iter()
            .zip(bit_pairs_iter)
            .map(|(c, (t, f))| BinaryDigitsTarget::select_bool_target(c, t, f, builder))
            .collect();

        BinaryDigitsTarget { bits: chosen_bits }
    }

    pub fn majority(
        a: &BinaryDigitsTarget,
        b: &BinaryDigitsTarget,
        c: &BinaryDigitsTarget,
        builder: &mut CB,
    ) -> BinaryDigitsTarget {
        let bit_pairs_iter = a.bits.iter().zip(b.bits.iter());

        let majority_bits = c
            .bits
            .iter()
            .zip(bit_pairs_iter)
            .map(|(b0, (b1, b2))| {
                let on_true = BinaryDigitsTarget::bit_or(*b1, *b2, builder);
                let on_false = BinaryDigitsTarget::bit_and(*b1, *b2, builder);
                BinaryDigitsTarget::select_bool_target(b0, &on_true, &on_false, builder)
            })
            .collect();
        BinaryDigitsTarget {
            bits: majority_bits,
        }
    }

    pub fn select_bool_target(
        chooser: &BoolTarget,
        on_true: &BoolTarget,
        on_false: &BoolTarget,
        builder: &mut CB,
    ) -> BoolTarget {
        let target = builder.select(*chooser, on_true.target, on_false.target);
        BoolTarget::new_unsafe(target)
    }

    pub fn xor(
        b1: BinaryDigitsTarget,
        b2: BinaryDigitsTarget,
        builder: &mut CB,
    ) -> BinaryDigitsTarget {
        BinaryDigitsTarget::apply_bitwise_to_binary_digits_target(
            b1,
            b2,
            builder,
            BinaryDigitsTarget::bit_xor,
        )
    }

    pub fn and(
        b1: BinaryDigitsTarget,
        b2: BinaryDigitsTarget,
        builder: &mut CB,
    ) -> BinaryDigitsTarget {
        BinaryDigitsTarget::apply_bitwise_to_binary_digits_target(
            b1,
            b2,
            builder,
            BinaryDigitsTarget::bit_and,
        )
    }

    pub fn apply_bitwise_to_binary_digits_target(
        b1: BinaryDigitsTarget,
        b2: BinaryDigitsTarget,
        builder: &mut CB,
        op: fn(BoolTarget, BoolTarget, &mut CB) -> BoolTarget,
    ) -> BinaryDigitsTarget {
        BinaryDigitsTarget {
            bits: BinaryDigitsTarget::apply_bitwise_and_output_bool_targets(&b1, &b2, builder, op),
        }
    }

    pub fn apply_bitwise_and_output_bool_targets(
        b1: &BinaryDigitsTarget,
        b2: &BinaryDigitsTarget,
        builder: &mut CB,
        op: fn(BoolTarget, BoolTarget, &mut CB) -> BoolTarget,
    ) -> Vec<BoolTarget> {
        b1.bits
            .iter()
            .zip(b2.bits.iter())
            .map(|(bit1, bit2)| op(*bit1, *bit2, builder))
            .collect()
    }

    pub fn bit_and(b1: BoolTarget, b2: BoolTarget, builder: &mut CB) -> BoolTarget {
        builder.and(b1, b2)
    }

    pub fn bit_or(b1: BoolTarget, b2: BoolTarget, builder: &mut CB) -> BoolTarget {
        builder.or(b1, b2)
    }

    pub fn bit_xor(b1: BoolTarget, b2: BoolTarget, builder: &mut CB) -> BoolTarget {
        // a xor b = (a or b) and (not (a and b))
        let b1_or_b2 = builder.or(b1, b2);
        let b1_and_b2 = builder.and(b1, b2);
        let not_b1_and_b2 = builder.not(b1_and_b2);
        builder.and(b1_or_b2, not_b1_and_b2)
    }

    pub fn add_module_32_bits(
        b1: &BinaryDigitsTarget,
        b2: &BinaryDigitsTarget,
        builder: &mut CB,
    ) -> BinaryDigitsTarget {
        let partial_sum = BinaryDigitsTarget::apply_bitwise_and_output_bool_targets(
            &b1,
            &b2,
            builder,
            BinaryDigitsTarget::bit_xor,
        );
        let partial_carries = BinaryDigitsTarget::apply_bitwise_and_output_bool_targets(
            &b1,
            &b2,
            builder,
            BinaryDigitsTarget::bit_and,
        );

        let mut carry_in = builder._false();

        let sum = (0..b1.number_of_digits())
            .map(|idx_bit| {
                let sum_with_carry_in =
                    BinaryDigitsTarget::bit_xor(partial_sum[idx_bit], carry_in, builder);
                let carry_out =
                    BinaryDigitsTarget::bit_or(partial_carries[idx_bit], carry_in, builder);

                carry_in = carry_out; // The new carry_in is the current carry_out
                sum_with_carry_in
            })
            .collect();

        BinaryDigitsTarget { bits: sum }
    }
}
