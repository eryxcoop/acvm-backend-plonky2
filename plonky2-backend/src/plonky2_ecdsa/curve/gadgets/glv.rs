use core::marker::PhantomData;

use num::rational::Ratio;
use num::traits::FromBytes;
use num::BigUint;
use plonky2::field::extension::Extendable;
use plonky2::field::secp256k1_base::Secp256K1Base;
use plonky2::field::secp256k1_scalar::Secp256K1Scalar;
use plonky2::field::types::{Field, PrimeField};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;

use crate::plonky2_ecdsa::biguint::biguint::{BigUintTarget, GeneratedValuesBigUint, WitnessBigUint};
use crate::plonky2_ecdsa::biguint::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use crate::plonky2_ecdsa::biguint::gadgets::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use crate::plonky2_ecdsa::biguint::gadgets::split_nonnative::CircuitBuilderSplit;

use super::curve::{generate_random_point, AffinePointTarget, CircuitBuilderCurve};

pub const GLV_BETA: Secp256K1Base = Secp256K1Base([
    13923278643952681454,
    11308619431505398165,
    7954561588662645993,
    8856726876819556112,
]);

pub const GLV_S: Secp256K1Scalar = Secp256K1Scalar([
    16069571880186789234,
    1310022930574435960,
    11900229862571533402,
    6008836872998760672,
]);

const A1: Secp256K1Scalar = Secp256K1Scalar([16747920425669159701, 3496713202691238861, 0, 0]);

const MINUS_B1: Secp256K1Scalar =
    Secp256K1Scalar([8022177200260244675, 16448129721693014056, 0, 0]);

const A2: Secp256K1Scalar = Secp256K1Scalar([6323353552219852760, 1498098850674701302, 1, 0]);

const B2: Secp256K1Scalar = Secp256K1Scalar([16747920425669159701, 3496713202691238861, 0, 0]);

/// Algorithm 15.41 in Handbook of Elliptic and Hyperelliptic Curve Cryptography.
/// Decompose a scalar `k` into two small scalars `k1, k2` with `|k1|, |k2| < √p` that satisfy
/// `k1 + s * k2 = k`.
/// Returns `(|k1|, |k2|, k1 < 0, k2 < 0)`.
pub fn decompose_secp256k1_scalar(
    k: Secp256K1Scalar,
) -> (Secp256K1Scalar, Secp256K1Scalar, bool, bool) {
    let p = Secp256K1Scalar::order();
    let c1_biguint = Ratio::new(
        B2.to_canonical_biguint() * k.to_canonical_biguint(),
        p.clone(),
    )
    .round()
    .to_integer();
    let c1 = Secp256K1Scalar::from_noncanonical_biguint(c1_biguint);
    let c2_biguint = Ratio::new(
        MINUS_B1.to_canonical_biguint() * k.to_canonical_biguint(),
        p.clone(),
    )
    .round()
    .to_integer();
    let c2 = Secp256K1Scalar::from_noncanonical_biguint(c2_biguint);

    let k1_raw = k - c1 * A1 - c2 * A2;
    let k2_raw = c1 * MINUS_B1 - c2 * B2;
    debug_assert!(k1_raw + GLV_S * k2_raw == k);

    let two = BigUint::from_slice(&[2]);
    let k1_neg = k1_raw.to_canonical_biguint() > p.clone() / two.clone();
    let k1 = if k1_neg {
        Secp256K1Scalar::from_noncanonical_biguint(p.clone() - k1_raw.to_canonical_biguint())
    } else {
        k1_raw
    };
    let k2_neg = k2_raw.to_canonical_biguint() > p.clone() / two;
    let k2 = if k2_neg {
        Secp256K1Scalar::from_noncanonical_biguint(p - k2_raw.to_canonical_biguint())
    } else {
        k2_raw
    };

    (k1, k2, k1_neg, k2_neg)
}

