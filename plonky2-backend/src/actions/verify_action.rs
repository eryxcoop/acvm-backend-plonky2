use super::*;

/// The verify action will receive a proof path and a verification key path, and verify the program
/// execution using those values.
pub struct VerifyAction {
    pub proof_path: String,
    pub vk_path: String,
}

impl VerifyAction {
    pub fn run(&self) {
        let verifier_data = deserialize_verifying_key_within_file_path(&self.vk_path);
        let compressed_proof = deserialize_proof_within_file_path(&self.proof_path, &verifier_data);
        verifier_data
            .verify_compressed(compressed_proof)
            .expect("Verification failed");
    }
}
