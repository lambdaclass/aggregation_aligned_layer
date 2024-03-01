#![no_main]
sp1_zkvm::entrypoint!(main);

// use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
// use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrElement;
// use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrField;
// use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
// use lambdaworks_math::traits::Deserializable;
// use lambdaworks_plonk::prover::Proof;
// use lambdaworks_plonk::prover::Prover;
// use lambdaworks_plonk::setup::setup;
// use lambdaworks_plonk::setup::VerificationKey;
// use lambdaworks_plonk::test_utils::circuit_1::test_common_preprocessed_input_1;
// use lambdaworks_plonk::test_utils::circuit_1::test_witness_1;
// use lambdaworks_plonk::test_utils::utils::test_srs;
// use lambdaworks_plonk::test_utils::utils::TestRandomFieldGenerator;
// use lambdaworks_plonk::test_utils::utils::KZG;
use cairo_platinum_prover::air::verify_cairo_proof_ffi;
use lambdaworks_plonk::verifier::Verifier;
use stark_platinum_prover::proof::options::ProofOptions;

fn main() {
    let mut proof_bytes = [0u8; 265809];
    sp1_zkvm::io::read_slice(&mut proof_bytes);

    let proof_options = ProofOptions::default_test_options();
    let verification_result = verify_cairo_proof_ffi(&proof_bytes, &proof_options);

    // let vk_path = sp1_zkvm::io::read::<String>();

    // Circuit for the program:
    //  public input x
    //  public input y
    //  private input e
    //  z = x * e
    //  assert y == z
    // let circuit = test_common_preprocessed_input_1();

    // let srs = test_srs(circuit.n);
    // let kzg = KZG::new(srs);

    // let x: FrElement = FrElement::from(4_u64);
    // let y: FrElement = FrElement::from(12_u64);
    // let pub_input = vec![x.clone(), y];

    // -----
    // let e = FrElement::from(3_u64);
    // let vk = setup(&circuit, &kzg);
    // let random_generator = TestRandomFieldGenerator {};
    // let witness = test_witness_1(x, e);

    // let prover = Prover::new(kzg.clone(), random_generator);
    // let proof = prover.prove(&witness, &pub_input, &circuit, &vk);
    // -----

    // sp1_zkvm::io::write::<u32>(&1);
    // let proof_bytes = std::fs::read(proof_path).expect("Could not read proof file");
    // sp1_zkvm::io::write::<u32>(&2);
    // let proof: Proof<FrField, KZG> =
    //     Proof::deserialize(&proof_bytes).expect("Could not deserialize PLONK proof");
    // sp1_zkvm::io::write::<u32>(&3);

    // let vk_bytes = std::fs::read(vk_path).expect("Could not read verification key file");
    // sp1_zkvm::io::write::<u32>(&4);
    // let vk: VerificationKey<ShortWeierstrassProjectivePoint<BLS12381Curve>> =
    //     VerificationKey::deserialize(&vk_bytes)
    //         .expect("Could not deserialize PLONK verification key");

    // let verifier = Verifier::new(kzg);
    // let verification_result = verifier.verify(&proof, &pub_input, &circuit, &vk);

    // We write the result of the verification into the output.
    sp1_zkvm::io::write::<bool>(&verification_result);
}
