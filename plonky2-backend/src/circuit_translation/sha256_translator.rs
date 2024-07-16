use super::*;

#[derive(Clone)]
struct CompressionIterationState {
    a: BinaryDigitsTarget,
    b: BinaryDigitsTarget,
    c: BinaryDigitsTarget,
    d: BinaryDigitsTarget,
    e: BinaryDigitsTarget,
    f: BinaryDigitsTarget,
    g: BinaryDigitsTarget,
    h: BinaryDigitsTarget,
}

impl CompressionIterationState {
    pub fn from_vec(vec: Vec<BinaryDigitsTarget>) -> Self {
        Self {
            a: vec[0].clone(),
            b: vec[1].clone(),
            c: vec[2].clone(),
            d: vec[3].clone(),
            e: vec[4].clone(),
            f: vec[5].clone(),
            g: vec[6].clone(),
            h: vec[7].clone(),
        }
    }

    pub fn unpack(self) -> [BinaryDigitsTarget; 8] {
        [
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h,
        ]
    }
}

pub struct Sha256CompressionTranslator<'a> {
    circuit_builder: &'a mut CircuitBuilderFromAcirToPlonky2,
    inputs: &'a Box<[FunctionInput; 16]>,
    hash_values: &'a Box<[FunctionInput; 8]>,
    outputs: &'a Box<[Witness; 8]>,
}

