# template-systems-rs

[![CI](https://github.com/ulquiorracode/template-systems-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ulquiorracode/template-systems-rs/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust: 2024](https://img.shields.io/badge/Language-Rust%202024-orange.svg)](https://www.rust-lang.org/)

Battle-tested systems-grade template for high-performance Rust services, engines, and libraries.

## Features

- **Nextest & Matrix CI**: Blazing fast parallel testing with `cargo-nextest` and `Swatinem/rust-cache@v2`.
- **Zero-Waste Build Configuration**: `CARGO_INCREMENTAL=0`, `-C debuginfo=0`, and LTO release optimizations.
- **Strict Linting**: Pre-configured `-D warnings` across rustfmt and clippy.
- **Community Governance**: Issue templates, PR template, Dependabot (`target-branch: dev`), `CONTRIBUTING.md`, and `SECURITY.md`.
- **Criterion Harness**: Ready-to-run statistical micro-benchmarking suite.
