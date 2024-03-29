#![no_main]
sp1_zkvm::entrypoint!(main);

use lambdaworks_crypto::commitments::kzg::StructuredReferenceString;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrElement;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrField;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::twist::BLS12381TwistCurve;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use lambdaworks_math::traits::Deserializable;
use lambdaworks_plonk::prover::Proof;
use lambdaworks_plonk::setup::CommonPreprocessedInput;
use lambdaworks_plonk::setup::VerificationKey;
use lambdaworks_plonk::test_utils::utils::KZG;
use lambdaworks_plonk::verifier::Verifier;

fn main() {
    let mut circuit_bytes = [0u8; 1944];
    sp1_zkvm::io::read_slice(&mut circuit_bytes);
    let mut proof_bytes = [0u8; 1620];
    sp1_zkvm::io::read_slice(&mut proof_bytes);
    let mut srs_bytes = [0u8; 1596];
    sp1_zkvm::io::read_slice(&mut srs_bytes);
    let mut vk_bytes = [0u8; 1152];
    sp1_zkvm::io::read_slice(&mut vk_bytes);

    // Circuit for the program:
    //  public input x
    //  public input y
    //  private input e
    //  z = x * e
    //  assert y == z
    // let circuit = test_common_preprocessed_input_1();
    let circuit: CommonPreprocessedInput<FrField> =
        bincode::deserialize(&circuit_bytes).expect("Could not deserialize circuit");

    let srs: StructuredReferenceString<
        ShortWeierstrassProjectivePoint<BLS12381Curve>,
        ShortWeierstrassProjectivePoint<BLS12381TwistCurve>,
    > = StructuredReferenceString::deserialize(&srs_bytes).expect("Could not deserialize SRS");

    let kzg = KZG::new(srs);
    let x: FrElement = FrElement::from(4_u64);
    let y: FrElement = FrElement::from(12_u64);
    let pub_input = vec![x, y];

    let proof: Proof<FrField, KZG> =
        Proof::deserialize(&proof_bytes).expect("Could not deserialize PLONK proof");

    let vk: VerificationKey<ShortWeierstrassProjectivePoint<BLS12381Curve>> =
        VerificationKey::deserialize(&vk_bytes)
            .expect("Could not deserialize PLONK verification key");

    let verifier = Verifier::new(kzg);
    let verification_result = verifier.verify(&proof, &pub_input, &circuit, &vk);

    // We write the result of the verification into the output.
    sp1_zkvm::io::write::<bool>(&verification_result);
}
