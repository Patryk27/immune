#!/usr/bin/env bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/unfair_advantage.wasm
zip immune assets/* assets/tutorial/* levels/* out/* index.html