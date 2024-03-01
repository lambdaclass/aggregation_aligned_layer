//! A simple script to generate and verify the proof of a given program.
use lambdaworks_math::{
    elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrField,
    traits::Deserializable,
};
use lambdaworks_plonk::{prover::Proof, test_utils::utils::KZG};
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
// const PLONK_PROOF: &[u8] = include_bytes!("../../program/proving_data/plonk_simple_mul.proof");
const CAIRO_PROOF: &[u8] = include_bytes!("../../program/proving_data/fibonacci_5.proof");

fn main() {
    sp1_core::utils::setup_logger();

    // let proof_file_path = "../program/proving_data/plonk_simple_mul.proof";
    // let proof_bytes = std::fs::read(proof_file_path).expect("Could not read proof file");

    // let _plonk_proof: Proof<FrField, KZG> =
    //     lambdaworks_plonk::prover::Proof::deserialize(&PLONK_PROOF).unwrap();

    let mut stdin = SP1Stdin::new();
    // The paths where the PLONK proof and verification key should be found
    // let vk_file_path = "../program/proving_data/plonk_vk.bin";

    // println!("PLONK PROOF LEN: {}", PLONK_PROOF.len());

    // stdin.write(&proof_file_path);
    stdin.write_slice(&CAIRO_PROOF);
    // stdin.write(&vk_file_path);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // NOTE(marian): It currently panics if we uncomment this code. We should see why is
    // this happening.
    // let plonk_verification_result = proof.stdout.read::<bool>();
    // println!(
    //     "PLONK proof verification result: {}",
    //     plonk_verification_result
    // );

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("SP1 verification failed");
    println!("SP1 proof verified successfully");

    let verification_result = proof.stdout.read::<bool>();

    println!("CAIRO verification result: {}", verification_result);
    // Save proof.
    // let sp1_proof_path = "proof-with-io.json";
    // proof.save(sp1_proof_path).expect("saving proof failed");
    // println!("Proof saved to {}", sp1_proof_path);
}
