[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/zkLinkProtocol/zklink_sdk/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/zkLinkProtocol/zklink_sdk/actions?query=branch%3Amain)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)  
ZkLink SDK is an open-source cross-platform library that implements low-level cryptographic wallet functionality
for ZkLink and Ethereum-based  blockchain networks.
The codebase is primarily in Rust and provides cross-language bindings using multiple tools:

- [uniffi-bindgen-go](https://github.com/NordSecurity/uniffi-bindgen-go) is used to generate bindings for Golang.
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) is used to generate bindings for JavaScript and TypeScript to facilitate interactions with the Wasm modules (for web browser extensions or other web-related apps).

## Pre-requisites
Naturally, you will need the [Rust toolchain] installed.
Besides that, for [UniFFI] language bindings, you will need the Golang formatters:

```bash
make prepare_ffi
```

For the JavaScript-Wasm bindings, you will need the `wasm-pack` to be installed:
```bash
make prepare_wasm
```

## Generate Golang bindings

First, you need to build the rust code as a library:
```bash
make build_binding_lib 
```
The dynamic and static system library `zklink_sdk` will be in `./target/release/lib`. Next, you need to generate the bindings:

```bash
make build_bindging_files 
```

The default binding path is `./binding_tests/generated`, if you want to generate bindings to another directory, just set the  `BINDINGS_DIR` when run the `make` command:

```bash
make build_binding_files BINDINGS_DIR=/path/to/the/bindings/directory
```

To use the bindings and the library, see the detail in `make test_go` and `make run_exampl_go`command.

## Generate Wasm bindings

```bash
make build_wasm
```


