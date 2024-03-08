//! A simple script to generate and verify the proof of a given program.
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../risc0_fibonacci_verifier/elf/riscv32im-succinct-zkvm-elf");
const RISC0_FIB_RECEIPT: &[u8] =
    include_bytes!("../../risc0_fibonacci_verifier/proving_data/risc0_fibo.proof");

fn main() {
    sp1_core::utils::setup_logger();

    let mut stdin = SP1Stdin::new();
    println!("RECEIPT LEN: {}", RISC0_FIB_RECEIPT.len());
    println!("ELF LEN: {}", ELF.len());

    stdin.write_slice(RISC0_FIB_RECEIPT);

    println!("Starting SP1 proof generation...");
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    let receipt_verification_result = proof.stdout.read::<bool>();
    println!(
        "RISC0 fibonacci receipt verification result: {}",
        receipt_verification_result
    );

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("SP1 verification failed");
    println!("SP1 proof verified successfully");

    // Save proof.
    let sp1_proof_path = "risc0-fib-proof-with-io.json";
    proof.save(sp1_proof_path).expect("saving proof failed");
    println!("Proof saved to {}", sp1_proof_path);
}
