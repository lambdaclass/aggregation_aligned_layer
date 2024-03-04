#![no_main]
sp1_zkvm::entrypoint!(main);

use ark_bn254::{Bn254, FrConfig};
use ark_crypto_primitives::snark::SNARK;
use ark_ff::{Fp, MontBackend};
use ark_groth16::{Groth16, PreparedVerifyingKey, Proof};
use ark_serialize::*;

fn main() {
    let mut proof_bytes = [0u8; 256];
    sp1_zkvm::io::read_slice(&mut proof_bytes);
    let mut pub_input_bytes = [0u8; 32];
    sp1_zkvm::io::read_slice(&mut pub_input_bytes);
    let mut pvk_bytes = [0u8; 35930];
    sp1_zkvm::io::read_slice(&mut pvk_bytes);

    let proof = Proof::deserialize_uncompressed(proof_bytes.as_slice()).unwrap();
    let image: Fp<MontBackend<FrConfig, 4>, 4> =
        Fp::deserialize_uncompressed(pub_input_bytes.as_slice()).unwrap();
    let pvk = PreparedVerifyingKey::deserialize_uncompressed(pvk_bytes.as_slice()).unwrap();

    let verification_result =
        Groth16::<Bn254>::verify_with_processed_vk(&pvk, &[image], &proof).unwrap();

    // We write the result of the verification into the output.
    sp1_zkvm::io::write::<bool>(&verification_result);
}
