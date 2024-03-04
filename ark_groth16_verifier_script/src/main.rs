//! A simple script to generate and verify the proof of a given program.
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../ark_groth16_verifier/elf/riscv32im-succinct-zkvm-elf");

const G16_PROOF: &[u8] =
    include_bytes!("../../ark_groth16_verifier/proving_data/mimc_groth16.proof");
const PVK: &[u8] = include_bytes!("../../ark_groth16_verifier/proving_data/groth16_pvk.bin");
const PUB_INPUT: &[u8] =
    include_bytes!("../../ark_groth16_verifier/proving_data/groth16_pub_input.bin");

fn main() {
    sp1_core::utils::setup_logger();

    let mut stdin = SP1Stdin::new();

    stdin.write_slice(&G16_PROOF);
    stdin.write_slice(&PUB_INPUT);
    stdin.write_slice(&PVK);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    let groth16_verification_result = proof.stdout.read::<bool>();
    println!(
        "Groth16 proof verification result: {}",
        groth16_verification_result
    );

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("SP1 verification failed");
    println!("SP1 proof verified successfully");

    // Save proof.
    let sp1_proof_path = "groth16-proof-with-io.json";
    proof.save(sp1_proof_path).expect("saving proof failed");
    println!("Proof saved to {}", sp1_proof_path);
}
