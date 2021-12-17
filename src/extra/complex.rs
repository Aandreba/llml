use num::{Float, Complex};

use super::consts::Consts;

pub fn sqrt<T: Float> (x: T) -> Complex<T> {
    if x >= T::zero() {
        return Complex { re: x.sqrt(), im: T::zero() }
    }

    Complex { re: T::zero(), im: (-x).sqrt() }
}

pub fn fst_cbrt<T: Float + Consts> (x: Complex<T>) -> Complex<T> {
    if x.im.is_zero() {
        if x.re.is_sign_positive() {
            // simple positive real ∛r, and copy `im` for its sign
            Complex::new(x.re.cbrt(), x.im)
        } else {
            // ∛(r e^(iπ)) = ∛r e^(iπ/3) = ∛r/2 + i∛r√3/2
            // ∛(r e^(-iπ)) = ∛r e^(-iπ/3) = ∛r/2 - i∛r√3/2
            let two = T::two();
            let three = T::three();
            let re = (-x.re).cbrt() / two;
            let im = three.sqrt() * re;
            if x.im.is_sign_positive() {
                Complex::new(re, im)
            } else {
                Complex::new(re, -im)
            }
        }
    } else if x.re.is_zero() {
        // ∛(r e^(iπ/2)) = ∛r e^(iπ/6) = ∛r√3/2 + i∛r/2
        // ∛(r e^(-iπ/2)) = ∛r e^(-iπ/6) = ∛r√3/2 - i∛r/2
        let two = T::two();
        let three = T::three();
        let im = x.im.abs().cbrt() / two;
        let re = three.sqrt() * im;
        if x.im.is_sign_positive() {
            Complex::new(re, im)
        } else {
            Complex::new(re, -im)
        }
    } else {
        // formula: cbrt(r e^(it)) = cbrt(r) e^(it/3)
        let three = T::three();
        let (r, theta) = x.to_polar();
        Complex::from_polar(r.cbrt(), theta / three)
    }
}