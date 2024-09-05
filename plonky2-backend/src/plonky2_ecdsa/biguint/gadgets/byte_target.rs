use plonky2::iop::target::BoolTarget;

#[derive(Clone, Debug)]
pub struct ByteTarget {
    pub bits: Vec<BoolTarget>,
}

impl Default for ByteTarget {
    fn default() -> Self {
        Self {
            bits: Default::default(),
        }
    }
}