use plonky2::iop::target::BoolTarget;

#[derive(Clone, Debug)]
pub struct BinaryDigitsTarget {
    pub bits: Vec<BoolTarget>,
}
