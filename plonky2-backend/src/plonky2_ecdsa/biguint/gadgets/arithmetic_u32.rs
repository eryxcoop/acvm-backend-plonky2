use core::marker::PhantomData;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use crate::binary_digits_target::BinaryDigitsTarget;
use crate::plonky2_ecdsa::biguint::gates::add_many_u32::U32AddManyGate;
use crate::plonky2_ecdsa::biguint::gates::arithmetic_u32::U32ArithmeticGate;
use crate::plonky2_ecdsa::biguint::gates::subtraction_u32::U32SubtractionGate;
use crate::plonky2_ecdsa::biguint::serialization::{ReadU32, WriteU32};
use crate::plonky2_ecdsa::biguint::witness::GeneratedValuesU32;

#[derive(Clone, Copy, Debug)]
pub struct U32Target(pub Target);

pub trait CircuitBuilderU32<F: RichField + Extendable<D>, const D: usize> {
    fn add_virtual_u32_target(&mut self) -> U32Target;

    fn add_virtual_u32_targets(&mut self, n: usize) -> Vec<U32Target>;

    /// Returns a U32Target for the value `c`, which is assumed to be at most 32 bits.
    fn constant_u32(&mut self, c: u32) -> U32Target;

    fn zero_u32(&mut self) -> U32Target;

    fn one_u32(&mut self) -> U32Target;

    fn connect_u32(&mut self, x: U32Target, y: U32Target);

    fn assert_zero_u32(&mut self, x: U32Target);

    /// Checks for special cases where the value of
    /// `x * y + z`
    /// can be determined without adding a `U32ArithmeticGate`.
    fn arithmetic_u32_special_cases(
        &mut self,
        x: U32Target,
        y: U32Target,
        z: U32Target,
    ) -> Option<(U32Target, U32Target)>;

    // Returns x * y + z.
    fn mul_add_u32(&mut self, x: U32Target, y: U32Target, z: U32Target) -> (U32Target, U32Target);

    fn add_u32(&mut self, a: U32Target, b: U32Target) -> (U32Target, U32Target);

    fn add_many_u32(&mut self, to_add: &[U32Target]) -> (U32Target, U32Target);

    fn add_u32s_with_carry(
        &mut self,
        to_add: &[U32Target],
        carry: U32Target,
    ) -> (U32Target, U32Target);

    fn mul_u32(&mut self, a: U32Target, b: U32Target) -> (U32Target, U32Target);

    // Returns x - y - borrow, as a pair (result, borrow), where borrow is 0 or 1 depending on whether borrowing from the next digit is required (iff y + borrow > x).
    fn sub_u32(&mut self, x: U32Target, y: U32Target, borrow: U32Target) -> (U32Target, U32Target);

    fn split_into_bool_targets(&mut self, a: U32Target) -> [BoolTarget; 32];

    fn split_into_byte_targets(&mut self, a: U32Target) -> [BinaryDigitsTarget; 4];

    fn constant_byte(&mut self, byte: u8) -> BinaryDigitsTarget;

    fn connect_byte(&mut self, x: BinaryDigitsTarget, y: BinaryDigitsTarget);

