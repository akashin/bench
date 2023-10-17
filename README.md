# Cranelift bench
In this repo I want to measure performance difference between code compiled via
Rust ->         Cranelift IR -> aarch64 VS
Rust -> Wasm -> Cranelift IR -> aarch64

# Method:
Firstly, setup two compilation ways:

## Compilation

### Without wasm step
using https://github.com/bjorn3/rustc_codegen_cranelift compiler

Firstly, you should download and unpack compiler from here https://github.com/bjorn3/rustc_codegen_cranelift/releases/tag/dev

If path to ``dist`` dir you downloaded is ``../../Downloads`` you can just run ``./test.sh`` to compile both ways and run benchmarks (will take time)

Compile rust:
``sudo <path-to-dist-dir-you downloaded>/dist/cargo-clif b --release``

Run benchmark
``./target/release/cranelift-bench >no-wasm.txt``

### With wasm step
Init cargo for compiling to wasm:
``rustup target add wasm32-wasi``
Compile rust to wasm:
``sudo cargo build --target wasm32-wasi --release``
Precompile wasm via cranelift:
``wasmtime compile target/wasm32-wasi/release/cranelift-bench.wasm -o compiled-wasm``
Run precompiled wasm:
``wasmtime compiled-wasm --allow-precompiled >with-wasm.txt``


## Used benchmarks
Currently matrix multiplication and monte-carlo pi calculation only. I think with such results it is enought.

# Results
On my machine (apple m2):
|                      | Rust -> CLIF    | Rust -> Wasm -> Clif  |
|----------------------|-----------------|-----------------------|
| monte-carlo-pi       |  562.755252666s | 13.702631166s         |
| matrix-multiplication| 16.245338166s   | 4.367239s             |
