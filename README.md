# SP1 ZK verifiers 

## SP1 toolchain setup

First of all, install the SP1 toolchain. For more details visit the [sp1 installation guide](https://succinctlabs.github.io/sp1/getting-started/install.html).

```bash
curl -L https://sp1.succinct.xyz | bash
```
This will install sp1up, then simply follow the instructions on-screen, which will make the sp1up command available in your CLI. Then run

```bash
sp1up
```

Verify your installation running

```bash
cargo prove --version
```

# Arkworks pairing verification
## Build the arkworks pairing verification program

Once the SP1 toolchain is installed, move to the `program` folder and compile to the SP1 zkVM target

```bash
make build_ark_pairing_program 
```

## Generate proof of the pairing verification

```bash
make sp1_prove_ark_pairing
```

# Lambdaworks pairing verification
## Build the arkworks pairing verification program
```bash
make build_lw_pairing_program 
```

## Generate proof of the pairing verification

```bash
make sp1_prove_lw_pairing
```

# Arkworks Groth16 verifier (WIP)

# Lambdaworks PLONK verifier

## Build Lambdaworks PLONK verifier program

```bash
cd program
cargo prove build
```

## Generate SP1 proof of PLONK verifier

To generate a proof and save it to disk, go to the `script` folder in the root of the project
and run it

```bash
cd ../script
RUST_LOG=info cargo run --release
```

After a successfull run, the proof will be written into `proof-with-io.json`.

**NOTE**: When reading the PLONK verification result from the program output, the script panics. However, this does not affect the proof generation. For the moment that code is commented until we have more information about what is happening under the hood.

## Acknoledgements

Thanks to [SuccintLabs](https://succinct.xyz/) for the development of the [SP1 zkVM](https://github.com/succinctlabs/sp1) which was used for this proof of concept.
