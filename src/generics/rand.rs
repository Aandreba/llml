use rand::{prelude::*, distributions::Standard};
use crate::{vec::*, mat::*};
use super::{Complxf, Complxd};

macro_rules! impl_rand {
    ($($target:ident),+) => {
        $(
            impl Distribution<$target> for Standard {
                #[inline(always)]
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $target {
                    $target::new(self.sample(rng))
                }
            }
        )*
    };
}

impl Distribution<Complxf> for Standard {
    #[inline(always)]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Complxf {
        Complxf(self.sample(rng))
    }
}

impl Distribution<Complxd> for Standard {
    #[inline(always)]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Complxd {
        Complxd(self.sample(rng))
    }
}

impl_rand!(EucVec2f, EucVec3f, EucVec4f);
impl_rand!(Mat2f, Mat3f);

impl_rand!(EucVec2d, EucVec3d, EucVec4d);
impl_rand!(Mat2d, Mat3d);