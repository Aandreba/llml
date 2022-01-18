use llml::{Complxf, traits::{ComplexSqrt, Sqrt}};
use rand::random;


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
    let polar = alpha.polar();
    assert_eq!(alpha.ln(), Complxf::new(polar.radius.ln(), polar.angle));
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