use llml::{EucVecf2};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_vecs();
        assert_eq!(alpha $sy beta, EucVecf2::new([alpha.x() $sy beta.x(), alpha.y() $sy beta.y()]))
    }
}

#[test]
fn add () {
    test_arith!(+);
}

#[test]
fn sub () {
    test_arith!(-);
}

#[test]
fn mul () {
    test_arith!(*);
}

#[test]
fn div () {
    test_arith!(/);
}

#[test]
fn neg () {
    let alpha : EucVecf2 = random();
    assert_eq!(-alpha, EucVecf2::new([-alpha.x(), -alpha.y()]))
}

#[test]
fn sum () {
    let alpha : EucVecf2 = random();
    assert_eq!(alpha.sum(), alpha.x() + alpha.y())
}


#[test]
fn dot () {
    let (alpha, beta) = get_vecs();
    assert_eq!(alpha.dot(beta), alpha.x() * beta.x() + alpha.y() * beta.y())
}

#[test]
fn norm () {
    let alpha : EucVecf2 = random();
    assert_eq!(alpha.norm(), alpha.x().hypot(alpha.y()))
}

#[test]
fn unit () {
    let alpha : EucVecf2 = random();

    let norm = alpha.x().hypot(alpha.y());
    assert_eq!(alpha.unit(), EucVecf2::new([alpha.x() / norm, alpha.y() / norm]))
}

fn get_vecs () -> (EucVecf2, EucVecf2) {
    (random(), random())
}