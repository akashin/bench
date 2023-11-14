# Cranelift bench
In this repo I want to measure performance difference between code compiled via:
1. Rust -> aarch64
2. Rust -> Cranelift IR -> aarch64
3. Rust -> Wasm -> Cranelift IR -> aarch64

## Workloads
Currently matrix multiplication and monte-carlo pi calculation only. I think with such results it is enought.

# How to reproduce
Below are steps to reproduce each of the methods

## Direct compilation using `rustc`
```sh
cargo run --release > results/rustc.txt
```

## Using `rustc_codegen_cranelift`

```sh
# Download and unpack compiler from here https://github.com/bjorn3/rustc_codegen_cranelift/releases/tag/dev
wget https://github.com/rust-lang/rustc_codegen_cranelift/releases/download/dev/cg_clif-x86_64-apple-darwin.tar.xz
tar -xf cg_clif-x86_64-apple-darwin.tar.xz

# Compile the project with rustc_codegen_cranelift and run the benchmark
dist/cargo-clif run --release > results/rustc_cranelift.txt
```

## Using Rustc WASM target

```sh
# Init cargo for compiling to wasm
rustup target add wasm32-wasi

# Compile Rust to wasm
cargo build --target wasm32-wasi --release

# Install wasmtime
brew install wasmtime

# Precompile wasm via cranelift
wasmtime compile target/wasm32-wasi/release/cranelift-bench.wasm -o compiled-wasm

# Run precompiled wasm:
wasmtime compiled-wasm --allow-precompiled > results/wasm_cranelift.txt
```

# Results
On my machine (apple m2):
|                      | Rust -> CLIF    | Rust -> Wasm -> Clif  |
|----------------------|-----------------|-----------------------|
| monte-carlo-pi       |  562.755252666s | 13.702631166s         |
| matrix-multiplication| 16.245338166s   | 4.367239s             |