impl<'a> Sha256CompressionTranslator<'a> {
    pub fn new_for(
        circuit_builder: &'a mut CircuitBuilderFromAcirToPlonky2,
        inputs: &'a Box<[FunctionInput; 16]>,
        hash_values: &'a Box<[FunctionInput; 8]>,
        outputs: &'a Box<[Witness; 8]>,
    ) -> Sha256CompressionTranslator<'a> {
        Self {
            circuit_builder,
            inputs,
            hash_values,
            outputs,
        }
    }

    pub fn translate(&mut self) {
        let mut binary_input_targets: Vec<BinaryDigitsTarget> = self
            .inputs
            .into_iter()
            .map(|input| {
                self.circuit_builder
                    .binary_number_target_for_witness(input.witness, 32)
            })
            .collect();

        for t in 16..64 {
            let new_w_t = self.calculate_w_t(
                &binary_input_targets[t - 2],
                &binary_input_targets[t - 7],
                &binary_input_targets[t - 15],
                &binary_input_targets[t - 16],
            );
            binary_input_targets.push(new_w_t);
        }

        let mut constant_k_digit_target_values = self.initial_k();
        let initial_h = self.initial_h();
        let mut iteration_states: Vec<CompressionIterationState> =
            vec![CompressionIterationState::from_vec(initial_h.clone())];
        for t in 0..64 {
            let prev_iteration_state = iteration_states[t].clone();
            let next_state = self.compression_function_iteration(
                prev_iteration_state,
                &binary_input_targets[t],
                &constant_k_digit_target_values[t],
            );
            iteration_states.push(next_state);
        }
        // Link all the binary digits target outputs into the corresponding output targets
        let last_iteration_state = iteration_states.last().unwrap().clone();
        let output_binary_targets = last_iteration_state.unpack();

        let mut final_h: Vec<BinaryDigitsTarget> = Vec::new();
        for i in 0..8 {
            final_h.push(BinaryDigitsTarget::add_module_32_bits(
                &initial_h[i],
                &output_binary_targets[i],
                &mut self.circuit_builder.builder,
            ))
        }

        for (output_witness, output_binary_target) in self.outputs.iter().zip(final_h.iter()) {
            let new_output_target = self
                .circuit_builder
                .convert_binary_number_to_number(output_binary_target.clone());
            self.circuit_builder
                .witness_target_map
                .insert(*output_witness, new_output_target);
        }
    }

    fn sigma_0(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = BinaryDigitsTarget::rotate_right(target, 7, &mut self.circuit_builder.builder);
        let x2 = BinaryDigitsTarget::rotate_right(target, 18, &mut self.circuit_builder.builder);
        let x3 = BinaryDigitsTarget::shift_right(target, 3, &mut self.circuit_builder.builder);

        let y1 = BinaryDigitsTarget::xor(x1, x2, &mut self.circuit_builder.builder);
        let y2 = BinaryDigitsTarget::xor(y1, x3, &mut self.circuit_builder.builder);

        y2
    }

    fn sigma_1(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = BinaryDigitsTarget::rotate_right(target, 17, &mut self.circuit_builder.builder);
        let x2 = BinaryDigitsTarget::rotate_right(target, 19, &mut self.circuit_builder.builder);
        let x3 = BinaryDigitsTarget::shift_right(target, 10, &mut self.circuit_builder.builder);

        let y1 = BinaryDigitsTarget::xor(x1, x2, &mut self.circuit_builder.builder);
        let y2 = BinaryDigitsTarget::xor(y1, x3, &mut self.circuit_builder.builder);

        y2
    }

    fn big_sigma_0(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = BinaryDigitsTarget::rotate_right(target, 2, &mut self.circuit_builder.builder);
        let x2 = BinaryDigitsTarget::rotate_right(target, 13, &mut self.circuit_builder.builder);
        let x3 = BinaryDigitsTarget::rotate_right(target, 22, &mut self.circuit_builder.builder);

        let y1 = BinaryDigitsTarget::xor(x1, x2, &mut self.circuit_builder.builder);
        let y2 = BinaryDigitsTarget::xor(y1, x3, &mut self.circuit_builder.builder);
        y2
    }

    fn big_sigma_1(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = BinaryDigitsTarget::rotate_right(target, 6, &mut self.circuit_builder.builder);
        let x2 = BinaryDigitsTarget::rotate_right(target, 11, &mut self.circuit_builder.builder);
        let x3 = BinaryDigitsTarget::rotate_right(target, 25, &mut self.circuit_builder.builder);

        let y1 = BinaryDigitsTarget::xor(x1, x2, &mut self.circuit_builder.builder);
        let y2 = BinaryDigitsTarget::xor(y1, x3, &mut self.circuit_builder.builder);
        y2
    }

    fn initial_h(&mut self) -> Vec<BinaryDigitsTarget> {
        let mut binary_inputs: Vec<BinaryDigitsTarget> = self
            .inputs
            .into_iter()
            .map(|input| {
                self.circuit_builder
                    .binary_number_target_for_witness(input.witness, 32)
            })
            .collect();
        binary_inputs
    }

    fn initial_k(&mut self) -> Vec<BinaryDigitsTarget> {
        let sha256_k_constants: Vec<u32> = vec![
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
            0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
            0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
            0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
            0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
            0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
            0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
            0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
            0xc67178f2,
        ];
        let binary_constants = sha256_k_constants
            .iter()
            .map(|n| {
                self.circuit_builder
                    .binary_number_target_for_constant(*n as usize, 32)
            })
            .collect();
        binary_constants
    }

    fn calculate_w_t(
        &mut self,
        w_t_2: &BinaryDigitsTarget,
        w_t_7: &BinaryDigitsTarget,
        w_t_15: &BinaryDigitsTarget,
        w_t_16: &BinaryDigitsTarget,
    ) -> BinaryDigitsTarget {
        let sigma_1 = self.sigma_1(w_t_2);
        let sumand_1 = BinaryDigitsTarget::add_module_32_bits(
            &sigma_1,
            w_t_7,
            &mut self.circuit_builder.builder,
        );
        let sigma_0 = self.sigma_0(w_t_15);
        let sumand_2 = BinaryDigitsTarget::add_module_32_bits(
            &sigma_0,
            w_t_16,
            &mut self.circuit_builder.builder,
        );
        BinaryDigitsTarget::add_module_32_bits(
            &sumand_1,
            &sumand_2,
            &mut self.circuit_builder.builder,
        )
    }

    fn compression_function_iteration(
        &mut self,
        s: CompressionIterationState,
        w_t: &BinaryDigitsTarget,
        k_t: &BinaryDigitsTarget,
    ) -> CompressionIterationState {
        let [a, b, c, d, e, f, g, h] = s.unpack();
        let big_sigma_1 = self.big_sigma_1(&e);
        let choose_e_f_g =
            BinaryDigitsTarget::choose(&e, &f, &g, &mut self.circuit_builder.builder);
        let sumand_0 =
            BinaryDigitsTarget::add_module_32_bits(k_t, w_t, &mut self.circuit_builder.builder);
        let sumand_1 = BinaryDigitsTarget::add_module_32_bits(
            &h,
            &big_sigma_1,
            &mut self.circuit_builder.builder,
        );
        let sumand_2 = BinaryDigitsTarget::add_module_32_bits(
            &choose_e_f_g,
            &sumand_0,
            &mut self.circuit_builder.builder,
        );
        let t_1 = BinaryDigitsTarget::add_module_32_bits(
            &sumand_1,
            &sumand_2,
            &mut self.circuit_builder.builder,
        );

        let big_sigma_0 = self.big_sigma_0(&a);
        let majority_a_b_c =
            BinaryDigitsTarget::majority(&a, &b, &c, &mut self.circuit_builder.builder);
        let t_2 = BinaryDigitsTarget::add_module_32_bits(
            &big_sigma_0,
            &majority_a_b_c,
            &mut self.circuit_builder.builder,
        );
        CompressionIterationState {
            a: BinaryDigitsTarget::add_module_32_bits(
                &t_1,
                &t_2,
                &mut self.circuit_builder.builder,
            ),
            b: a,
            c: b,
            d: c,
            e: BinaryDigitsTarget::add_module_32_bits(&d, &t_1, &mut self.circuit_builder.builder),
            f: e,
            g: f,
            h: g,
        }
    }
}
