# rn-sdk

Rust smart contract library designed for Numbat's Andes VM. Also provides a debugging mode with mocks.

# Examples

For examples on how to use the Numbat WASM framework, see https://github.com/NumbatNetwork/sc-examples-rs

# IDE

The framework is designed to be easiest to use with the Numbat IDE VSCode extension: https://marketplace.visualstudio.com/items?itemName=Numbat.vscode-numbat-ide

# Manual build

To build a smart contract without the IDE, run the following command in the contract crate:
```
cargo build --bin wasm --target=wasm32-unknown-unknown --release
```

The resulting .wasm file will be in directory target/wasm32-unknown-unknown/release/wasm.wasm

# Debugging

Step-by-step debugging of smart contracts is possible in VSCode. To do this, it is required to have a separate debug crate and to have tasks.json and launch.json in .vscode properly configured. See https://github.com/NumbatNetwork/sc-examples-rs for examples on how to set this up. 

# Advanced

To debug macros:
```
cargo +nightly rustc --bin wasm -- -Z unstable-options --pretty=expanded > demacroed.rs
```

To check wasm size:
```
twiggy top -n 20 target/wasm32-unknown-unknown/release/wasm.wasm
```