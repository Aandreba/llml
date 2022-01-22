# Low Level Math Library (LLML)
[![Crate](https://img.shields.io/crates/v/llml.svg)](https://crates.io/crates/llml)
[![API](https://docs.rs/llml/badge.svg)](https://docs.rs/llml)

Implementation of basic math data types with high level frontend and low level backend

## Supported targets
LLML is currently supported on **x86/x86_64** and **ARM/aarch64**, with plans for _WASM_ support and a _naive_ implementation, available for all targets, in the future.

## Instruction set support
As of today, LLML supports up to **SSE3** for x86/x86_64, and **Neon** for ARM/aarch64. Support for **AVX** and **AVX2** is expected in the near future, as an optional feature.

## Currently implemented
### Complex numbers for ```f32``` and ```f64```
- Basic arithmetic (**Addition**, **Subtraction**, **Negation**, **Multiplication**, **Division**)
- Complex arithmetic (**Multiplication** and **Division**)
- **Inverse** and **Conjugate**
- **Radius**, **Angle** & **Polar** coordinates
- **Square root**, **Exponential** and **Natural Logarithm**
- **Sine**, **Cosine** and **Tangent**
- **Power** of integer, decimal & complex
- Exponential of imaginary (**```expi```**)
- Complex square root of real (**```sqrtc```**)
- Power by complex (**```powc```**) and power by imaginary (**```powci```**)

### Euclidian vectors of 2, 3 & 4 values for ```f32``` and ```f64```
- Basic arithmetic (**Addition**, **Subtraction**, **Negation**, **Multiplication** & **Division**)
- **Summation** (```sum```), **norm** and **unit** vector
- **Dot product** (```dot```) and **cross product** (```cross```)
- **Square root** (```sqrt```) and **fast square root**  (```sqrt_fast```, only faster in x86/x86_64)

### Matrices of 2x2 & 3x3 for ```f32``` and ```f64```
- Scalar arithmetic (**Addition**, **Subtraction**, **Negation**, **Multiplication**, **Division**)
- Matrix arithmetic (Matrix-Matrix and Matrix-Vector **multiplication**)
- **Trace** (```tr```) and **determinant** (```det```)
- Safe (```inv```) and unsafe (```inv_unsafe```) **inverse**
- **Transpose** (```transp```)
- **Rotation matrix**

## Current features
### llml_rand
Implements **random** generation for all of the crate's data types with [rand](https://crates.io/crates/rand)

### llml_serde
Implements **serialization** and **deserialization** for all of the crate's data types with [serde](https://crates.io/crates/serde)