pub trait CircuitBuilderGlv<F: RichField + Extendable<D>, const D: usize> {
    fn secp256k1_glv_beta(&mut self) -> NonNativeTarget<Secp256K1Base>;

    fn decompose_secp256k1_scalar(
        &mut self,
        k: &NonNativeTarget<Secp256K1Scalar>,
    ) -> (
        NonNativeTarget<Secp256K1Scalar>,
        NonNativeTarget<Secp256K1Scalar>,
        BoolTarget,
        BoolTarget,
    );

    fn glv_mul(
        &mut self,
        p: &AffinePointTarget,
        k: &NonNativeTarget<Secp256K1Scalar>,
    ) -> AffinePointTarget;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderGlv<F, D>
    for CircuitBuilder<F, D>
{
    fn secp256k1_glv_beta(&mut self) -> NonNativeTarget<Secp256K1Base> {
        self.constant_nonnative(GLV_BETA)
    }

    fn decompose_secp256k1_scalar(
        &mut self,
        k: &NonNativeTarget<Secp256K1Scalar>,
    ) -> (
        NonNativeTarget<Secp256K1Scalar>,
        NonNativeTarget<Secp256K1Scalar>,
        BoolTarget,
        BoolTarget,
    ) {
        let k1 = self.add_virtual_nonnative_target_sized::<Secp256K1Scalar>(4);
        let k2 = self.add_virtual_nonnative_target_sized::<Secp256K1Scalar>(4);
        let k1_neg = self.add_virtual_bool_target_unsafe();
        let k2_neg = self.add_virtual_bool_target_unsafe();

        self.add_simple_generator(GLVDecompositionGenerator::<F, D> {
            k: k.clone(),
            k1: k1.clone(),
            k2: k2.clone(),
            k1_neg,
            k2_neg,
            _phantom: PhantomData,
        });

        // Check that `k1_raw + GLV_S * k2_raw == k`.
        let k1_raw = self.nonnative_conditional_neg(&k1, k1_neg);
        let k2_raw = self.nonnative_conditional_neg(&k2, k2_neg);
        let s = self.constant_nonnative(GLV_S);
        let mut should_be_k = self.mul_nonnative(&s, &k2_raw);
        should_be_k = self.add_nonnative(&should_be_k, &k1_raw);
        self.connect_nonnative(&should_be_k, k);

        (k1, k2, k1_neg, k2_neg)
    }

    fn glv_mul(
        &mut self,
        p: &AffinePointTarget,
        k: &NonNativeTarget<Secp256K1Scalar>,
    ) -> AffinePointTarget {
        let (k1, k2, k1_neg, k2_neg) = self.decompose_secp256k1_scalar(k);

        let beta = self.secp256k1_glv_beta();
        let beta_px = self.mul_nonnative(&beta, &p.x);
        let sp = AffinePointTarget {
            x: beta_px,
            y: p.y.clone(),
        };

        let p_neg = self.curve_conditional_neg(p, k1_neg);
        let sp_neg = self.curve_conditional_neg(&sp, k2_neg);
        curve_msm_circuit(self, &p_neg, &sp_neg, &k1, &k2)
    }
}

/// Computes `n*p + m*q` using windowed MSM, with a 2-bit window.
/// See Algorithm 9.23 in Handbook of Elliptic and Hyperelliptic Curve Cryptography for a
/// description.
/// Note: Doesn't work if `p == q`.
pub fn curve_msm_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    p: &AffinePointTarget,
    q: &AffinePointTarget,
    n: &NonNativeTarget<Secp256K1Scalar>,
    m: &NonNativeTarget<Secp256K1Scalar>,
) -> AffinePointTarget {
    let limbs_n = builder.split_nonnative_to_2_bit_limbs(n);
    let limbs_m = builder.split_nonnative_to_2_bit_limbs(m);
    assert_eq!(limbs_n.len(), limbs_m.len());

    let rando_t = builder.constant_affine_point(
        Secp256K1Base::from_noncanonical_biguint(BigUint::from_be_bytes(&[
            168, 108, 112, 254, 40, 235, 44, 180, 232, 129, 170, 129, 151, 26, 229, 18, 19, 137,
            245, 62, 139, 130, 119, 30, 84, 53, 9, 156, 170, 172, 160, 15,
        ])),
        Secp256K1Base::from_noncanonical_biguint(BigUint::from_be_bytes(&[
            60, 32, 167, 79, 44, 197, 157, 125, 248, 190, 148, 181, 142, 227, 95, 8, 136, 133, 192,
            43, 110, 22, 130, 29, 171, 221, 92, 43, 9, 1, 185, 27,
        ])),
    );

    let neg_rando = builder.constant_affine_point(
        Secp256K1Base::from_noncanonical_biguint(BigUint::from_be_bytes(&[
            168, 108, 112, 254, 40, 235, 44, 180, 232, 129, 170, 129, 151, 26, 229, 18, 19, 137,
            245, 62, 139, 130, 119, 30, 84, 53, 9, 156, 170, 172, 160, 15,
        ])),
        Secp256K1Base::from_noncanonical_biguint(BigUint::from_be_bytes(&[
            195, 223, 88, 176, 211, 58, 98, 130, 7, 65, 107, 74, 113, 28, 160, 247, 119, 122, 63,
            212, 145, 233, 125, 226, 84, 34, 163, 211, 246, 254, 67, 20,
        ])),
    );

    // Precomputes `precomputation[i + 4*j] = i*p + j*q` for `i,j=0..4`.
    let mut precomputation = vec![p.clone(); 16];
    let mut cur_p = rando_t.clone();
    let mut cur_q = rando_t.clone();
    for i in 0..4 {
        precomputation[i] = cur_p.clone();
        precomputation[4 * i] = cur_q.clone();
        cur_p = builder.curve_add(&cur_p, p);
        cur_q = builder.curve_add(&cur_q, q);
    }
    for i in 1..4 {
        precomputation[i] = builder.curve_add(&precomputation[i], &neg_rando);
        precomputation[4 * i] = builder.curve_add(&precomputation[4 * i], &neg_rando);
    }
    for i in 1..4 {
        for j in 1..4 {
            precomputation[i + 4 * j] =
                builder.curve_add(&precomputation[i], &precomputation[4 * j]);
        }
    }

    let four = builder.constant(F::from_canonical_usize(4));

    let zero = builder.zero();
    let mut result = rando_t;
    for (limb_n, limb_m) in limbs_n.into_iter().zip(limbs_m).rev() {
        result = builder.curve_repeated_double(&result, 2);
        let index = builder.mul_add(four, limb_m, limb_n);
        let r = builder.random_access_curve_points(index, precomputation.clone());
        let is_zero = builder.is_equal(index, zero);
        let should_add = builder.not(is_zero);
        result = builder.curve_conditional_add(&result, &r, should_add);
    }
    let to_add = builder.constant_affine_point(
        Secp256K1Base::from_noncanonical_biguint(BigUint::from_be_bytes(&[
            4, 240, 116, 128, 2, 142, 26, 67, 121, 228, 15, 172, 125, 56, 178, 55, 220, 178, 31,
            194, 90, 168, 40, 127, 59, 193, 0, 121, 236, 178, 130, 29,
        ])),
        Secp256K1Base::from_noncanonical_biguint(BigUint::from_be_bytes(&[
            195, 20, 74, 65, 215, 167, 153, 201, 235, 110, 231, 40, 207, 121, 30, 55, 18, 16, 205,
            138, 169, 66, 20, 253, 49, 54, 35, 152, 247, 117, 246, 155,
        ])),
    );

    result = builder.curve_add(&result, &to_add);

    result
}

