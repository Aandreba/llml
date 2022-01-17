use rand::{prelude::*, distributions::Standard};
use crate::*;

macro_rules! impl_rand {
    ($($target:ident),+) => {
        $(
            impl Distribution<$target> for Standard {
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $target {
                    $target::new(self.sample(rng))
                }
            }
        )*
    };
}

impl_rand!(EucVecf2, EucVecf3, EucVecf4);
impl_rand!(Matf2, Matf3);

impl_rand!(EucVecd2, EucVecd3, EucVecd4);
impl_rand!(Matd2);