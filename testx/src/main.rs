use llml::{mat::Matf3};
use llml::others::Random;

fn main () {
    let alpha = Matf3::random();
    let beta = Matf3::random();
    let gamma = alpha + beta;

    assert_eq!(alpha.x + beta.x, gamma.x);
    assert_eq!(alpha.y + beta.y, gamma.y);
    assert_eq!(alpha.z + beta.z, gamma.z);
}