#[derive(Debug)]
struct GLVDecompositionGenerator<F: RichField + Extendable<D>, const D: usize> {
    k: NonNativeTarget<Secp256K1Scalar>,
    k1: NonNativeTarget<Secp256K1Scalar>,
    k2: NonNativeTarget<Secp256K1Scalar>,
    k1_neg: BoolTarget,
    k2_neg: BoolTarget,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for GLVDecompositionGenerator<F, D>
{
    fn dependencies(&self) -> Vec<Target> {
        self.k.value.limbs.iter().map(|l| l.0).collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let k = Secp256K1Scalar::from_noncanonical_biguint(
            witness.get_biguint_target(self.k.value.clone()),
        );

        let (k1, k2, k1_neg, k2_neg) = decompose_secp256k1_scalar(k);

        out_buffer.set_biguint_target(&self.k1.value, &k1.to_canonical_biguint());
        out_buffer.set_biguint_target(&self.k2.value, &k2.to_canonical_biguint());
        out_buffer.set_bool_target(self.k1_neg, k1_neg);
        out_buffer.set_bool_target(self.k2_neg, k2_neg);
    }

    fn id(&self) -> String {
        todo!()
    }

    fn serialize(
        &self,
        _dst: &mut Vec<u8>,
        _common_data: &plonky2::plonk::circuit_data::CommonCircuitData<F, D>,
    ) -> plonky2::util::serialization::IoResult<()> {
        todo!()
    }

    fn deserialize(
        _src: &mut plonky2::util::serialization::Buffer,
        _common_data: &plonky2::plonk::circuit_data::CommonCircuitData<F, D>,
    ) -> plonky2::util::serialization::IoResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

const WINDOW_SIZE: usize = 4;

pub trait CircuitBuilderWindowedMul<F: RichField + Extendable<D>, const D: usize> {
    fn precompute_window(&mut self, p: &AffinePointTarget) -> Vec<AffinePointTarget>;

    fn random_access_curve_points(
        &mut self,
        access_index: Target,
        v: Vec<AffinePointTarget>,
    ) -> AffinePointTarget;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderWindowedMul<F, D>
    for CircuitBuilder<F, D>
{
    fn precompute_window(&mut self, p: &AffinePointTarget) -> Vec<AffinePointTarget> {
        let (x, y) = generate_random_point(self);
        let g = self.constant_affine_point(x, y);
        let neg = self.constant_affine_point(x, -y);
        let mut multiples = vec![g];
        for i in 1..1 << WINDOW_SIZE {
            multiples.push(self.curve_add(p, &multiples[i - 1]));
        }
        for i in 1..1 << WINDOW_SIZE {
            multiples[i] = self.curve_add(&neg, &multiples[i]);
        }
        multiples
    }

    fn random_access_curve_points(
        &mut self,
        access_index: Target,
        v: Vec<AffinePointTarget>,
    ) -> AffinePointTarget {
        let num_limbs = 8;
        let zero = self.zero_u32();
        let x_limbs: Vec<Vec<_>> = (0..num_limbs)
            .map(|i| {
                v.iter()
                    .map(|p| p.x.value.limbs.get(i).unwrap_or(&zero).0)
                    .collect()
            })
            .collect();
        let y_limbs: Vec<Vec<_>> = (0..num_limbs)
            .map(|i| {
                v.iter()
                    .map(|p| p.y.value.limbs.get(i).unwrap_or(&zero).0)
                    .collect()
            })
            .collect();

        let selected_x_limbs: Vec<_> = x_limbs
            .iter()
            .map(|limbs| U32Target(self.random_access(access_index, limbs.clone())))
            .collect();
        let selected_y_limbs: Vec<_> = y_limbs
            .iter()
            .map(|limbs| U32Target(self.random_access(access_index, limbs.clone())))
            .collect();

        let x = NonNativeTarget {
            value: BigUintTarget {
                limbs: selected_x_limbs,
            },
            _phantom: PhantomData,
        };
        let y = NonNativeTarget {
            value: BigUintTarget {
                limbs: selected_y_limbs,
            },
            _phantom: PhantomData,
        };
        AffinePointTarget { x, y }
    }
}
