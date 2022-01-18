use llml::{generics::Complxd, traits::{ComplexSqrt}};
use rand::random;


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
    let polar = alpha.polar();
    assert_eq!(alpha.ln(), Complxd::new(polar.radius.ln(), polar.angle));
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