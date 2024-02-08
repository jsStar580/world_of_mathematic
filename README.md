<h1 align="center">world_of_mathematics</h1>

The world_of_mathematics repository is a collection of Rust libraries for designing and working with zero-knowledge succinct non-interactive arguments (zkSNARKs). It provides efficient implementations of the key world_of_mathematicsic components underlying zkSNARKs, including finite fields, elliptic curves, and polynomials.

This library is released under the MIT License and the Apache v2 License (see [License](#license)).

**WARNING:** This is an academic proof-of-concept prototype, and in particular has not received careful code review. This implementation is NOT ready for production use.

## Directory structure

This repository contains several Rust crates:

- [`ark-ff`](ff): Generic abstractions for, and implementations of various kinds of finite fields
- [`ark-ec`](ec): Generic abstractions for prime-order groups, and implementations of various kinds of (pairing-friendly and standard) elliptic curves
- [`ark-poly`](poly): Interfaces for univariate, multivariate, and multilinear polynomials, and FFTs over finite fields
- [`ark-serialize`](serialize): Efficient interfaces for serialization and point compression for finite fields and elliptic curves

## Build guide

The library compiles on the `stable` toolchain of the Rust compiler (v 1.51+). To install the latest version of Rust, first install `rustup` by following the instructions [here](https://rustup.rs/), or via your platform's package manager. Once `rustup` is installed, install the Rust toolchain by invoking:

```bash
rustup install stable
```
After that, use `cargo`, the standard Rust build tool, to build the libraries:

```bash
git clone https://github.com/jsStar580/world_of_mathematics.git
cd world_of_mathematics
cargo build --release
```