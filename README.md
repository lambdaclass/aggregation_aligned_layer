# SP1 zk-verifiers 

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

## Arkworks pairing verification
### Build the arkworks pairing verification program

Once the SP1 toolchain is installed, move to the `program` folder and compile to the SP1 zkVM target

```bash
make build_ark_pairing_program 
```

### Generate proof of the pairing verification

```bash
make sp1_prove_ark_pairing
```

## Lambdaworks pairing verification
### Build the arkworks pairing verification program
```bash
make build_lw_pairing_program 
```

### Generate proof of the pairing verification

```bash
make sp1_prove_lw_pairing
```

## Arkworks Groth16 verifier (WIP)

## Lambdaworks PLONK verifier

### Build Lambdaworks PLONK verifier program

```bash
make build_lw_plonk_program
```

### Generate SP1 proof of PLONK verifier

```bash
make sp1_prove_lw_plonk
```

## Acknoledgements

Thanks to [SuccintLabs](https://succinct.xyz/) for the development of the [SP1 zkVM](https://github.com/succinctlabs/sp1) which was used for this proof of concept.
