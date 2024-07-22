use super::*;


/// Module in charge of translating each AssertZero operation. Currently, the Plonky2 feature used
/// to deal with systems of equations is the ArithmeticGate, which the CircuitBuilder uses
/// extensively (through methods like add, sub, mul, mul_const, etc.)
pub struct AssertZeroTranslator<'a> {
    builder: &'a mut CircuitBuilder<F, D>,
    witness_target_map: &'a mut HashMap<Witness, Target>,
    expression: &'a Expression,
}

impl<'a> AssertZeroTranslator<'a> {
    pub fn new_for(
        builder: &'a mut CircuitBuilder<F, D>,
        witness_target_map: &'a mut HashMap<Witness, Target>,
        expression: &'a Expression,
    ) -> AssertZeroTranslator<'a> {
        Self {
            builder,
            witness_target_map,
            expression,
        }
    }

    pub fn translate(&mut self) {
        self._register_intermediate_witnesses_for_assert_zero();
        self._translate_assert_zero();
    }

    fn _translate_assert_zero(self: &mut Self) {
        let g_constant = self._field_element_to_goldilocks_field(&self.expression.q_c);

        let constant_target = self.builder.constant(g_constant);
        let mut current_acc_target = constant_target;
        current_acc_target = self._add_linear_combinations(current_acc_target);
        current_acc_target = self._add_cuadratic_combinations(current_acc_target);
        self.builder.assert_zero(current_acc_target);
    }

    fn _get_or_create_target_for_witness(self: &mut Self, witness: Witness) -> Target {
        match self.witness_target_map.get(&witness) {
            Some(target) => *target,
            None => {
                let target = self.builder.add_virtual_target();
                self.witness_target_map.insert(witness, target);
                target
            }
        }
    }

    /// Witnesses that weren't registered previously because it's its first appearance.
    fn _register_intermediate_witnesses_for_assert_zero(&mut self) {
        for (_, witness_1, witness_2) in &self.expression.mul_terms {
            self._get_or_create_target_for_witness(*witness_1);
            self._get_or_create_target_for_witness(*witness_2);
        }
        for (_, witness) in &self.expression.linear_combinations {
            self._get_or_create_target_for_witness(*witness);
        }
    }

    fn _add_cuadratic_combinations(self: &mut Self, mut current_acc_target: Target) -> Target {
        let mul_terms = &self.expression.mul_terms;
        for mul_term in mul_terms {
            let (f_cuadratic_factor, public_input_witness_1, public_input_witness_2) = mul_term;
            let cuadratic_target = self._compute_cuadratic_term_target(
                f_cuadratic_factor,
                public_input_witness_1,
                public_input_witness_2,
            );
            let new_target = self.builder.add(cuadratic_target, current_acc_target);
            current_acc_target = new_target;
        }
        current_acc_target
    }

    fn _add_linear_combinations(self: &mut Self, mut current_acc_target: Target) -> Target {
        let linear_combinations = &self.expression.linear_combinations;
        for (f_multiply_factor, public_input_witness) in linear_combinations {
            let linear_combination_target =
                self._compute_linear_combination_target(f_multiply_factor, public_input_witness);
            let new_target = self
                .builder
                .add(linear_combination_target, current_acc_target);
            current_acc_target = new_target;
        }
        current_acc_target
    }

    fn _compute_linear_combination_target(
        self: &mut Self,
        f_multiply_constant_factor: &FieldElement,
        public_input_witness: &Witness,
    ) -> Target {
        let factor_target = *self.witness_target_map.get(public_input_witness).unwrap();
        let g_first_pi_factor = self._field_element_to_goldilocks_field(f_multiply_constant_factor);
        self.builder.mul_const(g_first_pi_factor, factor_target)
    }

    fn _compute_cuadratic_term_target(
        self: &mut Self,
        f_cuadratic_factor: &FieldElement,
        public_input_witness_1: &Witness,
        public_input_witness_2: &Witness,
    ) -> Target {
        let g_cuadratic_factor = self._field_element_to_goldilocks_field(f_cuadratic_factor);
        let first_public_input_target =
            *self.witness_target_map.get(public_input_witness_1).unwrap();
        let second_public_input_target =
            *self.witness_target_map.get(public_input_witness_2).unwrap();

        let cuadratic_target = self
            .builder
            .mul(first_public_input_target, second_public_input_target);
        self.builder.mul_const(g_cuadratic_factor, cuadratic_target)
    }

    fn _field_element_to_goldilocks_field(self: &mut Self, fe: &FieldElement) -> F {
        let fe_as_big_uint = BigUint::from_bytes_be(&fe.to_be_bytes() as &[u8]);
        F::from_noncanonical_biguint(fe_as_big_uint)
    }
}
