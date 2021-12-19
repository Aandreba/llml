use std::ops::{Div, Add, Mul, Rem, Sub, Neg};
use num::{Num, One, Zero};

#[derive(Debug, Clone, Copy)]
pub struct Fraction<T> {
    pub num: T,
    pub den: T
}

impl<T> Fraction<T> {
    pub fn new (num: T, den: T) -> Self {
        Fraction { num, den }
    }

    pub fn of_value (num: T) -> Self where T: One {
        Fraction { num, den: T::one() }
    }
}

// ADDITION
impl<I, T: Mul<I, Output = A>, A: Add<A, Output = A>> Add<Fraction<I>> for Fraction<T> {
    type Output = Fraction<A>;

    fn add (self, rhs: Fraction<I>) -> Self::Output {
        Fraction::new(self.num * rhs.den + self.den * rhs.num, self.den * rhs.den)
    }
}

// SUBTRACTION
impl<I, T: Mul<I, Output = A>, A: Sub<A, Output = A>> Sub<Fraction<I>> for Fraction<T> {
    type Output = Fraction<A>;

    fn sub (self, rhs: Fraction<I>) -> Self::Output {
        Fraction::new(self.num * rhs.den - self.den * rhs.num, self.den * rhs.den)
    }
}

// MULTIPLICATION
impl<I, O, T: Mul<I, Output = O>> Mul<Fraction<I>> for Fraction<T> {
    type Output = Fraction<O>;

    fn mul (self, rhs: Fraction<I>) -> Self::Output {
        Fraction::new(self.num * rhs.num, self.den * rhs.den)
    }
}


// DIVISION
impl<I, O, T: Mul<I, Output = O>> Div<Fraction<I>> for Fraction<T> {
    type Output = Fraction<O>;

    fn div (self, rhs: Fraction<I>) -> Self::Output {
        Fraction::new(self.num * rhs.den, self.den * rhs.num)
    }
}

// NEGATE
impl<T: Neg<Output = T>> Neg for Fraction<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fraction::new(-self.num, self.den)
    }
}

// MOD
impl<T: Num + Copy + Rem<T>> Rem for Fraction<T> {
    type Output = Fraction<T>;

    fn rem (self, rhs: Self) -> Self::Output {
        let val = (self.num / self.den) % (rhs.num / rhs.den);
        Fraction::of_value(val)
    }
}

// CONSTANTS
impl<T: Num + Copy + Zero> Zero for Fraction<T> {
    fn zero() -> Self {
        Self::of_value(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.num.is_zero()
    }
}

impl<T: Num + Copy + One> One for Fraction<T> {
    fn one() -> Self {
        Self::of_value(T::one())
    }

    fn is_one(&self) -> bool where Self: PartialEq {
        self.num == self.den
    }
}

// COMPARE
impl<T: PartialEq + Mul<Output = T>> PartialEq for Fraction<T> {
    fn eq (&self, other: &Self) -> bool {
        self.num * other.den == other.num * self.den
    }
}

impl<T: PartialOrd + Mul<Output = T>> PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num * other.den).partial_cmp(&(other.num * self.den))
    }
}

// NUM
impl<T: Num + Copy> Num for Fraction<T> {
    type FromStrRadixErr = Result<String, T::FromStrRadixErr>;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let split : Vec<&str> = str.split("\\s*/\\s*").collect();
        match split.len() {
            1 => T::from_str_radix(split[0], radix).map(|x| Self::of_value(x)).map_err(|e| Err(e)),
            2 => {
                match T::from_str_radix(split[0], radix) {
                    Err(x) => Err(Err(x)),
                    Ok(first) => match T::from_str_radix(split[1], radix) {
                        Err(x) => Err(Err(x)),
                        Ok(last) => Ok(Self::new(first, last))
                    }
                }
            },
            _ => Err(Ok("Invalid input".to_string()))
        }
    }
}