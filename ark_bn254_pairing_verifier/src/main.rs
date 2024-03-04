#![no_main]
sp1_zkvm::entrypoint!(main);

use ark_bn254::Bn254;
use ark_ec::{pairing::Pairing, AffineRepr};

pub type G1Affine = ark_bn254::G1Affine;
pub type G2Affine = ark_bn254::G2Affine;

fn main() {
    let a = G1Affine::generator();
    let b = G2Affine::generator();

    // We can compute the pairing of two points on the curve, either monolithically...
    let e1 = Bn254::pairing(a, b);
    // ... or in two steps. First, we compute the Miller loop...
    let ml_result = Bn254::miller_loop(a, b);
    // ... and then the final exponentiation.
    let e2 = Bn254::final_exponentiation(ml_result).unwrap();

    let verification_result = e1 == e2;

    // We write the result of the verification into the output.
    sp1_zkvm::io::write::<bool>(&verification_result);
}
