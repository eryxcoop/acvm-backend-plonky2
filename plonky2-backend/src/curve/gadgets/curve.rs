use num::traits::FromBytes;
use plonky2::field::secp256k1_scalar::Secp256K1Scalar;
use plonky2::field::types::{Field, PrimeField, PrimeField64, Sample};
use plonky2::field::{extension::Extendable, secp256k1_base::Secp256K1Base};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::witness::Witness;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use serde::{Deserialize, Serialize};

use crate::biguint::gadgets::nonnative::{
    CircuitBuilderNonNative, NonNativeTarget, WitnessNonNative,
};

const SECP256K1_A: Secp256K1Base = Secp256K1Base::ZERO;
const SECP256K1_B: Secp256K1Base = Secp256K1Base([7, 0, 0, 0]);

// 55066263022277343669578718895168534326250603453777594175500187360389116729240
pub const SECP256K1_GENERATOR_X: Secp256K1Base = Secp256K1Base([
    0x59F2815B16F81798,
    0x029BFCDB2DCE28D9,
    0x55A06295CE870B07,
    0x79BE667EF9DCBBAC,
]);

/// 32670510020758816978083085130507043184471273380659243275938904335757337482424
pub const SECP256K1_GENERATOR_Y: Secp256K1Base = Secp256K1Base([
    0x9C47D08FFB10D4B8,
    0xFD17B448A6855419,
    0x5DA4FBFC0E1108A8,
    0x483ADA7726A3C465,
]);

pub fn generate_random_point<F: RichField + Extendable<D>, const D: usize>(
    _builder: &mut CircuitBuilder<F, D>,
) -> (Secp256K1Base, Secp256K1Base) {
    loop {
        let x = Secp256K1Base::rand();
        let candidate = x * x * x + Secp256K1Base::from_canonical_u8(7);
        if let Some(y) = candidate.sqrt() {
            // return builder.constant_affine_point(x, y);
            return (x, y);
        }
    }
}

/// A Target representing an affine point on the curve `C`. We use incomplete arithmetic for efficiency,
/// so we assume these points are not zero.
#[derive(Clone, Debug)]
pub struct AffinePointTarget {
    pub x: NonNativeTarget<Secp256K1Base>,
    pub y: NonNativeTarget<Secp256K1Base>,
}

impl AffinePointTarget {
    pub fn to_vec(&self) -> Vec<NonNativeTarget<Secp256K1Base>> {
        vec![self.x.clone(), self.y.clone()]
    }
}

pub trait CircuitBuilderCurve<F: RichField + Extendable<D>, const D: usize> {
    fn constant_affine_point(&mut self, x: Secp256K1Base, y: Secp256K1Base) -> AffinePointTarget;

    fn connect_affine_point(&mut self, lhs: &AffinePointTarget, rhs: &AffinePointTarget);

    fn add_virtual_affine_point_target(&mut self) -> AffinePointTarget;

    fn curve_assert_valid(&mut self, p: &AffinePointTarget);

    fn curve_neg(&mut self, p: &AffinePointTarget) -> AffinePointTarget;

    fn curve_conditional_neg(&mut self, p: &AffinePointTarget, b: BoolTarget) -> AffinePointTarget;

    fn curve_double(&mut self, p: &AffinePointTarget) -> AffinePointTarget;

    fn curve_repeated_double(&mut self, p: &AffinePointTarget, n: usize) -> AffinePointTarget;

    /// Add two points, which are assumed to be non-equal.
    fn curve_add(&mut self, p1: &AffinePointTarget, p2: &AffinePointTarget) -> AffinePointTarget;

    fn curve_conditional_add(
        &mut self,
        p1: &AffinePointTarget,
        p2: &AffinePointTarget,
        b: BoolTarget,
    ) -> AffinePointTarget;

    fn curve_scalar_mul(
        &mut self,
        p: &AffinePointTarget,
        n: &NonNativeTarget<Secp256K1Scalar>,
    ) -> AffinePointTarget;

