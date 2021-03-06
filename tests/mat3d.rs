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

#[cfg(feature = "llml_serde")]
#[test]
fn serde () {
    let alpha : Matd3 = random();
    let json = serde_json::to_string(&alpha).unwrap();
    let beta : Matd3 = serde_json::from_str(json.as_str()).unwrap();
    
    let diff : [f64;9] = (alpha - beta).into();
    assert!(diff.into_iter().sum::<f64>() <= f64::EPSILON * 9.);
}

#[test]
fn eq () {
    assert_eq!(Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]), Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]));
    assert_ne!(Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]), Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 8.]))
}

#[test]
fn into () {
    assert_eq!(Into::<Matf3>::into(Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.])), Matf3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]));
    assert_eq!(Into::<[f64;9]>::into(Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.])), [1., 2., 3., 4., 5., 6., 7., 8., 9.])
}

#[test]
fn rot () {
    let (sin, cos) = 1f64.sin_cos();
    let alpha = Matd3::from_rot(1., -std::f64::consts::PI, std::f64::consts::FRAC_PI_2);
    let beta = Matd3::new([
        0., cos, sin,
        1., 0., 0.,
        0., sin, -cos
    ]);

    let diff : [f64;9] = (alpha - beta).into();
    assert!(diff.into_iter().sum::<f64>() <= f64::EPSILON * 9.);
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
fn scal_mul () {
    let alpha : [f64;9] = random();
    let beta : [f64;9] = random();
    let mul = [
        alpha[0] * beta[0],
        alpha[1] * beta[1],
        alpha[2] * beta[2],
        alpha[3] * beta[3],
        alpha[4] * beta[4],
        alpha[5] * beta[5],
        alpha[6] * beta[6],
        alpha[7] * beta[7],
        alpha[8] * beta[8],
    ];

    let a = Matd3::new(alpha);
    let b = Matd3::new(beta);

    assert_eq!(a.scal_mul(b), Matd3::new(mul))
}

#[test]
fn scal_div () {
    let alpha : [f64;9] = random();
    let beta : [f64;9] = random();
    let mul = [
        alpha[0] / beta[0],
        alpha[1] / beta[1],
        alpha[2] / beta[2],
        alpha[3] / beta[3],
        alpha[4] / beta[4],
        alpha[5] / beta[5],
        alpha[6] / beta[6],
        alpha[7] / beta[7],
        alpha[8] / beta[8],
    ];

    let a = Matd3::new(alpha);
    let b = Matd3::new(beta);

    assert_eq!(a.scal_div(b), Matd3::new(mul))
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