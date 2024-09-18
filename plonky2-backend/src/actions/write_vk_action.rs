use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::{impl_gate_serializer, read_gate_impl, get_gate_tag_impl};
use plonky2::gates::arithmetic_base::ArithmeticGate;
use plonky2::gates::arithmetic_extension::ArithmeticExtensionGate;
use plonky2::gates::base_sum::BaseSumGate;
use plonky2::gates::constant::ConstantGate;
use plonky2::gates::coset_interpolation::CosetInterpolationGate;
use plonky2::gates::exponentiation::ExponentiationGate;
use plonky2::gates::lookup::LookupGate;
use plonky2::gates::lookup_table::LookupTableGate;
use plonky2::gates::multiplication_extension::MulExtensionGate;
use plonky2::gates::noop::NoopGate;
use plonky2::gates::poseidon::PoseidonGate;
use plonky2::gates::poseidon_mds::PoseidonMdsGate;
use plonky2::gates::public_input::PublicInputGate;
use plonky2::gates::random_access::RandomAccessGate;
use plonky2::gates::reducing::ReducingGate;
use plonky2::gates::reducing_extension::ReducingExtensionGate;
use plonky2::util::serialization::GateSerializer;
use crate::plonky2_ecdsa::biguint::gates::add_many_u32::U32AddManyGate;
use crate::plonky2_ecdsa::biguint::gates::arithmetic_u32::U32ArithmeticGate;
use crate::plonky2_ecdsa::biguint::gates::comparison::ComparisonGate;
use crate::plonky2_ecdsa::biguint::gates::range_check_u32::U32RangeCheckGate;
use crate::plonky2_ecdsa::biguint::gates::subtraction_u32::U32SubtractionGate;
use super::*;

/// The Write Verification Key Action will translate the ACIR circuit into the Plonky2 circuit
/// (again) and write the necessary data for the Verifier to verify the computation.
pub struct WriteVKAction {
    pub acir_program_json_path: String,
    pub vk_path_output: String,
}

pub struct BackendGateSerializer;
impl<F: RichField + Extendable<D>, const D: usize> GateSerializer<F, D> for BackendGateSerializer {
    impl_gate_serializer! {
            DefaultGateSerializer,
            ArithmeticGate,
            ArithmeticExtensionGate<D>,
            BaseSumGate<2>,
            BaseSumGate<4>,
            ConstantGate,
            CosetInterpolationGate<F, D>,
            ExponentiationGate<F, D>,
            LookupGate,
            LookupTableGate,
            MulExtensionGate<D>,
            NoopGate,
            PoseidonMdsGate<F, D>,
            PoseidonGate<F, D>,
            PublicInputGate,
            RandomAccessGate<F, D>,
            ReducingExtensionGate<D>,
            ReducingGate<D>,
            ComparisonGate<F, D>,
            U32AddManyGate<F,D>,
            U32ArithmeticGate<F,D>,
            U32RangeCheckGate<F,D>,
            U32SubtractionGate<F,D>
        }
}

impl WriteVKAction {
    pub fn run(&self) {
        let acir_program: Program =
            deserialize_program_within_file_path(&self.acir_program_json_path);
        let acir_circuit = &acir_program.functions[0];
        let mut translator = CircuitBuilderFromAcirToPlonky2::new();
        translator.translate_circuit(acir_circuit);
        let CircuitBuilderFromAcirToPlonky2 { builder, .. } = translator;
        let plonky2_circuit = builder.build::<C>();
        let verifier_data = plonky2_circuit.verifier_data();
        let gate_serializer = BackendGateSerializer;
        let serialized_verifier_data = verifier_data.to_bytes(&gate_serializer).unwrap();
        write_bytes_to_file_path(serialized_verifier_data, &self.vk_path_output);
    }
}
