#![no_main]
sp1_zkvm::entrypoint!(main);

use ark_bls12_377::{Bls12_377, FrConfig};
use ark_crypto_primitives::snark::SNARK;
use ark_ff::{Fp, MontBackend};
use ark_groth16::{Groth16, PreparedVerifyingKey, Proof};
use ark_serialize::*;

fn main() {
    let mut proof_bytes = [0u8; 1620];
    sp1_zkvm::io::read_slice(&mut proof_bytes);
    let mut pub_input_bytes = [0u8; 1596];
    sp1_zkvm::io::read_slice(&mut pub_input_bytes);
    let mut pvk_bytes = [0u8; 1152];
    sp1_zkvm::io::read_slice(&mut pvk_bytes);

    let proof = Proof::deserialize_uncompressed(proof_bytes.as_slice()).unwrap();
    let image: Fp<MontBackend<FrConfig, 4>, 4> =
        Fp::deserialize_uncompressed(pub_input_bytes.as_slice()).unwrap();
    let pvk = PreparedVerifyingKey::deserialize_uncompressed(pvk_bytes.as_slice()).unwrap();

    let verification_result =
        Groth16::<Bls12_377>::verify_with_processed_vk(&pvk, &[image], &proof).unwrap();

    // Circuit for the program:
    //  public input x
    //  public input y
    //  private input e
    //  z = x * e
    //  assert y == z
    // let circuit = test_common_preprocessed_input_1();
    // let circuit: CommonPreprocessedInput<FrField> =
    //     bincode::deserialize(&circuit_bytes).expect("Could not deserialize circuit");

    // let srs: StructuredReferenceString<
    //     ShortWeierstrassProjectivePoint<BLS12381Curve>,
    //     ShortWeierstrassProjectivePoint<BLS12381TwistCurve>,
    // > = StructuredReferenceString::deserialize(&srs_bytes).expect("Could not deserialize SRS");

    // let kzg = KZG::new(srs);
    // let x: FrElement = FrElement::from(4_u64);
    // let y: FrElement = FrElement::from(12_u64);
    // let pub_input = vec![x, y];

    // let proof: Proof<FrField, KZG> =
    //     Proof::deserialize(&proof_bytes).expect("Could not deserialize PLONK proof");

    // let vk: VerificationKey<ShortWeierstrassProjectivePoint<BLS12381Curve>> =
    //     VerificationKey::deserialize(&vk_bytes)
    //         .expect("Could not deserialize PLONK verification key");

    // let verifier = Verifier::new(kzg);
    // let verification_result = verifier.verify(&proof, &pub_input, &circuit, &vk);

    // We write the result of the verification into the output.
    sp1_zkvm::io::write::<bool>(&verification_result);
}