    fn curve_generator_constant(&mut self) -> AffinePointTarget;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderCurve<F, D>
    for CircuitBuilder<F, D>
{
    fn constant_affine_point(&mut self, x: Secp256K1Base, y: Secp256K1Base) -> AffinePointTarget {
        debug_assert!(!x.is_zero() && !y.is_zero());
        AffinePointTarget {
            x: self.constant_nonnative(x),
            y: self.constant_nonnative(y),
        }
    }

    fn connect_affine_point(&mut self, lhs: &AffinePointTarget, rhs: &AffinePointTarget) {
        self.connect_nonnative(&lhs.x, &rhs.x);
        self.connect_nonnative(&lhs.y, &rhs.y);
    }

    fn add_virtual_affine_point_target(&mut self) -> AffinePointTarget {
        let x = self.add_virtual_nonnative_target();
        let y = self.add_virtual_nonnative_target();

        AffinePointTarget { x, y }
    }

    fn curve_assert_valid(&mut self, p: &AffinePointTarget) {
        let a = self.constant_nonnative(SECP256K1_A);
        let b = self.constant_nonnative(SECP256K1_B);

        let y_squared = self.mul_nonnative(&p.y, &p.y);
        let x_squared = self.mul_nonnative(&p.x, &p.x);
        let x_cubed = self.mul_nonnative(&x_squared, &p.x);
        let a_x = self.mul_nonnative(&a, &p.x);
        let a_x_plus_b = self.add_nonnative(&a_x, &b);
        let rhs = self.add_nonnative(&x_cubed, &a_x_plus_b);

        self.connect_nonnative(&y_squared, &rhs);
    }

    fn curve_neg(&mut self, p: &AffinePointTarget) -> AffinePointTarget {
        let neg_y = self.neg_nonnative(&p.y);
        AffinePointTarget {
            x: p.x.clone(),
            y: neg_y,
        }
    }

    fn curve_conditional_neg(&mut self, p: &AffinePointTarget, b: BoolTarget) -> AffinePointTarget {
        AffinePointTarget {
            x: p.x.clone(),
            y: self.nonnative_conditional_neg(&p.y, b),
        }
    }

    fn curve_double(&mut self, p: &AffinePointTarget) -> AffinePointTarget {
        let AffinePointTarget { x, y } = p;
        let double_y = self.add_nonnative(y, y);
        let inv_double_y = self.inv_nonnative(&double_y);
        let x_squared = self.mul_nonnative(x, x);
        let double_x_squared = self.add_nonnative(&x_squared, &x_squared);
        let triple_x_squared = self.add_nonnative(&double_x_squared, &x_squared);

        let a = self.constant_nonnative(SECP256K1_A);
        let triple_xx_a = self.add_nonnative(&triple_x_squared, &a);
        let lambda = self.mul_nonnative(&triple_xx_a, &inv_double_y);
        let lambda_squared = self.mul_nonnative(&lambda, &lambda);
        let x_double = self.add_nonnative(x, x);

        let x3 = self.sub_nonnative(&lambda_squared, &x_double);

        let x_diff = self.sub_nonnative(x, &x3);
        let lambda_x_diff = self.mul_nonnative(&lambda, &x_diff);

        let y3 = self.sub_nonnative(&lambda_x_diff, y);

        AffinePointTarget { x: x3, y: y3 }
    }

    fn curve_repeated_double(&mut self, p: &AffinePointTarget, n: usize) -> AffinePointTarget {
        let mut result = p.clone();

        for _ in 0..n {
            result = self.curve_double(&result);
        }

        result
    }

    fn curve_add(&mut self, p1: &AffinePointTarget, p2: &AffinePointTarget) -> AffinePointTarget {
        let AffinePointTarget { x: x1, y: y1 } = p1;
        let AffinePointTarget { x: x2, y: y2 } = p2;

        let u = self.sub_nonnative(y2, y1);
        let v = self.sub_nonnative(x2, x1);
        let v_inv = self.inv_nonnative(&v);
        let s = self.mul_nonnative(&u, &v_inv);
        let s_squared = self.mul_nonnative(&s, &s);
        let x_sum = self.add_nonnative(x2, x1);
        let x3 = self.sub_nonnative(&s_squared, &x_sum);
        let x_diff = self.sub_nonnative(x1, &x3);
        let prod = self.mul_nonnative(&s, &x_diff);
        let y3 = self.sub_nonnative(&prod, y1);

        AffinePointTarget { x: x3, y: y3 }
    }

    fn curve_conditional_add(
        &mut self,
        p1: &AffinePointTarget,
        p2: &AffinePointTarget,
        b: BoolTarget,
    ) -> AffinePointTarget {
        let not_b = self.not(b);
        let sum = self.curve_add(p1, p2);
        let x_if_true = self.mul_nonnative_by_bool(&sum.x, b);
        let y_if_true = self.mul_nonnative_by_bool(&sum.y, b);
        let x_if_false = self.mul_nonnative_by_bool(&p1.x, not_b);
        let y_if_false = self.mul_nonnative_by_bool(&p1.y, not_b);

        let x = self.add_nonnative(&x_if_true, &x_if_false);
        let y = self.add_nonnative(&y_if_true, &y_if_false);

        AffinePointTarget { x, y }
    }

    fn curve_scalar_mul(
        &mut self,
        p: &AffinePointTarget,
        n: &NonNativeTarget<Secp256K1Scalar>,
    ) -> AffinePointTarget {
        let bits = self.split_nonnative_to_bits(n);

        // TODO: CHECK THIS
        let (x, y) = generate_random_point(self);
        let randot = self.constant_affine_point(x, y);
        // Result starts at `rando`, which is later subtracted, because we don't support arithmetic with the zero point.
        let mut result = self.add_virtual_affine_point_target();
        self.connect_affine_point(&randot, &result);

        let mut two_i_times_p = self.add_virtual_affine_point_target();
        self.connect_affine_point(p, &two_i_times_p);

        for &bit in bits.iter() {
            let not_bit = self.not(bit);

            let result_plus_2_i_p = self.curve_add(&result, &two_i_times_p);

            let new_x_if_bit = self.mul_nonnative_by_bool(&result_plus_2_i_p.x, bit);
            let new_x_if_not_bit = self.mul_nonnative_by_bool(&result.x, not_bit);
            let new_y_if_bit = self.mul_nonnative_by_bool(&result_plus_2_i_p.y, bit);
            let new_y_if_not_bit = self.mul_nonnative_by_bool(&result.y, not_bit);

            let new_x = self.add_nonnative(&new_x_if_bit, &new_x_if_not_bit);
            let new_y = self.add_nonnative(&new_y_if_bit, &new_y_if_not_bit);

            result = AffinePointTarget { x: new_x, y: new_y };

            two_i_times_p = self.curve_double(&two_i_times_p);
        }

        // Subtract off result's intial value of `rando`.
        let neg_r = self.curve_neg(&randot);
        result = self.curve_add(&result, &neg_r);

        result
    }

    fn curve_generator_constant(&mut self) -> AffinePointTarget {
        self.constant_affine_point(SECP256K1_GENERATOR_X, SECP256K1_GENERATOR_Y)
    }
}

pub trait WitnessPoint<F: PrimeField64>: Witness<F> {
    fn set_affine_point_target(
        &mut self,
        target: &AffinePointTarget,
        x: &Secp256K1Base,
        y: &Secp256K1Base,
    );
}

impl<T: Witness<F>, F: PrimeField64> WitnessPoint<F> for T {
    fn set_affine_point_target(
        &mut self,
        target: &AffinePointTarget,
        x: &Secp256K1Base,
        y: &Secp256K1Base,
    ) {
        self.set_non_native_target(&target.x, x);
        self.set_non_native_target(&target.y, y);
    }
}

#[cfg(test)]
mod tests {
    use num::{traits::FromBytes, BigUint, FromPrimitive};
    use plonky2::{
        field::{
            secp256k1_base::Secp256K1Base,
            secp256k1_scalar::Secp256K1Scalar,
            types::{Field, Sample},
        },
        iop::witness::PartialWitness,
        plonk::{
            circuit_builder::CircuitBuilder,
            circuit_data::CircuitConfig,
            config::{GenericConfig, KeccakGoldilocksConfig},
        },
    };

