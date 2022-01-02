# Low Level Math Library (LLML)
[![Crate](https://img.shields.io/crates/v/llml.svg)](https://crates.io/crates/llml)
[![API](https://docs.rs/llml/badge.svg)](https://docs.rs/llml)

Implementation of basic math data types with high level frontend and low level backend

# Current implementation
✔️ -> Implemented\
🟧 -> Implemented, not released yet\
⚠️ -> Implementation expected\
❌ -> Not implemented

## Single precision
### Complex number
|                    | SSE (x86/64) | Neon (aarch64) | Wasm | Naive |
| ----------------   | ------------ | -------------- | ---- | ----- |
| Addition           | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Subtraction        | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Multiplication     | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Division           | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Conjugate          | ✔️           | ✔️            | ⚠️   | ✔️   |
| Polar Coordinates  | ✔️           | ✔️            | ⚠️   | ✔️   |
| Square Root        | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Exponential        | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Natural Logarithm  | ✔️           | ✔️            | ⚠️   | ✔️   |
| Sine               | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Cosine             | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Tangent            | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Hyperbolic Sine    | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Hyperbolic Cosine  | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Hyperbolic Tangent | ⚠️           | ⚠️            | ⚠️   | ⚠️   |

### Quaternion
|                    | SSE (x86/64) | Neon (aarch64) | Wasm | Naive |
| ----------------   | ------------ | -------------- | ---- | ----- |
| Addition           | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Subtraction        | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Multiplication     | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Division           | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Conjugate          | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Polar Coordinates  | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Square Root        | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Exponential        | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Natural Logarithm  | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Sine               | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Cosine             | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Tangent            | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Hyperbolic Sine    | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Hyperbolic Cosine  | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Hyperbolic Tangent | ⚠️           | ⚠️            | ⚠️   | ⚠️   |

### Affine Transform 2D
|                    | SSE (x86/64) | Neon (aarch64) | Wasm | Naive |
| ----------------   | ------------ | -------------- | ---- | ----- |
| Addition           | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Subtraction        | ⚠️           | ⚠️            | ⚠️   | ⚠️   |
| Tranformation      | 🟧           | ⚠️            | ⚠️   | 🟧   |

### Vector 2
|                  | SSE (x86/64) | Neon (aarch64) | Wasm | Naive |
| ---------------- | ------------ | -------------- | ---- | ----- |
| Addition         | ✔️           | ✔️            | ⚠️   | ✔️   |
| Subtraction      | ✔️           | ✔️            | ⚠️   | ✔️   |
| Multiplication   | ✔️           | ✔️            | ⚠️   | ✔️   |
| Division         | ✔️           | ✔️            | ⚠️   | ✔️   |
| Dot product      | ✔️           | ✔️            | ⚠️   | ✔️   |
| Sum              | ✔️           | ✔️            | ⚠️   | ✔️   |
| Norm             | ✔️           | ✔️            | ⚠️   | ✔️   |
| Norm<sup>2</sup> | ✔️           | ✔️            | ⚠️   | ✔️   |
| Unit             | ✔️           | ✔️            | ⚠️   | ✔️   |

### Vector 3
|                    | SSE (x86/64) | Neon (aarch64) | Wasm | Naive |
| ------------------ | ------------ | -------------- | ---- | ----- |
| Addition           | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Subtraction        | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Multiplication     | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Division           | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Dot product        | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Cross product      | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Sum                | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Norm               | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Norm<sup>2</sup>   | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Unit               | ✔️           | ⚠️            | ⚠️   | ✔️   |

### Vector 4
|                  | SSE (x86/64) | Neon (aarch64) | Wasm | Naive |
| ---------------- | ------------ | -------------- | ---- | ----- |
| Addition         | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Subtraction      | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Multiplication   | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Division         | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Dot product      | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Sum              | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Norm             | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Norm<sup>2</sup> | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Unit             | ✔️           | ⚠️            | ⚠️   | ✔️   |

### Matrix 2
|                  | SSE (x86/64) | Neon (aarch64) | Wasm | Naive |
| ---------------- | ------------ | -------------- | ---- | ----- |
| Addition         | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Subtraction      | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Multiplication   | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Division         | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Transposed       | ✔️           | ✔️            | ⚠️   | ✔️   |
| Inverse          | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Trace            | ✔️           | ✔️            | ⚠️   | ✔️   |
| Determinant      | ✔️           | ⚠️            | ⚠️   | ⚠️   |

### Matrix 3
|                  | SSE (x86/64) | Neon (aarch64) | Wasm | Naive |
| ---------------- | ------------ | -------------- | ---- | ----- |
| Addition         | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Subtraction      | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Multiplication   | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Division         | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Transposed       | ✔️           | ✔️            | ⚠️   | ✔️   |
| Inverse          | ✔️           | ⚠️            | ⚠️   | ⚠️   |
| Trace            | ✔️           | ⚠️            | ⚠️   | ✔️   |
| Determinant      | ✔️           | ⚠️            | ⚠️   | ⚠️   |

### Matrix 4
4x4 matrices are yet to be implemented, but are expected to be in the future

## Double precision
Double precision arithmetic is to be implemented on all architectures, except on Naive 