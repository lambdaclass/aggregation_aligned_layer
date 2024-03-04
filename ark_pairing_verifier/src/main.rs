#![no_main]
sp1_zkvm::entrypoint!(main);

use ark_ec::{pairing::Pairing, AffineRepr};
// use ark_std::UniformRand;
use ark_bls12_381::Bls12_381;
use ark_ec::bls12;

pub type G1Affine = bls12::G1Affine<ark_bls12_381::Config>;
pub type G2Affine = bls12::G2Affine<ark_bls12_381::Config>;

fn main() {
    let a = G1Affine::generator();
    let b = G2Affine::generator();

    // We can compute the pairing of two points on the curve, either monolithically...
    let e1 = Bls12_381::pairing(a, b);
    // ... or in two steps. First, we compute the Miller loop...
    let ml_result = Bls12_381::miller_loop(a, b);
    // ... and then the final exponentiation.
    let e2 = Bls12_381::final_exponentiation(ml_result).unwrap();

    let verification_result = e1 == e2;

    // We write the result of the verification into the output.
    sp1_zkvm::io::write::<bool>(&verification_result);
}
