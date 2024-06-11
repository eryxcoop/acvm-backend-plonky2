use super::*;

pub struct Sha256Translator<'a> {
    circuit_builder: &'a mut CircuitBuilderFromAcirToPlonky2,
    witness_target_map: &'a mut HashMap<Witness, Target>,
    inputs: &Vec<FunctionInput>,
    outputs: Box<[Witness; 32]>
}

impl<'a> Sha256Translator<'a> {
    pub fn new_for(circuit_builder: &'a mut CircuitBuilderFromAcirToPlonky2,
                   witness_target_map: &'a mut HashMap<Witness, Target>,
                   inputs: &Vec<FunctionInput>,
                   outputs: Box<[Witness; 32]>) -> Sha256Translator<'a> {
        Self { circuit_builder, witness_target_map, inputs, outputs }
    }

    pub fn translate(&mut self) {
        eprintln!("----------SHA256--------");
        let M = vec![];
        let input_bytes_0 = vec![self.inputs[4*i], self.inputs[4*i+1], self.inputs[4*i+2], self.inputs[4*i+3]];
        let M_0 = self.binary_digit_of_32_bits_from_input(input_bytes_0);
        M.push(M_0);
        for i in 0..14 {
            // Fill with zeroes
            M.push(self.binary_digit_of_32_bits_from_input(vec![]));
        }
        // Size is 4
        let bit_targets = vec![BoolTarget(); 32];
        self.circuit_builder.builder.connect(bit_targets[30],
                                             self.circuit_builder.builder.constant(F::from_canonical_u8(1)));
        M.push(BinaryDigitsTarget {bits: bit_targets});
    }

    fn binary_digit_of_32_bits_from_input(&mut self, witness_bytes: Vec<Witness>) {
        let targets = witness_bytes.iter().map(|witness| self.circuit_builder._get_or_create_target_for_witness(witness)).collect();
        let byte_targets = targets.iter().map(|target| self.circuit_builder.convert_number_to_binary_number(target, 8)).collect();
        let bit_targets = vec![BoolTarget(); 32];

        for idx_byte in 0..witness_bytes.len() {
            for idx_bit in 0..8 {
                self.circuit_builder.builder.connect(bit_targets[i], byte_targets[idx_byte].bits[idx_bit]);
            }
        }
        for idx_byte in witness_bytes.len()..4 {
            for idx_bit in 0..8 {
                self.circuit_builder.builder.connect(bit_targets[i], self.circuit_builder.builder.constant(F::from_canonical_u8(0)));
            }
        }

        let binary_digits_target = BinaryDigitsTarget { bits: bit_targets };
    }

    fn sigma_0(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = target.rotate_right(7);
        let x2 = target.rotate_right(18);
        let x3 = target.shift_right(3);

        let y1 = self.circuit_builder.xor(x1, x2);
        let y2 = self.circuit_builder.xor(y1, x3);

        y2
    }

    fn sigma_1(&mut self, target: &BinaryDigitsTarget) -> BinaryDigitsTarget {
        let x1 = target.rotate_right(17);
        let x2 = target.rotate_right(19);
        let x3 = target.shift_right(10);

        let y1 = self.circuit_builder.xor(x1, x2);
        let y2 = self.circuit_builder.xor(y1, x3);

        y2
    }

    //         h = ['0x6a09e667', '0xbb67ae85', '0x3c6ef372', '0xa54ff53a', '0x510e527f', '0x9b05688c', '0x1f83d9ab', '0x5be0cd19']
    //         k = ['0x428a2f98', '0x71374491', '0xb5c0fbcf', '0xe9b5dba5', '0x3956c25b', '0x59f111f1', '0x923f82a4','0xab1c5ed5', '0xd807aa98', '0x12835b01', '0x243185be', '0x550c7dc3', '0x72be5d74', '0x80deb1fe','0x9bdc06a7', '0xc19bf174', '0xe49b69c1', '0xefbe4786', '0x0fc19dc6', '0x240ca1cc', '0x2de92c6f','0x4a7484aa', '0x5cb0a9dc', '0x76f988da', '0x983e5152', '0xa831c66d', '0xb00327c8', '0xbf597fc7','0xc6e00bf3', '0xd5a79147', '0x06ca6351', '0x14292967', '0x27b70a85', '0x2e1b2138', '0x4d2c6dfc','0x53380d13', '0x650a7354', '0x766a0abb', '0x81c2c92e', '0x92722c85', '0xa2bfe8a1', '0xa81a664b','0xc24b8b70', '0xc76c51a3', '0xd192e819', '0xd6990624', '0xf40e3585', '0x106aa070', '0x19a4c116','0x1e376c08', '0x2748774c', '0x34b0bcb5', '0x391c0cb3', '0x4ed8aa4a', '0x5b9cca4f', '0x682e6ff3','0x748f82ee', '0x78a5636f', '0x84c87814', '0x8cc70208', '0x90befffa', '0xa4506ceb', '0xbef9a3f7','0xc67178f2']
}