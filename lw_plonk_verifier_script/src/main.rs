//! A simple script to generate and verify the proof of a given program.
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../lw_plonk_verifier/elf/riscv32im-succinct-zkvm-elf");

const PLONK_CIRCUIT: &[u8] =
    include_bytes!("../../lw_plonk_verifier/proving_data/plonk_circuit.bin");
const PLONK_PROOF: &[u8] =
    include_bytes!("../../lw_plonk_verifier/proving_data/plonk_simple_mul.proof");
const PLONK_SRS: &[u8] = include_bytes!("../../lw_plonk_verifier/proving_data/plonk_srs.bin");
const PLONK_VK: &[u8] = include_bytes!("../../lw_plonk_verifier/proving_data/plonk_vk.bin");

fn main() {
    sp1_core::utils::setup_logger();

    let mut stdin = SP1Stdin::new();

    stdin.write_slice(&PLONK_CIRCUIT);
    stdin.write_slice(&PLONK_PROOF);
    stdin.write_slice(&PLONK_SRS);
    stdin.write_slice(&PLONK_VK);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // NOTE(marian): It currently panics if we uncomment this code. We should see why is
    // this happening.
    let plonk_verification_result = proof.stdout.read::<bool>();
    println!(
        "PLONK proof verification result: {}",
        plonk_verification_result
    );

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("SP1 verification failed");
    println!("SP1 proof verified successfully");

    // Save proof.
    let sp1_proof_path = "proof-with-io.json";
    proof.save(sp1_proof_path).expect("saving proof failed");
    println!("Proof saved to {}", sp1_proof_path);
}
