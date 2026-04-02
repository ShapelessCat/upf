# Unified Pseudopotential Format

[![CI](https://github.com/ShapelessCat/upf/actions/workflows/ci.yml/badge.svg)](https://github.com/ShapelessCat/upf/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/upf.svg)](https://crates.io/crates/upf)
[![Documentation](https://docs.rs/upf/badge.svg)](https://docs.rs/upf)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

The Unified Pseudopotential Format (UPF) specification is here:
<https://pseudopotentials.quantum-espresso.org/home/unified-pseudopotential-format>.

This repository provides a Rust library that reads Unified Pseudopotential
Format (UPF) text files and deserializes them into typed Rust structs for
further computation.

Project documentation:

- [Project guide](docs/PROJECT.md)

Local verification:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo doc --no-deps`

Core API:

- `upf::from_str`
- `upf::from_reader`
- `upf::from_file`
