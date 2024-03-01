#![no_main]
sp1_zkvm::entrypoint!(main);

use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrElement;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrField;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use lambdaworks_math::traits::Deserializable;
use lambdaworks_plonk::prover::Proof;
use lambdaworks_plonk::setup::VerificationKey;
use lambdaworks_plonk::test_utils::circuit_1::test_common_preprocessed_input_1;
use lambdaworks_plonk::test_utils::utils::test_srs;
use lambdaworks_plonk::test_utils::utils::KZG;
use lambdaworks_plonk::verifier::Verifier;

fn main() {
    let proof_path = sp1_zkvm::io::read::<String>();
    let vk_path = sp1_zkvm::io::read::<String>();

    // Circuit for the program:
    //  public input x
    //  public input y
    //  private input e
    //  z = x * e
    //  assert y == z
    let circuit = test_common_preprocessed_input_1();

    let srs = test_srs(circuit.n);
    let kzg = KZG::new(srs);

    let x: FrElement = FrElement::from(4_u64);
    let y: FrElement = FrElement::from(12_u64);
    let pub_input = vec![x, y];

    let proof_bytes = std::fs::read(proof_path).expect("Could not read proof file");
    let proof: Proof<FrField, KZG> =
        Proof::deserialize(&proof_bytes).expect("Could not deserialize PLONK proof");

    let vk_bytes = std::fs::read(vk_path).expect("Could not read verification key file");
    let vk: VerificationKey<ShortWeierstrassProjectivePoint<BLS12381Curve>> =
        VerificationKey::deserialize(&vk_bytes)
            .expect("Could not deserialize PLONK verification key");

    let verifier = Verifier::new(kzg);
    let verification_result = verifier.verify(&proof, &pub_input, &circuit, &vk);

    // We write the result of the verification into the output.
    sp1_zkvm::io::write::<bool>(&verification_result);
}
