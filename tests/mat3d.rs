use llml::mat::{Matd3, Matf3};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_mats();
        assert_eq!(alpha $sy beta, Matd3::new([
            alpha.xx() $sy beta.xx(), alpha.xy() $sy beta.xy(), alpha.xz() $sy beta.xz(),
            alpha.yx() $sy beta.yx(), alpha.yy() $sy beta.yy(), alpha.yz() $sy beta.yz(),
            alpha.zx() $sy beta.zx(), alpha.zy() $sy beta.zy(), alpha.zz() $sy beta.zz(),
        ]));
    }
}

#[test]
fn eq () {
    assert_eq!(Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]), Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]));
    assert_ne!(Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]), Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 8.]))
}

#[test]
fn into () {
    assert_eq!(Into::<Matf3>::into(Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.])), Matf3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]))
}

#[test]
fn of_rot () {
    todo!()
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
    let alpha = Matd3::new([
        1., 2., 3., 
        4., 5., 6.,
        7., 8., 9.
    ]);

    let beta = Matd3::new([
        10., 11., 12., 
        13., 14., 15.,
        16., 17., 18.
    ]);

    assert_eq!(alpha * beta, Matd3::new([
        84., 90., 96.,
        201., 216., 231.,
        318., 342., 366.
    ]));
}

#[test]
fn neg () {
    let alpha : Matd3 = random();
    assert_eq!(-alpha, Matd3::new([
        -alpha.xx(), -alpha.xy(), -alpha.xz(),
        -alpha.yx(), -alpha.yy(), -alpha.yz(),
        -alpha.zx(), -alpha.zy(), -alpha.zz(),
    ]))
}

#[test]
fn tr () {
    let alpha = Matd3::new([
        1., 2., 3., 
        4., 5., 6.,
        7., 8., 9.
    ]);

    assert_eq!(alpha.tr(), 15.)
}

#[test]
fn det () {
    let alpha = Matd3::new([
        1., 2., 3., 
        4., 5., 6.,
        7., 8., 9.
    ]);

    let beta = Matd3::new([
        11., 11., 12., 
        13., 14., 15.,
        16., 17., 18.
    ]);

    assert_eq!(alpha.det(), 0.);
    assert_eq!(beta.det(), -3.)
}

#[test]
fn inv () {
    let alpha = Matd3::new([
        1., 2., 3., 
        4., 5., 6.,
        7., 8., 9.
    ]);

    let beta = Matd3::new([
        11., 11., 12., 
        13., 14., 15.,
        16., 17., 18.
    ]);

    assert_eq!(alpha.inv(), None);
    assert_eq!(beta.inv(), Some(Matd3::new([
        1., -2., 1.,
        -2., -2., 3.,
        1., 33. / 9., -33. / 9.
    ])));
}

fn get_mats () -> (Matd3, Matd3) {
    (random(), random())
}