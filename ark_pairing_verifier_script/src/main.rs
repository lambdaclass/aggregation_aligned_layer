//! A simple script to generate and verify the proof of a given program.
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] =
    include_bytes!("../../pairing_verifier_arkworks/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    sp1_core::utils::setup_logger();

    let stdin = SP1Stdin::new();
    println!("Starting sp1 proof generation...");
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    let pairing_verification_result = proof.stdout.read::<bool>();
    println!(
        "Pairing proof verification result: {}",
        pairing_verification_result
    );

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("SP1 verification failed");
    println!("SP1 proof verified successfully");

    // Save proof.
    let sp1_proof_path = "ark-pairing-proof-with-io.json";
    proof.save(sp1_proof_path).expect("saving proof failed");
    println!("Proof saved to {}", sp1_proof_path);
}
