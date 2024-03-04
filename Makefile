.PHONY: build_ark_pairing_program

build_ark_pairing_program:
	cd ark_pairing_verifier && cargo prove build

sp1_prove_ark_pairing: build_ark_pairing_program
	cd ark_pairing_verifier_script && RUST_LOG=info cargo run --release
