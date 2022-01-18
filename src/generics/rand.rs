use rand::{prelude::*, distributions::Standard};
use super::{Complxf, Complxd};
use crate::*;

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

impl_rand!(EucVecf2, EucVecf3, EucVecf4);
impl_rand!(Matf2, Matf3);

impl_rand!(EucVecd2, EucVecd3, EucVecd4);
impl_rand!(Matd2, Matd3);