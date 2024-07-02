use super::*;

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
        self._register_targets_for_input_witnesses();
        let mut binary_inputs: Vec<BinaryDigitsTarget> = self
            .inputs
            .into_iter()
            .map(|input| {
                self.circuit_builder
                    .binary_number_target_for_witness(input.witness, 32)
            })
            .collect();

        let mut k = self.initial_k();
        let mut current_state = CompressionIterationState::from_vec(self.initial_h());
        for t in 16..64 {
            let current_k = k[t].clone();
            let current_w = self.calculate_w_t(
                &binary_inputs[t - 2],
                &binary_inputs[t - 7],
                &binary_inputs[t - 15],
                &binary_inputs[t - 16],
            );
            binary_inputs.push(current_w.clone());
            let next_state =
                self.compression_function_iteration(current_state, &current_w, &current_k);
            current_state = next_state;
        }

        // Link all the binary digits target outputs into the corresponding output targets
        let output_binary_targets = current_state.unpack();
        for (output_witness, output_binary_target) in
            self.outputs.iter().zip(output_binary_targets.iter())
        {
            let new_output_target = self
                .circuit_builder
                .convert_binary_number_to_number(output_binary_target.clone());
            self.circuit_builder
                .witness_target_map
                .insert(*output_witness, new_output_target);
        }
    }

    fn _register_targets_for_input_witnesses(&mut self) {
        for hash_value in self.hash_values.iter() {
            self._get_or_create_target_for_witness(hash_value.witness);
        }
    }

    fn binary_digit_of_32_bits_from_witnesses(
        &mut self,
        witness_bytes: Vec<Witness>,
    ) -> BinaryDigitsTarget {
        let targets: Vec<Target> = witness_bytes
            .into_iter()
            .map(|witness| {
                self.circuit_builder
                    ._get_or_create_target_for_witness(witness)
            })
            .collect();
        let byte_targets: Vec<BinaryDigitsTarget> = targets
            .iter()
            .map(|target| {
                self.circuit_builder
                    .convert_number_to_binary_number(*target, 8)
            })
            .collect();

        let mut bits = vec![];
        for idx_byte in 0..4 {
            if idx_byte < byte_targets.len() {
                for idx_bit in 0..8 {
                    bits.push(byte_targets[idx_byte].bits[idx_bit]);
                }
            } else {
                for _ in 0..8 {
                    let zeroes: &mut Vec<BoolTarget> = &mut self.circuit_builder.zeroes(8);
                    bits.append(zeroes);
                }
            }
        }

        BinaryDigitsTarget { bits }
    }

    fn sigma_0(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = self.circuit_builder.rotate_right(target, 7);
        let x2 = self.circuit_builder.rotate_right(target, 18);
        let x3 = self.circuit_builder.shift_right(target, 3);

        let y1 = self.circuit_builder.xor(x1, x2);
        let y2 = self.circuit_builder.xor(y1, x3);

        y2
    }

    fn sigma_1(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = self.circuit_builder.rotate_right(target, 17);
        let x2 = self.circuit_builder.rotate_right(target, 19);
        let x3 = self.circuit_builder.shift_right(target, 10);

        let y1 = self.circuit_builder.xor(x1, x2);
        let y2 = self.circuit_builder.xor(y1, x3);

        y2
    }

    fn choose(
        &mut self,
        chooser: &BinaryDigitsTarget,
        on_true: &BinaryDigitsTarget,
        on_false: &BinaryDigitsTarget,
    ) -> BinaryDigitsTarget {
        let bit_pairs_iter = on_true.bits.iter().zip(on_false.bits.iter());

        let chosen_bits = chooser
            .bits
            .iter()
            .zip(bit_pairs_iter)
            .map(|(c, (t, f))| self.select_bool_target(c, t, f))
            .collect();

        BinaryDigitsTarget { bits: chosen_bits }
    }

    fn majority(
        &mut self,
        a: &BinaryDigitsTarget,
        b: &BinaryDigitsTarget,
        c: &BinaryDigitsTarget,
    ) -> BinaryDigitsTarget {
        let bit_pairs_iter = a.bits.iter().zip(b.bits.iter());

        let majority_bits = c
            .bits
            .iter()
            .zip(bit_pairs_iter)
            .map(|(b0, (b1, b2))| {
                let on_true = self.circuit_builder.bit_or(*b1, *b2);
                let on_false = self.circuit_builder.bit_and(*b1, *b2);
                self.select_bool_target(b0, &on_true, &on_false)
            })
            .collect();
        BinaryDigitsTarget {
            bits: majority_bits,
        }
    }

    fn select_bool_target(
        &mut self,
        chooser: &BoolTarget,
        on_true: &BoolTarget,
        on_false: &BoolTarget,
    ) -> BoolTarget {
        let target = self
            .circuit_builder
            .builder
            .select(*chooser, on_true.target, on_false.target);
        BoolTarget::new_unsafe(target)
    }

    fn _extract_witnesses(&self, inputs: Vec<FunctionInput>) -> Vec<Witness> {
        inputs.into_iter().map(|input| input.witness).collect()
    }

    fn _get_or_create_target_for_witness(self: &mut Self, witness: Witness) -> Target {
        match self.circuit_builder.witness_target_map.get(&witness) {
            Some(target) => *target,
            _ => {
                // None
                let target = self.circuit_builder.builder.add_virtual_target();
                self.circuit_builder
                    .witness_target_map
                    .insert(witness, target);
                target
            }
        }
    }

    fn initial_h(&mut self) -> Vec<BinaryDigitsTarget> {
        let numbers: Vec<u32> = vec![
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
            0x5be0cd19,
        ];
        numbers
            .iter()
            .map(|n| {
                self.circuit_builder
                    .binary_number_target_for_constant(*n as usize, 32)
            })
            .collect()
    }

    fn initial_k(&mut self) -> Vec<BinaryDigitsTarget> {
        let numbers: Vec<u32> = vec![
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
        numbers
            .iter()
            .map(|n| {
                self.circuit_builder
                    .binary_number_target_for_constant(*n as usize, 32)
            })
            .collect()
    }

    fn calculate_w_t(
        &mut self,
        w_t_2: &BinaryDigitsTarget,
        w_t_7: &BinaryDigitsTarget,
        w_t_15: &BinaryDigitsTarget,
        w_t_16: &BinaryDigitsTarget,
    ) -> BinaryDigitsTarget {
        let sigma_1 = self.sigma_1(w_t_2);
        let sumand_1 = self.circuit_builder.add_module_32_bits(&sigma_1, w_t_7);
        let sigma_0 = self.sigma_0(w_t_15);
        let sumand_2 = self.circuit_builder.add_module_32_bits(&sigma_0, w_t_16);
        self.circuit_builder
            .add_module_32_bits(&sumand_1, &sumand_2)
    }

    fn compression_function_iteration(
        &mut self,
        s: CompressionIterationState,
        w_t: &BinaryDigitsTarget,
        k_t: &BinaryDigitsTarget,
    ) -> CompressionIterationState {
        let [a, b, c, d, e, f, g, h] = s.unpack();
        let sigma_1 = self.sigma_1(&e);
        let majority = self.majority(&e, &f, &g);
        let sumand_aux = self.circuit_builder.add_module_32_bits(k_t, w_t);
        let sumand_1 = self.circuit_builder.add_module_32_bits(&h, &sigma_1);
        let sumand_2 = self
            .circuit_builder
            .add_module_32_bits(&majority, &sumand_aux);
        let t_1 = self
            .circuit_builder
            .add_module_32_bits(&sumand_1, &sumand_2);

        let sigma_0 = self.sigma_0(&a);
        let majority_2 = self.majority(&a, &b, &c);
        let t_2 = self
            .circuit_builder
            .add_module_32_bits(&sigma_0, &majority_2);
        CompressionIterationState {
            a: self.circuit_builder.add_module_32_bits(&t_1, &t_2),
            b: a,
            c: b,
            d: c,
            e: self.circuit_builder.add_module_32_bits(&d, &t_1),
            f: e,
            g: f,
            h: g,
        }
    }
}
