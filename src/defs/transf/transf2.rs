use std::ops::{Mul, Neg, Add};

use crate::{vec::{EucVec2}, others::{Complx, SinCos, One, Zero}, mat::Mat3};
import_derives!();

pub type Transf2 = Trans2<f32>;
pub type Transd2 = Trans2<f64>;

/// Affine transformation in 2 dimension
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
pub struct Trans2<T> {
    pub pos: EucVec2<T>,
    pub scale: EucVec2<T>,
    pub rot: Complx<T>
}

impl<T> Trans2<T> {
    pub fn new (pos: EucVec2<T>, scale: EucVec2<T>, angle: T) -> Self where T: SinCos {
        Self { pos, scale, rot: Complx::expi(angle) }
    }

    pub fn newc (pos: EucVec2<T>, scale: EucVec2<T>, rot: Complx<T>) -> Self {
        Self { pos, scale, rot }
    }
}

impl<T: Default> Trans2<T> {
    pub fn from_pos (pos: EucVec2<T>) -> Self {
        Self::newc(pos, EucVec2::default(), Complx::default())
    }

    pub fn from_scale (scale: EucVec2<T>) -> Self {
        Self::newc(EucVec2::default(), scale, Complx::default())
    }

    pub fn from_scale1 (scale: T) -> Self where T: Clone {
        Self::newc(EucVec2::default(), EucVec2::new(scale.clone(), scale), Complx::default())
    }

    pub fn from_angle (angle: T) -> Self where T: SinCos {
        Self::new(EucVec2::default(), EucVec2::default(), angle)
    }
}

impl<T> Trans2<T>  {
    pub fn position_matrix (self) -> Mat3<T> where T: Zero + One {
        Mat3::of_values (
            T::one(), T::zero(), self.pos.x,
            T::zero(), T::one(), self.pos.y,
            T::zero(), T::zero(), T::one()
        )
    }

    pub fn scale_matrix (self) -> Mat3<T> where T: Zero + One {
        Mat3::of_values (
            self.scale.x, T::zero(), T::zero(),
            T::zero(), self.scale.y, T::zero(),
            T::zero(), T::zero(), T::one()
        )
    }

    pub fn rotation_matrix (self) -> Mat3<T> where T: Clone + Neg<Output = T> + Zero + One {
        Mat3::of_values (
            self.rot.re.clone(), -self.rot.im.clone(), T::zero(),
            self.rot.im, self.rot.re, T::zero(),
            T::zero(), T::zero(), T::one()
        )
    }

    #[inline]
    pub fn transform (self, point: EucVec2<T>) -> EucVec2<T> where T: Mul<Output = T>, Complx<T>: Mul<Output = Complx<T>>, EucVec2<T>: Add<Output = EucVec2<T>> + Mul<Output = EucVec2<T>> {
        let result = EucVec2::from(self.rot * point.into());
        (result * self.scale) + self.pos
    }
}

impl<T: Clone + Zero + One + Neg<Output = T>> Into<Mat3<T>> for Trans2<T> where Mat3<T>: Mul<Output = Mat3<T>> {
    fn into(self) -> Mat3<T> {
        self.clone().position_matrix() * self.clone().rotation_matrix() * self.scale_matrix()
    }
}