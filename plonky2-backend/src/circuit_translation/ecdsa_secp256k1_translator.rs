use super::*;
use crate::biguint::biguint::CircuitBuilderBiguint;
use crate::biguint::gadgets::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use crate::curve::gadgets::curve::{AffinePointTarget, CircuitBuilderCurve};
use plonky2::field::secp256k1_base::Secp256K1Base;
use plonky2::field::secp256k1_scalar::Secp256K1Scalar;


pub struct EcdsaSecp256k1Translator<'a> {
    circuit_builder: &'a mut CircuitBuilderFromAcirToPlonky2,
    hashed_msg: &'a Box<[FunctionInput; 32]>,
    public_key_x: &'a Box<[FunctionInput; 32]>,
    public_key_y: &'a Box<[FunctionInput; 32]>,
    signature: &'a Box<[FunctionInput; 64]>,
    output: Witness,
}

impl<'a> EcdsaSecp256k1Translator<'a> {
    pub fn new_for(
        circuit_builder: &'a mut CircuitBuilderFromAcirToPlonky2,
        hashed_msg: &'a Box<[FunctionInput; 32]>,
        public_key_x: &'a Box<[FunctionInput; 32]>,
        public_key_y: &'a Box<[FunctionInput; 32]>,
        signature: &'a Box<[FunctionInput; 64]>,
        output: Witness,
    ) -> EcdsaSecp256k1Translator<'a> {
        Self {
            circuit_builder,
            hashed_msg,
            public_key_x,
            public_key_y,
            signature,
            output,
        }
    }

    pub fn translate(&mut self) {
        let public_key = self._public_key_target();
        let r = self._vector_to_scalar_field_element(
            self.signature[0..32].to_vec()
        );
        let s = self._vector_to_scalar_field_element(
            self.signature[32..64].to_vec()
        );
        let h = self._vector_to_scalar_field_element(
            self.hashed_msg.to_vec()
        );

        let r_point = self._calculate_r(&public_key, &r, &s, &h);

        let does_signature_verify = self.circuit_builder.builder.cmp_biguint(
            &r.value, &r_point.x.value,
        );

        let output_target = self.circuit_builder.target_for_witness(self.output);
        self.circuit_builder.builder.connect(does_signature_verify.target, output_target);
    }

    fn _calculate_r(
        &mut self,
        public_key: &AffinePointTarget,
        r: &NonNativeTarget<Secp256K1Scalar>,
        s: &NonNativeTarget<Secp256K1Scalar>,
        h: &NonNativeTarget<Secp256K1Scalar>,
    ) -> AffinePointTarget {
        let s1 = self.circuit_builder.builder.inv_nonnative(&s);

        let u_1 = self.circuit_builder.builder.mul_nonnative(&h, &s1);
        let u_2 = self.circuit_builder.builder.mul_nonnative(&r, &s1);

        let generator = self.circuit_builder.builder.curve_generator_constant();
        let r_factor_1 = self.circuit_builder.builder.curve_scalar_mul(
            &generator, &u_1,
        );

        let r_factor_2 = self.circuit_builder.builder.curve_scalar_mul(
            &public_key, &u_2,
        );

        let r_point = self.circuit_builder.builder.curve_add(
            &r_factor_1, &r_factor_2,
        );
        r_point
    }

    fn _public_key_target(&mut self) -> AffinePointTarget {
        let x = self._vector_to_base_field_element(self.public_key_x.to_vec());
        let y = self._vector_to_base_field_element(self.public_key_y.to_vec());
        AffinePointTarget { x, y }
    }

    fn _vector_to_base_field_element(
        &mut self, inputs: Vec<FunctionInput>,
    ) -> NonNativeTarget<Secp256K1Base> {
        let bytes_targets = self._inputs_to_targets(inputs);

        // Create a virtual NonNative target
        let nonnative_target = self.circuit_builder.builder.add_virtual_nonnative_target_sized(8);

        // BoolTargets for the NonNative target
        let bits_target_in_nonnative = self.circuit_builder.builder.split_nonnative_to_bits(
            &nonnative_target
        );

        self._connect_32_bytes_with_256_bits(bytes_targets, bits_target_in_nonnative);
        nonnative_target
    }

    fn _vector_to_scalar_field_element(
        &mut self, inputs: Vec<FunctionInput>,
    ) -> NonNativeTarget<Secp256K1Scalar> {
        let bytes_targets = self._inputs_to_targets(inputs);

        // Create a virtual NonNative target
        let nonnative_target = self.circuit_builder.builder.add_virtual_nonnative_target_sized(8);

        // BoolTargets for the NonNative target
        let bits_target_in_nonnative = self.circuit_builder.builder.split_nonnative_to_bits(
            &nonnative_target
        );

        self._connect_32_bytes_with_256_bits(bytes_targets, bits_target_in_nonnative);
        nonnative_target
    }

    fn _connect_32_bytes_with_256_bits(&mut self, bytes_targets: Vec<BinaryDigitsTarget>, bits_targets: Vec<BoolTarget>) {
        for index_byte in 0..32 {
            for index_bit in 0..8 {
                let index_bit_in_biguint = 8 * index_byte + index_bit;
                self.circuit_builder.builder.connect(
                    bits_targets[index_bit_in_biguint].target,
                    bytes_targets[index_byte].bits[index_bit].target,
                );
            }
        }
    }

    fn _inputs_to_targets(&mut self, inputs: Vec<FunctionInput>) -> Vec<BinaryDigitsTarget> {
        inputs
            .iter()
            .map(|input| {
                self.circuit_builder.binary_number_target_for_witness(input.witness, 8)
            })
            .collect()
    }
}
