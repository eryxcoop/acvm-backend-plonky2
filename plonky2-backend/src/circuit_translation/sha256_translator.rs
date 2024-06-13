use super::*;

pub struct Sha256Translator<'a> {
    circuit_builder: &'a mut CircuitBuilderFromAcirToPlonky2,
    inputs: &'a Vec<FunctionInput>,
    outputs: &'a Box<[Witness; 32]>
}

impl<'a> Sha256Translator<'a> {
    pub fn new_for(circuit_builder: &'a mut CircuitBuilderFromAcirToPlonky2,
                   inputs: &'a Vec<FunctionInput>,
                   outputs: &'a Box<[Witness; 32]>) -> Sha256Translator<'a> {
        Self { circuit_builder, inputs, outputs }
    }

    pub fn translate(&mut self) {
        eprintln!("----------SHA256--------");
        self._register_targets_for_output_witnesses();

        let mut m: Vec<BinaryDigitsTarget> = vec![];
        let input_bytes_0 = vec![self.inputs[0], self.inputs[1], self.inputs[2], self.inputs[3]];
        let m_0 = self.binary_digit_of_32_bits_from_witnesses(self._extract_witnesses(input_bytes_0));
        M.push(m_0);
        for _ in 0..14 {
            // Fill with zeroes
            M.push(self.circuit_builder.binary_number_target_for_constant(0, 32));
        }
        // Size is 4
        let binary_digits_target = self.circuit_builder.binary_number_target_for_constant(4, 32);
        M.push(binary_digits_target);
    }

    fn _register_targets_for_output_witnesses(&mut self) {
        for output in self.outputs.iter() {
            self._get_or_create_target_for_witness(*output);
        }
    }

    fn binary_digit_of_32_bits_from_witnesses(&mut self, witness_bytes: Vec<Witness>) -> BinaryDigitsTarget {
        let targets: Vec<Target> = witness_bytes.into_iter().map(|witness| self.circuit_builder._get_or_create_target_for_witness(witness)).collect();
        let byte_targets: Vec<BinaryDigitsTarget> = targets.iter().map(|target| self.circuit_builder.convert_number_to_binary_number(*target, 8)).collect();

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
        let x1 = self.circuit_builder.rotate_right(target,7);
        let x2 = self.circuit_builder.rotate_right(target,18);
        let x3 = self.circuit_builder.shift_right(target,3);

        let y1 = self.circuit_builder.xor(x1, x2);
        let y2 = self.circuit_builder.xor(y1, x3);

        y2
    }

    fn sigma_1(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = self.circuit_builder.rotate_right(target,17);
        let x2 = self.circuit_builder.rotate_right(target,19);
        let x3 = self.circuit_builder.shift_right(target,10);

        let y1 = self.circuit_builder.xor(x1, x2);
        let y2 = self.circuit_builder.xor(y1, x3);

        y2
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
                self.circuit_builder.witness_target_map.insert(witness, target);
                target
            }
        }
    }

    //         h = ['0x6a09e667', '0xbb67ae85', '0x3c6ef372', '0xa54ff53a', '0x510e527f', '0x9b05688c', '0x1f83d9ab', '0x5be0cd19']
    //         k = ['0x428a2f98', '0x71374491', '0xb5c0fbcf', '0xe9b5dba5', '0x3956c25b', '0x59f111f1', '0x923f82a4','0xab1c5ed5', '0xd807aa98', '0x12835b01', '0x243185be', '0x550c7dc3', '0x72be5d74', '0x80deb1fe','0x9bdc06a7', '0xc19bf174', '0xe49b69c1', '0xefbe4786', '0x0fc19dc6', '0x240ca1cc', '0x2de92c6f','0x4a7484aa', '0x5cb0a9dc', '0x76f988da', '0x983e5152', '0xa831c66d', '0xb00327c8', '0xbf597fc7','0xc6e00bf3', '0xd5a79147', '0x06ca6351', '0x14292967', '0x27b70a85', '0x2e1b2138', '0x4d2c6dfc','0x53380d13', '0x650a7354', '0x766a0abb', '0x81c2c92e', '0x92722c85', '0xa2bfe8a1', '0xa81a664b','0xc24b8b70', '0xc76c51a3', '0xd192e819', '0xd6990624', '0xf40e3585', '0x106aa070', '0x19a4c116','0x1e376c08', '0x2748774c', '0x34b0bcb5', '0x391c0cb3', '0x4ed8aa4a', '0x5b9cca4f', '0x682e6ff3','0x748f82ee', '0x78a5636f', '0x84c87814', '0x8cc70208', '0x90befffa', '0xa4506ceb', '0xbef9a3f7','0xc67178f2']
}