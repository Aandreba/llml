use core::slice;
use std::{iter::{Step, Sum}, ptr::{slice_from_raw_parts, addr_of}, ops::{Add, Mul}, fmt::Debug};
use num::{Float, Complex, Num, Zero, traits::Pow};
use crate::{extra::{complex::{sqrt, fst_cbrt}, consts::Consts}, array::{empty_array, build_array}, vec::seq::EucVec};

macro_rules! cubic_loop {
    () => {
        let pow = pow * zeta;
        let u = pow * c;
        k * (u + b + (d0 / u))
    };
}

// QUADRATIC
pub fn quadratic <T: Float + Copy + Consts> (a: T, b: T, c: T) -> Option<(T,T)> {
    let disc = b * b - T::four() * a * c;
    if disc >= T::zero() {
        let sqrt = disc.sqrt();
        let div = T::two() * a;
        return Some(((-b + sqrt) / div, (-b - sqrt) / div))
    }

    None
}

pub fn quadratic_complex <T: Float + Copy + Consts> (a: T, b: T, c: T) -> (Complex<T>, Complex<T>) {
    let disc = b * b - T::four() * a * c;
    let sqrt = sqrt(disc);
    let div = T::two() * a;

    ((sqrt - b) / div, (-sqrt - b) / div)
}

// CUBIC
pub fn cubic <T: Float + Copy + Consts> (a: T, b: T, c: T, d: T) -> (Complex<T>, Complex<T>, Complex<T>) {
    let zeta = (sqrt(-T::three()) - T::one()) / T::two();

    let b2 = b * b;
    let a3 = T::three() * a;

    let d0 = b2 - a3 * c;
    let d1 = T::two() * b2 * b - T::nine() * a * b * c + T::twenty_seven() * a * a * d;

    let c = fst_cbrt((sqrt(d1 * d1 - T::four() * d0 * d0 * d0) + d1) / T::two());
    let k = -T::one() / a3;

    let pow = Complex { re: T::one(), im: T::zero() };

    let pow = pow * zeta;
    let u = pow * c;
    let alpha = (u.inv() * d0 + u + b) * k;

    let pow = pow * zeta;
    let u = pow * c;
    let beta = (u.inv() * d0 + u + b) * k;

    let pow = pow * zeta;
    let u = pow * c;
    let gamma = (u.inv() * d0 + u + b) * k;

    (alpha, beta, gamma)
}

// NTH
pub fn poly<'a, T: Float + Copy + Consts + Debug, const N: usize> (vals: [T;N]) -> [Complex<T>;N-1] {
    match N {
        0|1 => panic!("Invalid size"),

        2 => [Complex::new(-vals[1] / vals[0], T::zero());N-1],
        
        3 => unsafe { 
            let tuple = quadratic_complex(vals[0], vals[1], vals[2]);
            let ptr = addr_of!(tuple);
            *(ptr as *const [Complex<T>;N-1])
        },

        4 => unsafe { 
            let tuple = cubic(vals[0], vals[1], vals[2], vals[3]);
            let ptr = addr_of!(tuple);
            *(ptr as *const [Complex<T>;N-1])
        },

        _ => {
            let limit = T::one() / T::max_value();
            let vals = EucVec::new(vals) / vals[0];
            let mut result = EucVec::new(build_array(|i| {
                let i = (i+1) as i32;
                Complex::new(T::one().powi(i), T::two().powi(i))
            }));

            let mut last = result.clone();

            loop {
                last = result.clone();
                for i in 0..(N-1) {
                    let val = apply_poly(result[i], &vals);
                    let mut div = Complex::new(T::one(), T::zero());

                    for j in 0..(N-1) {
                        if i == j { continue; }
                        div = div * (result[i] - result[j]);
                    }
                    
                    result[i] = result[i] - val / div;
                }

                let all_equal : bool = (result - last).into_iter()
                    .all(|x| x.re <= limit && x.im <= limit);

                if all_equal { break; }
            }
            
            result.into()
        }
    }
}

pub fn apply_poly<T: Float, const N: usize> (x: Complex<T>, vals: &EucVec<T,N>) -> Complex<T> {
    let mut pow = Complex::new(T::one(), T::zero());
    let mut result = Complex::new(T::zero(), T::zero());

    for i in (0..N).rev() {
        result = result + (pow * vals[i]);
        pow = pow * x;
    }

    result
}