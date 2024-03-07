#![no_main]
sp1_zkvm::entrypoint!(main);

use risc0_zkvm::Receipt;

pub const FIBONACCI_GUEST_ID: [u32; 8] = [
    3415224240, 3079988862, 1684040742, 2802764917, 936974713, 2148640136, 196309035, 667738874,
];

fn main() {
    let mut receipt_bytes = [0u8; 215514];
    sp1_zkvm::io::read_slice(&mut receipt_bytes);

    let receipt: Receipt = bincode::deserialize(&receipt_bytes).unwrap();
    let verification_result = receipt.verify(FIBONACCI_GUEST_ID).is_ok();

    sp1_zkvm::io::write::<bool>(&verification_result);
}
