use llml::{mat::{Matf2, Matd2}, vec::EucVecf4};
use rand::random;

macro_rules! test_arith {
    ($sy:tt) => {
        let (alpha, beta) = get_mats();
        assert_eq!(alpha $sy beta, Matf2::new([
            alpha.xx() $sy beta.xx(), alpha.xy() $sy beta.xy(),
            alpha.yx() $sy beta.yx(), alpha.yy() $sy beta.yy()
        ]));
    }
}

#[cfg(feature = "llml_serde")]
#[test]
fn serde () {
    let alpha : Matf2 = random();
    let json = serde_json::to_string(&alpha).unwrap();
    let beta : Matf2 = serde_json::from_str(json.as_str()).unwrap();

    let diff : [f32;4] = (alpha - beta).into();
    assert!(diff.into_iter().sum::<f32>() <= f32::EPSILON * 4.);
}

#[test]
fn eq () {
    assert_eq!(Matf2::new([1., 2., 3., 4.]), Matf2::new([1., 2., 3., 4.]));
    assert_ne!(Matf2::new([1., 2., 3., 4.]), Matf2::new([5., 6., 7., 8.]))
}

#[test]
fn into () {
    assert_eq!(Into::<Matd2>::into(Matf2::new([1., 2., 3., 4.])), Matd2::new([1., 2., 3., 4.]));
    assert_eq!(Into::<[f32;4]>::into(Matf2::new([1., 2., 3., 4.])), [1., 2., 3., 4.])
}

#[test]
fn rot () {
    let angle : f32 = random();
    assert_eq!(Matf2::of_rot(angle), Matf2::new([angle.cos(), -angle.sin(), angle.sin(), angle.cos()]));
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
    let alpha = Matf2::new([1., 2., 3., 4.]);
    let beta = Matf2::new([5., 6., 7., 8.]);
    assert_eq!(alpha * beta, Matf2::new([19., 22., 43., 50.]));
}

#[test]
fn scal_mul () {
    let alpha : EucVecf4 = random();
    let beta : EucVecf4 = random();
    let mul = alpha * beta;

    let a = Matf2::new(alpha.into());
    let b = Matf2::new(beta.into());
    assert_eq!(a.scal_mul(b), Matf2::new(mul.into()));
}

#[test]
fn scal_div () {
    let alpha : EucVecf4 = random();
    let beta : EucVecf4 = random();
    let mul = alpha * beta;

    let a = Matf2::new(alpha.into());
    let b = Matf2::new(beta.into());
    assert_eq!(a.scal_mul(b), Matf2::new(mul.into()));
}

#[test]
fn neg () {
    let alpha : Matf2 = random();
    assert_eq!(-alpha, Matf2::new([-alpha.xx(), -alpha.xy(), -alpha.yx(), -alpha.yy()]))
}

#[test]
fn tr () {
    let alpha = Matf2::new([1., 2., 3., 4.]);
    assert_eq!(alpha.tr(), 5.)
}

#[test]
fn det () {
    let alpha = Matf2::new([1., 2., 3., 4.]);
    assert_eq!(alpha.det(), -2.)
}

#[test]
fn inv () {
    let alpha = Matf2::new([1., 2., 3., 4.]);
    assert_eq!(alpha.inv(), Some(Matf2::new([-2., 1., 3./2., -1./2.])));
}

fn get_mats () -> (Matf2, Matf2) {
    (random(), random())
}