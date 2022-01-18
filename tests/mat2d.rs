use llml::{Matd2, Matf2};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_mats();
        assert_eq!(alpha $sy beta, Matd2::new([
            alpha.xx() $sy beta.xx(), alpha.xy() $sy beta.xy(),
            alpha.yx() $sy beta.yx(), alpha.yy() $sy beta.yy()
        ]));
    }
}

#[test]
fn eq () {
    assert_eq!(Matd2::new([1., 2., 3., 4.]), Matd2::new([1., 2., 3., 4.]));
    assert_ne!(Matd2::new([1., 2., 3., 4.]), Matd2::new([5., 6., 7., 8.]))
}

#[test]
fn into () {
    assert_eq!(Into::<Matf2>::into(Matd2::new([1., 2., 3., 4.])), Matf2::new([1., 2., 3., 4.]))
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
    let alpha = Matd2::new([1., 2., 3., 4.]);
    let beta = Matd2::new([5., 6., 7., 8.]);
    assert_eq!(alpha * beta, Matd2::new([19., 22., 43., 50.]));
}

#[test]
fn neg () {
    let alpha : Matd2 = random();
    assert_eq!(-alpha, Matd2::new([-alpha.xx(), -alpha.xy(), -alpha.yx(), -alpha.yy()]))
}

#[test]
fn tr () {
    let alpha = Matd2::new([1., 2., 3., 4.]);
    assert_eq!(alpha.tr(), 5.)
}

#[test]
fn det () {
    let alpha = Matd2::new([1., 2., 3., 4.]);
    assert_eq!(alpha.det(), -2.)
}

#[test]
fn inv () {
    let alpha = Matd2::new([1., 2., 3., 4.]);
    assert_eq!(alpha.inv(), Some(Matd2::new([-2., 1., 3./2., -1./2.])));
}

fn get_mats () -> (Matd2, Matd2) {
    (random(), random())
}