    fn connect_bit(&mut self, x: BoolTarget, y: BoolTarget);
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderU32<F, D>
    for CircuitBuilder<F, D>
{
    fn add_virtual_u32_target(&mut self) -> U32Target {
        U32Target(self.add_virtual_target())
    }

    fn add_virtual_u32_targets(&mut self, n: usize) -> Vec<U32Target> {
        self.add_virtual_targets(n)
            .into_iter()
            .map(U32Target)
            .collect()
    }

    /// Returns a U32Target for the value `c`, which is assumed to be at most 32 bits.
    fn constant_u32(&mut self, c: u32) -> U32Target {
        U32Target(self.constant(F::from_canonical_u32(c)))
    }

    fn zero_u32(&mut self) -> U32Target {
        U32Target(self.zero())
    }

    fn one_u32(&mut self) -> U32Target {
        U32Target(self.one())
    }

    fn connect_u32(&mut self, x: U32Target, y: U32Target) {
        self.connect(x.0, y.0)
    }

    fn assert_zero_u32(&mut self, x: U32Target) {
        self.assert_zero(x.0)
    }

    /// Checks for special cases where the value of
    /// `x * y + z`
    /// can be determined without adding a `U32ArithmeticGate`.
    fn arithmetic_u32_special_cases(
        &mut self,
        x: U32Target,
        y: U32Target,
        z: U32Target,
    ) -> Option<(U32Target, U32Target)> {
        let x_const = self.target_as_constant(x.0);
        let y_const = self.target_as_constant(y.0);
        let z_const = self.target_as_constant(z.0);

        // If both terms are constant, return their (constant) sum.
        let first_term_const = if let (Some(xx), Some(yy)) = (x_const, y_const) {
            Some(xx * yy)
        } else {
            None
        };

        if let (Some(a), Some(b)) = (first_term_const, z_const) {
            let sum = (a + b).to_canonical_u64();
            let (low, high) = (sum as u32, (sum >> 32) as u32);
            return Some((self.constant_u32(low), self.constant_u32(high)));
        }

        None
    }

    // Returns x * y + z.
    fn mul_add_u32(&mut self, x: U32Target, y: U32Target, z: U32Target) -> (U32Target, U32Target) {
        if let Some(result) = self.arithmetic_u32_special_cases(x, y, z) {
            return result;
        }

        let gate = U32ArithmeticGate::<F, D>::new_from_config(&self.config);
        let (row, copy) = self.find_slot(gate, &[], &[]);

        self.connect(Target::wire(row, gate.wire_ith_multiplicand_0(copy)), x.0);
        self.connect(Target::wire(row, gate.wire_ith_multiplicand_1(copy)), y.0);
        self.connect(Target::wire(row, gate.wire_ith_addend(copy)), z.0);

        let output_low = U32Target(Target::wire(row, gate.wire_ith_output_low_half(copy)));
        let output_high = U32Target(Target::wire(row, gate.wire_ith_output_high_half(copy)));

        (output_low, output_high)
    }

    fn add_u32(&mut self, a: U32Target, b: U32Target) -> (U32Target, U32Target) {
        let one = self.one_u32();
        self.mul_add_u32(a, one, b)
    }

    fn add_many_u32(&mut self, to_add: &[U32Target]) -> (U32Target, U32Target) {
        match to_add.len() {
            0 => (self.zero_u32(), self.zero_u32()),
            1 => (to_add[0], self.zero_u32()),
            2 => self.add_u32(to_add[0], to_add[1]),
            _ => {
                let num_addends = to_add.len();
                let gate = U32AddManyGate::<F, D>::new_from_config(&self.config, num_addends);
                let (row, copy) =
                    self.find_slot(gate, &[F::from_canonical_usize(num_addends)], &[]);

                for j in 0..num_addends {
                    self.connect(
                        Target::wire(row, gate.wire_ith_op_jth_addend(copy, j)),
                        to_add[j].0,
                    );
                }
                let zero = self.zero();
                self.connect(Target::wire(row, gate.wire_ith_carry(copy)), zero);

                let output_low = U32Target(Target::wire(row, gate.wire_ith_output_result(copy)));
                let output_high = U32Target(Target::wire(row, gate.wire_ith_output_carry(copy)));

                (output_low, output_high)
            }
        }
    }

    fn add_u32s_with_carry(
        &mut self,
        to_add: &[U32Target],
        carry: U32Target,
    ) -> (U32Target, U32Target) {
        if to_add.len() == 1 {
            return self.add_u32(to_add[0], carry);
        }

        let num_addends = to_add.len();

        let gate = U32AddManyGate::<F, D>::new_from_config(&self.config, num_addends);
        let (row, copy) = self.find_slot(gate, &[F::from_canonical_usize(num_addends)], &[]);

        for j in 0..num_addends {
            self.connect(
                Target::wire(row, gate.wire_ith_op_jth_addend(copy, j)),
                to_add[j].0,
            );
        }
        self.connect(Target::wire(row, gate.wire_ith_carry(copy)), carry.0);

        let output = U32Target(Target::wire(row, gate.wire_ith_output_result(copy)));
        let output_carry = U32Target(Target::wire(row, gate.wire_ith_output_carry(copy)));

        (output, output_carry)
    }

    fn mul_u32(&mut self, a: U32Target, b: U32Target) -> (U32Target, U32Target) {
        let zero = self.zero_u32();
        self.mul_add_u32(a, b, zero)
    }

    // Returns x - y - borrow, as a pair (result, borrow), where borrow is 0 or 1 depending on whether borrowing from the next digit is required (iff y + borrow > x).
    fn sub_u32(&mut self, x: U32Target, y: U32Target, borrow: U32Target) -> (U32Target, U32Target) {
        let gate = U32SubtractionGate::<F, D>::new_from_config(&self.config);
        let (row, copy) = self.find_slot(gate, &[], &[]);

        self.connect(Target::wire(row, gate.wire_ith_input_x(copy)), x.0);
        self.connect(Target::wire(row, gate.wire_ith_input_y(copy)), y.0);
        self.connect(
            Target::wire(row, gate.wire_ith_input_borrow(copy)),
            borrow.0,
        );

        let output_result = U32Target(Target::wire(row, gate.wire_ith_output_result(copy)));
        let output_borrow = U32Target(Target::wire(row, gate.wire_ith_output_borrow(copy)));

        (output_result, output_borrow)
    }

    fn split_into_bool_targets(&mut self, a: U32Target) -> [BoolTarget; 32] {
        let bool_targets: [BoolTarget; 32] = std::array::from_fn(|_|
            self.add_virtual_bool_target_safe()
        );
        let mut acumulator = self.constant(F::ZERO);
        for i in 0..32 {
            acumulator = self.mul_const_add(
                F::from_canonical_u32(2).exp_u64(32 - i - 1),
                bool_targets[i as usize].target,
                acumulator,
            );
        }
        self.connect(a.0, acumulator);
        bool_targets
    }

    fn split_into_byte_targets(&mut self, a: U32Target) -> [BinaryDigitsTarget; 4] {
        let bool_targets = self.split_into_bool_targets(a);
        [
            BinaryDigitsTarget {
                bits: bool_targets[0..8].to_vec(),
            },
            BinaryDigitsTarget {
                bits: bool_targets[8..16].to_vec(),
            },
            BinaryDigitsTarget {
                bits: bool_targets[16..24].to_vec(),
            },
            BinaryDigitsTarget {
                bits: bool_targets[24..32].to_vec(),
            },
        ]
    }

    fn constant_byte(&mut self, byte: u8) -> BinaryDigitsTarget {
        BinaryDigitsTarget {
            bits: (0u8..8u8)
                .rev()
                .map(|i| {
                    let value = ((1u8 << i) & byte) >> i;
                    println!("Index {} value {}", i, value);
                    BoolTarget::new_unsafe(
                        self.constant(F::from_canonical_u8(value)),
                    )
                })
                .collect(),
        }
    }

    fn connect_byte(&mut self, x: BinaryDigitsTarget, y: BinaryDigitsTarget) {
        for (a, b) in x.bits.iter().zip(y.bits.iter()) {
            self.connect_bit(*a, *b);
        }
    }

    fn connect_bit(&mut self, x: BoolTarget, y: BoolTarget) {
        self.connect(x.target, y.target);
    }
}

#[derive(Debug)]
struct SplitToU32Generator<F: RichField + Extendable<D>, const D: usize> {
    x: Target,
    low: U32Target,
    high: U32Target,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for SplitToU32Generator<F, D>
{
    fn id(&self) -> String {
        "SplitToU32Generator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        vec![self.x]
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let x = witness.get_target(self.x);
        let x_u64 = x.to_canonical_u64();
        let low = x_u64 as u32;
        let high = (x_u64 >> 32) as u32;

        out_buffer.set_u32_target(self.low, low);
        out_buffer.set_u32_target(self.high, high);
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_target(self.x)?;
        dst.write_target_u32(self.low)?;
        dst.write_target_u32(self.high)
    }

    fn deserialize(src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let x = src.read_target()?;
        let low = src.read_target_u32()?;
        let high = src.read_target_u32()?;
        Ok(Self {
            x,
            low,
            high,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use rand::rngs::OsRng;
    use rand::Rng;

    use super::*;

    #[test]
    pub fn test_add_many_u32s() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        const NUM_ADDENDS: usize = 15;

        let config = CircuitConfig::standard_recursion_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let mut rng = OsRng;
        let mut to_add = Vec::new();
        let mut sum = 0u64;
        for _ in 0..NUM_ADDENDS {
            let x: u32 = rng.gen();
            sum += x as u64;
            to_add.push(builder.constant_u32(x));
        }
        let carry = builder.zero_u32();
        let (result_low, result_high) = builder.add_u32s_with_carry(&to_add, carry);
        let expected_low = builder.constant_u32((sum % (1 << 32)) as u32);
        let expected_high = builder.constant_u32((sum >> 32) as u32);

        builder.connect_u32(result_low, expected_low);
        builder.connect_u32(result_high, expected_high);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof)
    }

    #[test]
    pub fn test_split_u32_into_target_bytes() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_recursion_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = builder.constant_u32(1071991);

        let [v1, v2, v3, v4] = builder.split_into_byte_targets(x);
        let e1 = builder.constant_byte(0b00000000);
        builder.connect_byte(v1, e1);
        let e2 = builder.constant_byte(0b00010000);
        builder.connect_byte(v2, e2);
        let e3 = builder.constant_byte(0b01011011);
        builder.connect_byte(v3, e3);
        let e4 = builder.constant_byte(0b01110111);
        builder.connect_byte(v4, e4);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof)
    }
}
