use std::ops::{Deref, Div, Add, Sub, Mul};
use num::Num;

use crate::frac_arith;

pub struct Fraction<T: Num> {
    pub num: T,
    pub den: T
}

impl<T: Num> Fraction<T> {
    pub fn new (num: T, den: T) -> Self {
        Fraction { num, den }
    }

    pub fn of_value (num: T) -> Self {
        Fraction { num, den: T::one() }
    }
}

impl<T: Num> Deref for Fraction<T> {
    type Target = <T as Div<T>>::Output;

    fn deref(&self) -> &Self::Target {
        let div = self.num / self.den;
        &div
    }
}

// ARITH
frac_arith!(Add, Fraction<T>, add, |x : &Fraction<T>, y : &Fraction<T>| {
    if x.den == y.den {
        return Fraction::new(x.num + y.num, x.den)
    }

    Fraction::new(x.num * y.den + y.num * x.den, x.den * y.den)
});

frac_arith!(Sub, Fraction<T>, sub, |x : &Fraction<T>, y : &Fraction<T>| {
    if x.den == y.den {
        return Fraction::new(x.num - y.num, x.den)
    }

    Fraction::new(x.num * y.den - y.num * x.den, x.den * y.den)
});

frac_arith!(Mul, Fraction<T>, mul, |x : &Fraction<T>, y : &Fraction<T>| {
    Fraction::new(x.num * y.num, x.den * y.den)
});

impl<T: Num + Copy + Mul<T>> Div for Fraction<T> {
    type Output = Fraction<T>;

    fn div (self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.den, rhs.num * self.den)
    }
}