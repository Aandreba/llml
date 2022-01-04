#![feature(asm, asm_const)]

use std::arch::asm;

fn main () {
    let alpha = &[1f32, 2.] as *const [f32;2] as *const f32;
    let beta : u64;

    unsafe {
        asm!(
            "add {0}, {1}, {1}",
            out(reg) beta,
            in(reg) alpha
        );

        println!("{:?}", *(beta as *const [f32;2]));
    }
}

/*fn main () {
    unsafe {
        let a : float32x2_t = transmute([1f32, 2.]);
        let b : float32x2_t = transmute([1f32, 2.]);

        let c : [f32;2] = transmute(vadd_f32(a, b));
        if c[0] == 2. {
            panic!()
        }
    }
}*/