# Direct compilation
cargo run --release > results/rustc.txt

# Using rustc_cranelift
dist/cargo-clif run --release > results/rustc_cranelift.txt

# Using WASM cranelift
cargo build --target wasm32-wasi --release
wasmtime compile target/wasm32-wasi/release/cranelift-bench.wasm -o compiled-wasm
wasmtime compiled-wasm --allow-precompiled > results/wasm_cranelift.txt
