# MNIST64 Rust + WASM Port

A Rust based inference implementation of incredible work by **jmagic** [MNIST64](https://github.com/jarnoh/mnist64).

> NOTE! The implementation as of now is partial and includes only conv1 and early head scoring.

The inference is based on the original C64 BNN model parameters widened to larger integer types.
