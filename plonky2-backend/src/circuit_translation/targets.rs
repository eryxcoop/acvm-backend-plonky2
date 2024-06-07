use plonky2::iop::target::BoolTarget;

#[derive(Clone, Debug)]
pub struct BinaryDigitsTarget {
    pub bits: Vec<BoolTarget>,
}

impl BinaryDigitsTarget {
    fn number_of_digits(&self) -> usize {
        self.bits.len()
    }
}
