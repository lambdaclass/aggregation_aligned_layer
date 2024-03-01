# SP1 over PLONK

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


## Build Lambdaworks PLONK verifier program

Once the SP1 toolchain is installed, move to the `program` folder and compile to the SP1 zkVM target

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
