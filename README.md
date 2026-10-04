# MNIST64 Rust + WASM Port

A Rust based inference implementation of incredible work by **jmagic** [MNIST64](https://github.com/jarnoh/mnist64).

The inference is based on the original C64 BNN model parameters.

What is different compared to MNIST64
- Weights of conv1 and early head are transformed from u8 to u64
- Processing of conv1 thresholds transformed to weighted Hamming distance to avoid masking

