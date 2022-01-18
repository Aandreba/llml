use std::{arch::aarch64::{float32x2_t, uint32x2_t, vbsl_f32}, intrinsics::transmute};
use llml::{Matf2, EucVecf2};

fn main () {
    unsafe {
        let alpha : float32x2_t = transmute([1,2]);
        let beta : float32x2_t = transmute([3,4]);
        let mask : uint32x2_t = transmute([u32::MAX, u32::MAX]);
        let gamma = vbsl_f32(mask, alpha, beta);

        println!("{:?}", gamma)
    }
}