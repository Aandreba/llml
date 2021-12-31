use rand::distributions::{Standard, Distribution};

pub trait Random {
    fn random () -> Self;
}

impl<T> Random for T where Standard: Distribution<T> {
    fn random () -> Self {
        rand::random()
    }
}