    use crate::{
        biguint::{
            biguint::WitnessBigUint, gadgets::nonnative::CircuitBuilderNonNative,
            witness::WitnessU32,
        },
        curve::gadgets::{
            curve::{CircuitBuilderCurve, SECP256K1_GENERATOR_X, SECP256K1_GENERATOR_Y},
            glv::CircuitBuilderGlv,
        },
    };

    // use core::ops::Neg;
    //
    // use anyhow::Result;
    // use plonky2::field::secp256k1_base::Secp256K1Base;
    // use plonky2::field::secp256k1_scalar::Secp256K1Scalar;
    // use plonky2::field::types::{Field, Sample};
    // use plonky2::iop::witness::PartialWitness;
    // use plonky2::plonk::circuit_builder::CircuitBuilder;
    // use plonky2::plonk::circuit_data::CircuitConfig;
    // use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig, PoseidonGoldilocksConfig};
    //
    // use crate::curve::curve_types::{AffinePoint, Curve, CurveScalar};
    // use crate::curve::secp256k1::Secp256K1;
    // use crate::curve_gadget::CircuitBuilderCurve;
    // use crate::nonnative::CircuitBuilderNonNative;
    //
    // #[test]
    // fn test_curve_point_is_valid() -> Result<()> {
    //     const D: usize = 2;
    //     type C = PoseidonGoldilocksConfig;
    //     type F = <C as GenericConfig<D>>::F;
    //
    //     let config = CircuitConfig::standard_ecc_config();
    //
    //     let pw = PartialWitness::new();
    //     let mut builder = CircuitBuilder::<F, D>::new(config);
    //
    //     let g = Secp256K1::GENERATOR_AFFINE;
    //     let g_target = builder.constant_affine_point(g);
    //     let neg_g_target = builder.curve_neg(&g_target);
    //
    //     builder.curve_assert_valid(&g_target);
    //     builder.curve_assert_valid(&neg_g_target);
    //
    //     let data = builder.build::();
    //     let proof = data.prove(pw).unwrap();
    //
    //     data.verify(proof)
    // }
    //
    // #[test]
    // #[should_panic]
    // fn test_curve_point_is_not_valid() {
    //     const D: usize = 2;
    //     type C = PoseidonGoldilocksConfig;
    //     type F = <C as GenericConfig<D>>::F;
    //
    //     let config = CircuitConfig::standard_ecc_config();
    //
    //     let pw = PartialWitness::new();
    //     let mut builder = CircuitBuilder::<F, D>::new(config);
    //
    //     let g = Secp256K1::GENERATOR_AFFINE;
    //     let not_g = AffinePoint::<Secp256K1> {
    //         x: g.x,
    //         y: g.y + Secp256K1Base::ONE,
    //         zero: g.zero,
    //     };
    //     let not_g_target = builder.constant_affine_point(not_g);
    //
    //     builder.curve_assert_valid(&not_g_target);
    //
    //     let data = builder.build::();
    //     let proof = data.prove(pw).unwrap();
    //
    //     data.verify(proof).unwrap()
    // }
    //
    // #[test]
    // fn test_curve_double() -> Result<()> {
    //     const D: usize = 2;
    //     type C = PoseidonGoldilocksConfig;
    //     type F = <C as GenericConfig<D>>::F;
    //
    //     let config = CircuitConfig::standard_ecc_config();
    //
    //     let pw = PartialWitness::new();
    //     let mut builder = CircuitBuilder::<F, D>::new(config);
    //
    //     let g = Secp256K1::GENERATOR_AFFINE;
    //     let g_target = builder.constant_affine_point(g);
    //     let neg_g_target = builder.curve_neg(&g_target);
    //
    //     let double_g = g.double();
    //     let double_g_expected = builder.constant_affine_point(double_g);
    //     builder.curve_assert_valid(&double_g_expected);
    //
    //     let double_neg_g = (-g).double();
    //     let double_neg_g_expected = builder.constant_affine_point(double_neg_g);
    //     builder.curve_assert_valid(&double_neg_g_expected);
    //
    //     let double_g_actual = builder.curve_double(&g_target);
    //     let double_neg_g_actual = builder.curve_double(&neg_g_target);
    //     builder.curve_assert_valid(&double_g_actual);
    //     builder.curve_assert_valid(&double_neg_g_actual);
    //
    //     builder.connect_affine_point(&double_g_expected, &double_g_actual);
    //     builder.connect_affine_point(&double_neg_g_expected, &double_neg_g_actual);
    //
    //     let data = builder.build::();
    //     let proof = data.prove(pw).unwrap();
    //
    //     data.verify(proof)
    // }
    //
    // #[test]
    // fn test_curve_add() -> Result<()> {
    //     const D: usize = 2;
    //     type C = PoseidonGoldilocksConfig;
    //     type F = <C as GenericConfig<D>>::F;
    //
    //     let config = CircuitConfig::standard_ecc_config();
    //
    //     let pw = PartialWitness::new();
    //     let mut builder = CircuitBuilder::<F, D>::new(config);
    //
    //     let g = Secp256K1::GENERATOR_AFFINE;
    //     let double_g = g.double();
    //     let g_plus_2g = (g + double_g).to_affine();
    //     let g_plus_2g_expected = builder.constant_affine_point(g_plus_2g);
    //     builder.curve_assert_valid(&g_plus_2g_expected);
    //
    //     let g_target = builder.constant_affine_point(g);
    //     let double_g_target = builder.curve_double(&g_target);
    //     let g_plus_2g_actual = builder.curve_add(&g_target, &double_g_target);
    //     builder.curve_assert_valid(&g_plus_2g_actual);
    //
    //     builder.connect_affine_point(&g_plus_2g_expected, &g_plus_2g_actual);
    //
    //     let data = builder.build::();
    //     let proof = data.prove(pw).unwrap();
    //
    //     data.verify(proof)
    // }
    //
    // #[test]
    // fn test_curve_conditional_add() -> Result<()> {
    //     const D: usize = 2;
    //     type C = PoseidonGoldilocksConfig;
    //     type F = <C as GenericConfig<D>>::F;
    //
    //     let config = CircuitConfig::standard_ecc_config();
    //
    //     let pw = PartialWitness::new();
    //     let mut builder = CircuitBuilder::<F, D>::new(config);
    //
    //     let g = Secp256K1::GENERATOR_AFFINE;
    //     let double_g = g.double();
    //     let g_plus_2g = (g + double_g).to_affine();
    //     let g_plus_2g_expected = builder.constant_affine_point(g_plus_2g);
    //
    //     let g_expected = builder.constant_affine_point(g);
    //     let double_g_target = builder.curve_double(&g_expected);
    //     let t = builder._true();
    //     let f = builder._false();
    //     let g_plus_2g_actual = builder.curve_conditional_add(&g_expected, &double_g_target, t);
    //     let g_actual = builder.curve_conditional_add(&g_expected, &double_g_target, f);
    //
    //     builder.connect_affine_point(&g_plus_2g_expected, &g_plus_2g_actual);
    //     builder.connect_affine_point(&g_expected, &g_actual);
    //
    //     let data = builder.build::();
    //     let proof = data.prove(pw).unwrap();
    //
    //     data.verify(proof)
    // }
    //
    #[test]
    fn test_curve_mul() {
        const D: usize = 2;
        type C = KeccakGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::wide_ecc_config();

        let mut builder = CircuitBuilder::<F, D>::new(config);

        let g = builder.constant_affine_point(SECP256K1_GENERATOR_X, SECP256K1_GENERATOR_Y);
        let scalar = builder.add_virtual_nonnative_target();
        let mul = builder.glv_mul(&g, &scalar);
        let expected = builder.constant_affine_point(
            Secp256K1Base::from_noncanonical_biguint(BigUint::from_be_bytes(&[
                47, 139, 222, 77, 26, 7, 32, 147, 85, 180, 167, 37, 10, 92, 81, 40, 232, 139, 132,
                189, 220, 97, 154, 183, 203, 168, 213, 105, 178, 64, 239, 228,
            ])),
            Secp256K1Base::from_noncanonical_biguint(BigUint::from_be_bytes(&[
                39, 83, 221, 217, 201, 26, 28, 41, 43, 36, 86, 34, 89, 54, 59, 217, 8, 119, 216,
                228, 84, 242, 151, 191, 35, 87, 130, 196, 89, 83, 153, 89,
            ])),
        );
        builder.connect_affine_point(&mul, &expected);

        let data = builder.build::<C>();
        let mut pw = PartialWitness::new();
        pw.set_biguint_target(
            &scalar.value,
            &BigUint::from_be_bytes(&[
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 254,
                186, 174, 220, 230, 175, 72, 160, 59, 191, 210, 94, 140, 208, 54, 65, 60,
            ]),
        );
        let proof = data.prove(pw).unwrap();

        let _ = data.verify(proof).unwrap();
    }
    //
    // #[test]
    // #[ignore]
    // fn test_curve_random() -> Result<()> {
    //     const D: usize = 2;
    //     type C = PoseidonGoldilocksConfig;
    //     type F = <C as GenericConfig<D>>::F;
    //
    //     let config = CircuitConfig::standard_ecc_config();
    //
    //     let pw = PartialWitness::new();
    //     let mut builder = CircuitBuilder::<F, D>::new(config);
    //
    //     let rando =
    //         (CurveScalar(Secp256K1Scalar::rand()) * Secp256K1::GENERATOR_PROJECTIVE).to_affine();
    //     let randot = builder.constant_affine_point(rando);
    //
    //     let two_target = builder.constant_nonnative(Secp256K1Scalar::TWO);
    //     let randot_doubled = builder.curve_double(&randot);
    //     let randot_times_two = builder.curve_scalar_mul(&randot, &two_target);
    //     builder.connect_affine_point(&randot_doubled, &randot_times_two);
    //
    //     let data = builder.build::();
    //     let proof = data.prove(pw).unwrap();
    //
    //     data.verify(proof)
    // }
}
