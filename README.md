# Aggregataion Aligned Layer 

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

## Arkworks BN254 pairing verification
### Build the arkworks pairing verification program

```bash
make build_ark_bn254_pairing_program 
```

### Generate proof of the pairing verification

```bash
make sp1_prove_ark_bn254_pairing
```

## Arkworks BLS12-381 pairing verification
### Build the arkworks pairing verification program

```bash
make build_ark_pairing_program 
```

### Generate proof of the pairing verification

```bash
make sp1_prove_ark_pairing
```

## Arkworks BN254 Groth16 verifier

### Build the arkworks Groth16 verifier program 

```bash
make build_ark_groth16_verifier 
```

### Generate proof of the Groth16 verification 

```bash
make sp1_prove_ark_groth16
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
