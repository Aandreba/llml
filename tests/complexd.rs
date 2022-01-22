use llml::{others::{Complxd, Complxf, ComplexSqrt}, vec::EucVecd2};
use rand::{random, distributions::Uniform, thread_rng, Rng};

#[test]
fn eq () {
    assert_eq!(Complxd::new(1., 2.), Complxd::new(1., 2.));
    assert_ne!(Complxd::new(1., 2.), Complxd::new(3., 3.))
}

#[test]
fn into () {
    assert_eq!(Into::<Complxf>::into(Complxd::new(1., 2.)), Complxf::new(1., 2.))
}

#[test]
fn add () {
    let alpha : Complxd = random();
    let beta : Complxd = random();

    assert_eq!(alpha + beta, Complxd::new(alpha.re() + beta.re(), alpha.im() + beta.im()))
}

#[test]
fn sub () {
    let alpha : Complxd = random();
    let beta : Complxd = random();

    assert_eq!(alpha - beta, Complxd::new(alpha.re() - beta.re(), alpha.im() - beta.im()))
}

#[test]
fn mul () {
    let alpha : Complxd = random();
    let beta : Complxd = random();

    assert_eq!(alpha * beta, Complxd::new(
        alpha.re() * beta.re() - alpha.im() * beta.im(),
        alpha.re() * beta.im() + alpha.im() * beta.re()
    ))
}

#[test]
fn div () {
    let alpha : Complxd = Complxd::new(1., 2.);
    let beta : Complxd = Complxd::new(2., 1.);

    let div = beta.re() * beta.re() + beta.im() * beta.im();
    let naive = Complxd::new(
        (alpha.re() * beta.re() + alpha.im() * beta.im()) / div,
        (-alpha.re() * beta.im() + alpha.im() * beta.re()) / div
    );

    let simd = alpha / beta;
    assert!((naive.re() - simd.re()).abs() <= f64::EPSILON);
    assert!((naive.im() - simd.im()).abs() <= f64::EPSILON)
}

#[test]
fn conj () {
    let alpha : Complxd = random();
    assert_eq!(alpha.conj(), Complxd::new(alpha.re(), -alpha.im()))
}

#[test]
fn sqrt () {
    let alpha = Complxd::new(-3., 4.);
    assert_eq!(alpha.sqrt(), Complxd::new(1., 2.));
    
}

#[test]
fn exp () {
    let alpha : Complxd = random();
    assert_eq!(alpha.exp(), Complxd::new(alpha.re().exp() * alpha.im().cos(), alpha.re().exp() * alpha.im().sin()));
}

#[test]
fn ln () {
    let alpha : Complxd = random();
    let (radius, angle) = alpha.polar();
    assert_eq!(alpha.ln(), Complxd::new(radius.ln(), angle));
}

#[test]
fn powi () {
    let alpha : Complxd = random();
    let beta : i32 = thread_rng().gen_range(-10..=10);

    let diff : EucVecd2 = (alpha.powi(beta as i32) - Complxd::exp((beta as f64) * alpha.ln())).into();
    assert!(diff.abs().sum() <= f64::EPSILON * 2.);
}

#[test]
fn powf () {
    let alpha : Complxd = random();
    let beta : f64 = random();

    let diff : EucVecd2 = (alpha.powf(beta) - Complxd::exp(beta * alpha.ln())).into();
    assert!(diff.abs().sum() <= f64::EPSILON * 2.);
}

#[test]
fn powc () {
    let alpha : Complxd = random();
    let beta : Complxd = random();

    let diff : EucVecd2 = (alpha.powc(beta) - Complxd::exp(beta * alpha.ln())).into();
    assert!(diff.abs().sum() <= f64::EPSILON * 2.);
}

#[test]
fn sin () {
    let alpha : Complxd = random();
    assert_eq!(alpha.sin(), Complxd::new(alpha.re().sin() * alpha.im().cosh(), alpha.re().cos() * alpha.im().sinh()));
}

#[test]
fn cos () {
    let alpha : Complxd = random();
    assert_eq!(alpha.cos(), Complxd::new(alpha.re().cos() * alpha.im().cosh(), -alpha.re().sin() * alpha.im().sinh()));
}

#[test]
fn tan () {
    let alpha : Complxd = random();
    let beta = Complxd::new(2. * alpha.re(), 2. * alpha.im());

    let div = beta.re().cos() + beta.im().cosh();
    assert_eq!(alpha.tan(), Complxd::new(
        beta.re().sin() / div,
        beta.im().sinh() / div
    ));
}
#[test]
fn expi () {
    let alpha : f64 = random();
    assert_eq!(Complxd::expi(alpha), Complxd::new(alpha.cos(), alpha.sin()))
}

#[test]
fn sqrtc () {
    let alpha : f64 = random();
    assert_eq!(alpha.sqrtc(), Complxd::new(alpha.sqrt(), 0.));
    assert_eq!((-alpha).sqrtc(), Complxd::new(0., alpha.sqrt()))
}