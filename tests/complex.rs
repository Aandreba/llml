use llml::{others::{Complxf, Complxd, ComplexSqrt, Zero}, vec::EucVecf2};
use rand::{random, thread_rng, Rng};

#[test]
fn eq () {
    assert_eq!(Complxf::new(1., 2.), Complxf::new(1., 2.));
    assert_ne!(Complxf::new(1., 2.), Complxf::new(3., 3.))
}

#[test]
fn into () {
    assert_eq!(Into::<Complxd>::into(Complxf::new(1., 2.)), Complxd::new(1., 2.))
}

#[test]
fn add () {
    let alpha : Complxf = random();
    let beta : Complxf = random();

    assert_eq!(alpha + beta, Complxf::new(alpha.re() + beta.re(), alpha.im() + beta.im()))
}

#[test]
fn sub () {
    let alpha : Complxf = random();
    let beta : Complxf = random();

    assert_eq!(alpha - beta, Complxf::new(alpha.re() - beta.re(), alpha.im() - beta.im()))
}

#[test]
fn mul () {
    let alpha : Complxf = random();
    let beta : Complxf = random();

    assert_eq!(alpha * beta, Complxf::new(
        alpha.re() * beta.re() - alpha.im() * beta.im(),
        alpha.re() * beta.im() + alpha.im() * beta.re()
    ))
}

#[test]
fn div () {
    let alpha : Complxf = Complxf::new(1., 2.);
    let beta : Complxf = Complxf::new(3., 4.);

    let div = beta.re() * beta.re() + beta.im() * beta.im();
    assert_eq!(alpha / beta, Complxf::new(
        (alpha.re() * beta.re() + alpha.im() * beta.im()) / div,
        (-alpha.re() * beta.im() + alpha.im() * beta.re()) / div
    ))
}

#[test]
fn conj () {
    let alpha : Complxf = random();
    assert_eq!(alpha.conj(), Complxf::new(alpha.re(), -alpha.im()))
}

#[test]
fn sqrt () {
    let alpha = Complxf::new(-3., 4.);
    assert_eq!(alpha.sqrt(), Complxf::new(1., 2.));
}

#[test]
fn exp () {
    let alpha : Complxf = random();
    assert_eq!(alpha.exp(), Complxf::new(alpha.re().exp() * alpha.im().cos(), alpha.re().exp() * alpha.im().sin()));
}

#[test]
fn ln () {
    let alpha : Complxf = random();
    let (radius, angle) = alpha.polar();
    assert_eq!(alpha.ln(), Complxf::new(radius.ln(), angle));
}

#[test]
fn powi () {
    let alpha : Complxf = random();
    loop {
        let beta : i32 = thread_rng().gen_range(-10..=10);
        if beta.is_zero() { continue; }

        let diff : EucVecf2 = (alpha.powi(beta as i32) - Complxf::exp((beta as f32) * alpha.ln())).into();
        assert!(diff.abs().sum() <= f32::EPSILON * 2.);
        break;
    }
}

#[test]
fn powf () {
    let alpha : Complxf = random();
    let beta : f32 = random();

    let diff : EucVecf2 = (alpha.powf(beta) - Complxf::exp(beta * alpha.ln())).into();
    assert!(diff.abs().sum() <= f32::EPSILON * 2.);
}

#[test]
fn powc () {
    let alpha : Complxf = random();
    let beta : Complxf = random();

    let diff : EucVecf2 = (alpha.powc(beta) - Complxf::exp(beta * alpha.ln())).into();
    assert!(diff.abs().sum() <= f32::EPSILON * 2.);
}

#[test]
fn sin () {
    let alpha : Complxf = random();
    assert_eq!(alpha.sin(), Complxf::new(alpha.re().sin() * alpha.im().cosh(), alpha.re().cos() * alpha.im().sinh()));
}

#[test]
fn cos () {
    let alpha : Complxf = random();
    assert_eq!(alpha.cos(), Complxf::new(alpha.re().cos() * alpha.im().cosh(), -alpha.re().sin() * alpha.im().sinh()));
}

#[test]
fn tan () {
    let alpha : Complxf = random();
    let beta = Complxf::new(2. * alpha.re(), 2. * alpha.im());

    let div = beta.re().cos() + beta.im().cosh();
    assert_eq!(alpha.tan(), Complxf::new(
        beta.re().sin() / div,
        beta.im().sinh() / div
    ));
}

#[test]
fn expi () {
    let alpha : f32 = random();
    assert_eq!(Complxf::expi(alpha), Complxf::new(alpha.cos(), alpha.sin()))
}

#[test]
fn sqrtc () {
    let alpha : f32 = random();
    assert_eq!(alpha.sqrtc(), Complxf::new(alpha.sqrt(), 0.));
    assert_eq!((-alpha).sqrtc(), Complxf::new(0., alpha.sqrt()))
}