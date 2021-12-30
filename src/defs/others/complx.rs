use std::ops::Neg;

use cfg_if::cfg_if;

pub type Complxf = Complx<f32>;
pub type Complxd = Complx<f64>;

cfg_if! {
    if #[cfg(feature = "llml_serde")] {
        mod def {
            use serde::{Serialize, Deserialize};
            
            #[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Complx<T> {
                pub re: T,
                pub im: T
            }
        }
    } else {
        mod def {
            #[derive(Default, Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Complx<T> {
                pub re: T,
                pub im: T
            }
        }
    }
}

pub use def::*;

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