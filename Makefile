build_ark_pairing_program:
	cd ark_pairing_verifier && cargo prove build

sp1_prove_ark_pairing: build_ark_pairing_program
	cd ark_pairing_verifier_script && RUST_LOG=info cargo run --release

build_ark_bn254_pairing_program:
	cd ark_bn254_pairing_verifier && cargo prove build

sp1_prove_ark_bn254_pairing: build_ark_pairing_program
	cd ark_bn254_pairing_verifier_script && RUST_LOG=info cargo run --release

build_ark_groth16_verifier:
	cd ark_groth16_verifier && cargo prove build

sp1_prove_ark_groth16:
	cd ark_groth16_verifier_script && RUST_LOG=info cargo run --release

build_lw_pairing_program:
	cd lw_pairing_verifier && cargo prove build

sp1_prove_lw_pairing: build_lw_pairing_program
	cd lw_pairing_verifier_script && RUST_LOG=info cargo run --release

build_lw_plonk_program:
	cd lw_plonk_verifier && cargo prove build

sp1_prove_lw_plonk: build_lw_plonk_program
	cd lw_plonk_verifier_script && RUST_LOG=info cargo run --release