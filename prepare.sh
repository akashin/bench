#!/bin/bash

cargo build --target wasm32-unknown-unknown --release
mkdir -p out
cp target/wasm32-unknown-unknown/release/cranelift-bench.wasm out/mod.wasm
cd out
wasm-strip mod.wasm
wasm2wat mod.wasm > mod.wat

wc -l mod.wat
