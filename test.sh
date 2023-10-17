sudo ../../Downloads/dist/cargo-clif b --release
./target/release/cranelift-bench >no-wasm.txt

sudo cargo build --target wasm32-wasi --release
wasmtime compile target/wasm32-wasi/release/cranelift-bench.wasm -o compiled-wasm
wasmtime compiled-wasm --allow-precompiled >with-wasm.txt
