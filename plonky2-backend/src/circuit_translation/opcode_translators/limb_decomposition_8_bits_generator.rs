use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use std::array;

/*impl<const LIMBS: usize> Default for [Target; LIMBS] {
    fn default() -> Self {
        [Target::VirtualTarget {index: 0}; LIMBS]
    }
}*/

#[derive(Debug)]
pub struct LimbDecomposition8BitsGenerator<const LIMBS: usize> {
    pub full_number: Target,
    pub limbs: [Target; LIMBS]
}

impl<F: RichField + Extendable<D>, const D: usize, const LIMBS: usize> SimpleGenerator<F, D>
        for LimbDecomposition8BitsGenerator<LIMBS> {
    fn id(&self) -> String {
        "LimbDecomposition8BitsGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        vec![self.full_number]
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let mut full_number = witness.get_target(self.full_number);
        let mut full_number = full_number.to_canonical_u64();

        for i in 0..LIMBS {
            let current_limb = full_number % 256;
            full_number = full_number / 256;
            out_buffer.set_target(self.limbs[i], F::from_canonical_u64(current_limb));
        }
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_target(self.full_number)?;
        for i in 0..LIMBS {
            dst.write_target(self.limbs[i])?;
        }
        Ok(())
    }

    fn deserialize(src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let full_number = src.read_target()?;
        let limbs: [Target; LIMBS] = array::from_fn(|_| src.read_target().unwrap());
        Ok(Self { full_number, limbs })
    }
}