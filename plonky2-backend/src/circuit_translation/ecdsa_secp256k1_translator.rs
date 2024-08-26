use super::*;

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
        todo!("Implementar ecdsadfssdds");
        // Calcular el inverso de s


        // Calcular R


        // Comparar r con r'

    }
}
