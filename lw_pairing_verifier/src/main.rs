#![no_main]
sp1_zkvm::entrypoint!(main);

use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::pairing::BLS12381AtePairing;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::twist::BLS12381TwistCurve;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::elliptic_curve::traits::IsPairing;
use lambdaworks_math::field::element::FieldElement;

fn main() {
    let p = BLS12381Curve::generator();
    let q = BLS12381TwistCurve::generator();
    // let a = U384::from_u64(11);
    // let b = U384::from_u64(93);

    let result = BLS12381AtePairing::compute_batch(&[
        (
            &p,
            &q, // &p.operate_with_self(a).to_affine(),
               // &q.operate_with_self(b).to_affine(),
        ),
        (
            // &p.operate_with_self(a * b).to_affine(),
            // &q.neg().to_affine(),
            &p,
            &q.neg(),
        ),
    ])
    .unwrap();

    let verification_result = result == FieldElement::one();
    // let verification_result = true;

    // We write the result of the verification into the output.
    sp1_zkvm::io::write::<bool>(&verification_result);
}
