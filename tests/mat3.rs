use llml::{mat::{Matf3, Matd3}};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_mats();
        assert_eq!(alpha $sy beta, Matf3::new([
            alpha.xx() $sy beta.xx(), alpha.xy() $sy beta.xy(), alpha.xz() $sy beta.xz(),
            alpha.yx() $sy beta.yx(), alpha.yy() $sy beta.yy(), alpha.yz() $sy beta.yz(),
            alpha.zx() $sy beta.zx(), alpha.zy() $sy beta.zy(), alpha.zz() $sy beta.zz(),
        ]));
    }
}

#[cfg(feature = "llml_serde")]
#[test]
fn serde () {
    let alpha : Matf3 = random();
    let json = serde_json::to_string(&alpha).unwrap();
    let beta : Matf3 = serde_json::from_str(json.as_str()).unwrap();
    
    let diff : [f32;9] = (alpha - beta).into();
    assert!(diff.into_iter().sum::<f32>() <= f32::EPSILON * 9.);
}

#[test]
fn eq () {
    assert_eq!(Matf3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]), Matf3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]));
    assert_ne!(Matf3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]), Matf3::new([1., 2., 3., 4., 5., 6., 7., 8., 8.]))
}

#[test]
fn into () {
    assert_eq!(Into::<Matd3>::into(Matf3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.])), Matd3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.]));
    assert_eq!(Into::<[f32;9]>::into(Matf3::new([1., 2., 3., 4., 5., 6., 7., 8., 9.])), [1., 2., 3., 4., 5., 6., 7., 8., 9.])
}

#[test]
fn rot () {
    let (sin, cos) = 1f32.sin_cos();
    let alpha = Matf3::from_rot(1., -std::f32::consts::PI, std::f32::consts::FRAC_PI_2);
    let beta = Matf3::new([
        0., cos, sin,
        1., 0., 0.,
        0., sin, -cos
    ]);

    let diff : [f32;9] = (alpha - beta).into();
    assert!(diff.into_iter().sum::<f32>() <= f32::EPSILON * 9.);
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
    let alpha = Matf3::new([
        1., 2., 3., 
        4., 5., 6.,
        7., 8., 9.
    ]);

    let beta = Matf3::new([
        10., 11., 12., 
        13., 14., 15.,
        16., 17., 18.
    ]);

    assert_eq!(alpha * beta, Matf3::new([
        84., 90., 96.,
        201., 216., 231.,
        318., 342., 366.
    ]));
}

#[test]
fn scal_mul () {
    let alpha : [f32;9] = random();
    let beta : [f32;9] = random();
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

    let a = Matf3::new(alpha);
    let b = Matf3::new(beta);

    assert_eq!(a.scal_mul(b), Matf3::new(mul))
}

#[test]
fn scal_div () {
    let alpha : [f32;9] = random();
    let beta : [f32;9] = random();
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

    let a = Matf3::new(alpha);
    let b = Matf3::new(beta);

    assert_eq!(a.scal_div(b), Matf3::new(mul))
}

#[test]
fn neg () {
    let alpha : Matf3 = random();
    assert_eq!(-alpha, Matf3::new([
        -alpha.xx(), -alpha.xy(), -alpha.xz(),
        -alpha.yx(), -alpha.yy(), -alpha.yz(),
        -alpha.zx(), -alpha.zy(), -alpha.zz(),
    ]))
}

#[test]
fn tr () {
    let alpha = Matf3::new([
        1., 2., 3., 
        4., 5., 6.,
        7., 8., 9.
    ]);

    assert_eq!(alpha.tr(), 15.)
}

#[test]
fn det () {
    let alpha = Matf3::new([
        1., 2., 3., 
        4., 5., 6.,
        7., 8., 9.
    ]);

    let beta = Matf3::new([
        11., 11., 12., 
        13., 14., 15.,
        16., 17., 18.
    ]);

    assert_eq!(alpha.det(), 0.);
    assert_eq!(beta.det(), -3.)
}

#[test]
fn inv () {
    let alpha = Matf3::new([
        1., 2., 3., 
        4., 5., 6.,
        7., 8., 9.
    ]);

    let beta = Matf3::new([
        11., 11., 12., 
        13., 14., 15.,
        16., 17., 18.
    ]);

    assert_eq!(alpha.inv(), None);
    assert_eq!(beta.inv(), Some(Matf3::new([
        1., -2., 1.,
        -2., -2., 3.,
        1., 33. / 9., -33. / 9.
    ])));
}

fn get_mats () -> (Matf3, Matf3) {
    (random(), random())
}