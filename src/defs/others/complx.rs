import_derives!();
use std::ops::Neg;

pub type Complxf = Complx<f32>;
pub type Complxd = Complx<f64>;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "llml_rand", derive(Rand))]
pub struct Complx<T> {
    pub re: T,
    pub im: T
}

impl<T> Complx<T>  {
    pub fn new (re: T, im: T) -> Self {
        Self { re, im }
    }

    pub fn conj (self) -> Self where T: Neg<Output = T> {
        Self::new(self.re, -self.im)
    }
}

impl Complxf {
    pub fn magn (self) -> f32 {
        self.re.hypot(self.im)
    }
}

impl Complxd {
    pub fn magn (self) -> f64 {
        self.re.hypot(self.im)
    }
}