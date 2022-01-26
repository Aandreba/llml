macro_rules! arm_use {
    () => {
        #[cfg(target_arch = "arm")]
        use core::arch::arm::*;

        #[cfg(target_arch = "aarch64")]
        use core::arch::aarch64::*;
    };
}

macro_rules! wrap {
    ($($name:ident, $og:ident),+) => {
        $(
            #[repr(transparent)]
            pub struct $name (pub(crate) $og);

            impl Copy for $name {}
            impl Eq for $name {}
        )*
    };
}

macro_rules! impl_eq {
    ($target:ident, $ty:ident, $len:literal, $($tag:ident)?) => {
        impl PartialEq for $target {
            #[inline(always)]
            fn eq (&self, other: &Self) -> bool {
                unsafe { concat_idents!(vaddv, $($tag,)? _, $ty)(concat_idents!(vabd, $($tag,)? _, $ty)(self.0, other.0)) == 0. }
            }
        }
    };
}

macro_rules! impl_vec {
    ($target:ident, $ty:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl_vec!(1, $target, $ty, $trait, $fun,);
        )*

        impl Neg for $target {
            type Output = Self;

            #[inline(always)]
            fn neg (self) -> Self::Output {
                unsafe { Self(concat_idents!(vneg, _, $ty)(self.0)) }
            }
        }

        impl Clone for $target {
            #[inline(always)]
            fn clone (&self) -> Self {
                unsafe { Self(concat_idents!(vld1, _dup_, $ty)(addr_of!(self.0).cast())) }
            }
        }

        impl $target {
            #[inline(always)]
            pub fn sum (self) -> $ty {
                unsafe { concat_idents!(vaddv, _, $ty)(self.0) }
            }
        }
    };
    
    ($target:ident, $ty:ident, $tag:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl_vec!(1, $target, $ty, $trait, $fun, $tag);
        )*

        impl Neg for $target {
            type Output = Self;

            #[inline(always)]
            fn neg (self) -> Self::Output {
                unsafe { Self(concat_idents!(vneg, $tag, _, $ty)(self.0)) }
            }
        }

        impl Clone for $target {
            #[inline(always)]
            fn clone (&self) -> Self {
                unsafe { Self(concat_idents!(vld1, $tag, _dup_, $ty)(addr_of!(self.0).cast())) }
            }
        }

        impl $target {
            #[inline(always)]
            pub fn sum (self) -> $ty {
                unsafe { concat_idents!(vaddv, $tag, _, $ty)(self.0) }
            }
        }
    };

    (1, $target:ident, $ty:ident, $trait:ident, $fun:ident, $($tag:ident)?) => {
        impl $trait for $target {
            type Output = Self;

            #[inline(always)]
            fn $fun (self, rhs: Self) -> Self::Output {
                unsafe { Self(concat_idents!(v, $fun, $($tag,)? _, $ty)(self.0, rhs.0)) }
            }
        }

        impl $trait<$ty> for $target {
            type Output = Self;

            #[inline(always)]
            fn $fun (self, rhs: $ty) -> Self::Output {
                self.$fun(Self::from_scal(rhs))
            }
        }

        impl $trait<$target> for $ty {
            type Output = $target;

            #[inline(always)]
            fn $fun (self, rhs: $target) -> Self::Output {
                $target::from_scal(self).$fun(rhs)
            }
        }
    };
}

macro_rules! impl_vec2 {
    ($($target:ident, $ty:ident),+) => {
        $(
            impl_vec2!(1, $target, $ty, );
            impl_eq!($target, $ty, 2, );
        )*
    };

    ($($target:ident, $ty:ident, $tag:ident),+) => {
        $(
            impl_vec2!(1, $target, $ty, $tag);
            impl_eq!($target, $ty, 2, $tag);
        )*
    };

    (1, $target:ident, $ty:ident, $($tag:ident)?) => {
        impl_vec!(
            $target, $ty $(,$tag)?,
            Add, add,
            Sub, sub,
            Mul, mul,
            Div, div
        );

        impl $target {
            #[inline]
            pub fn new (a: [$ty;2]) -> Self {
                unsafe { Self(concat_idents!(vld1, $($tag,)? _, $ty)(a.as_ptr().cast())) }
            }

            #[inline]
            pub fn from_scal (x: $ty) -> Self {
                unsafe { Self(concat_idents!(vld1, $($tag,)? _dup_, $ty)(&x)) }
            }

            #[inline(always)]
            pub fn x (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 0) }
            }

            #[inline(always)]
            pub fn y (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 1) }
            }

            #[inline(always)]
            pub fn abs (self) -> Self {
                unsafe { Self(concat_idents!(vabs, $($tag,)? _, $ty)(self.0)) }
            }

            #[inline(always)]
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
            }

            #[inline(always)]
            #[deprecated(since="0.2.0", note="use ```self.dot(self)``` instead")]
            pub fn norm2 (self) -> $ty {
                self.dot(self)
            }

            #[inline(always)]
            pub fn norm (self) -> $ty {
                self.x().hypot(self.y())
            }

            #[inline(always)]
            pub fn unit (self) -> Self {
                self / self.norm()
            }

            #[inline(always)]
            pub fn sqrt (self) -> Self {
                unsafe {
                    Self(concat_idents!(vsqrt, $($tag,)? _, $ty)(self.0))
                }
            }

            #[inline(always)]
            pub fn sqrt_fast (self) -> Self {
                self.sqrt()
            }
        }

        impl Into<[$ty;2]> for $target {
            #[inline(always)]
            fn into (self) -> [$ty;2] {
                unsafe { transmute(self) }
            }
        }
    };
}

macro_rules! impl_vec3 {
    ($($target:ident, $ty:ident),+) => {
        $(
            impl_vec3!(1, $target, $ty,);
            impl_eq!($target, $ty, 3,);
        )*
    };

    ($($target:ident, $ty:ident, $tag:ident),+) => {
        $(
            impl_vec3!(1, $target, $ty, $tag);
            impl_eq!($target, $ty, 3, $tag);
        )*
    };

    (1, $target:ident, $ty:ident, $($tag:ident)?) => {
        impl_vec!(
            $target, $ty $(,$tag)?,
            Add, add,
            Sub, sub,
            Mul, mul
        );

        impl Div for $target {
            type Output = Self;

            #[inline(always)]
            fn div (self, rhs: Self) -> Self::Output {
                unsafe {
                    let div = concat_idents!(vdiv, $($tag,)? _, $ty)(self.0, rhs.0);
                    Self(concat_idents!(vset, $($tag,)? _lane_, $ty)(0., div, 3))
                }
            }
        }

        impl Div<$ty> for $target {
            type Output = Self;

            #[inline(always)]
            fn div (self, rhs: $ty) -> Self::Output {
                self.div(Self::from_scal(rhs))
            }
        }

        impl Div<$target> for $ty {
            type Output = $target;

            #[inline(always)]
            fn div (self, rhs: $target) -> Self::Output {
                $target::from_scal(self).div(rhs)
            }
        }

        impl $target {
            #[inline]
            pub fn new (a: [$ty;3]) -> Self {
                unsafe { Self(transmute([a[0], a[1], a[2], 0.])) }
            }

            #[inline]
            pub fn from_scal (x: $ty) -> Self {
                Self::new([x, x, x])
            }

            #[inline(always)]
            pub fn x (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 0) }
            }

            #[inline(always)]
            pub fn y (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 1) }
            }

            #[inline(always)]
            pub fn z (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 2) }
            }

            #[inline(always)]
            pub fn abs (self) -> Self {
                unsafe { Self(concat_idents!(vabs, $($tag,)? _, $ty)(self.0)) }
            }

            #[inline(always)]
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
            }

            #[inline(always)]
            #[deprecated(since="0.2.0", note="use ```self.dot(self)``` instead")]
            pub fn norm2 (self) -> $ty {
                self.dot(self)
            }

            #[inline(always)]
            pub fn norm (self) -> $ty {
                self.dot(self).sqrt()
            }

            #[inline(always)]
            pub fn unit (self) -> Self {
                self / self.norm()
            }

            #[inline(always)]
            pub fn cross (self, rhs: Self) -> Self {
                let v1 = EucVecf3::new([self.y(), self.z(), self.x()]);
                let v2 = EucVecf3::new([rhs.z(), rhs.x(), rhs.y()]);
                let m1 = v1 * v2;

                let v1 = EucVecf3::new([self.z(), self.x(), self.y()]);
                let v2 = EucVecf3::new([rhs.y(), rhs.z(), rhs.x()]);
                let m2 = v1 * v2;
                
                m1 - m2
            }

            #[inline(always)]
            pub fn sqrt (self) -> Self {
                unsafe {
                    Self(concat_idents!(vsqrt, $($tag,)? _, $ty)(self.0))
                }
            }

            #[inline(always)]
            pub fn sqrt_fast (self) -> Self {
                self.sqrt()
            }
        }

        impl Into<[$ty;3]> for $target {
            #[inline(always)]
            fn into (self) -> [$ty;3] {
                unsafe { *(&self as *const Self as *const [$ty;3]) }
            }
        }
    };
}

macro_rules! impl_vec4 {
    ($($target:ident, $ty:ident),+) => {
        $(
            impl_vec4!(1, $target, $ty,);
            impl_eq!($target, $ty, 4,);
        )*
    };

    ($($target:ident, $ty:ident, $tag:ident),+) => {
        $(
            impl_vec4!(1, $target, $ty, $tag);
            impl_eq!($target, $ty, 4, $tag);
        )*
    };

    (1, $target:ident, $ty:ident, $($tag:ident)?) => {
        impl_vec!(
            $target, $ty $(,$tag)?,
            Add, add,
            Sub, sub,
            Mul, mul,
            Div, div
        );

        impl $target {
            #[inline]
            pub fn new (x: [$ty;4]) -> Self {
                unsafe { Self(vld1q_f32(x.as_ptr().cast())) }
            }

            #[inline]
            pub fn from_scal (x: f32) -> Self {
                unsafe { Self(vld1q_dup_f32(&x)) }
            }

            #[inline(always)]
            pub fn x (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 0) }
            }

            #[inline(always)]
            pub fn y (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 1) }
            }

            #[inline(always)]
            pub fn z (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 2) }
            }

            #[inline(always)]
            pub fn w (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 3) }
            }

            #[inline(always)]
            pub fn abs (self) -> Self {
                unsafe { Self(concat_idents!(vabs, $($tag,)? _, $ty)(self.0)) }
            }

            #[inline(always)]
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
            }

            #[inline(always)]
            #[deprecated(since="0.2.0", note="use ```self.dot(self)``` instead")]
            pub fn norm2 (self) -> $ty {
                self.dot(self)
            }

            #[inline(always)]
            pub fn norm (self) -> $ty {
                self.dot(self).sqrt()
            }

            #[inline(always)]
            pub fn unit (self) -> Self {
                self / self.norm()
            }

            #[inline(always)]
            pub fn sqrt (self) -> Self {
                unsafe {
                    Self(concat_idents!(vsqrt, $($tag,)? _, $ty)(self.0))
                }
            }

            #[inline(always)]
            pub fn sqrt_fast (self) -> Self {
                self.sqrt()
            }
        }

        impl Into<[$ty;4]> for $target {
            #[inline(always)]
            fn into (self) -> [$ty;4] {
                unsafe { *(&self as *const Self as *const [$ty;4]) }
            }
        }
    };
}

macro_rules! impl_mat2 {
    ($target:ident, $ty:ident) => {
        impl_mat2!(
            $target, $ty,
            Add, add,
            Sub, sub
        );

        impl_mat2_scal!(
            $target, $ty,
            Mul, mul,
            Div, div
        );

        impl $target {
            #[inline(always)]
            pub fn scal_mul (self, rhs: Self) -> Self {
                Self(self.0 * rhs.0)
            }

            #[inline(always)]
            pub fn scal_div (self, rhs: Self) -> Self {
                Self(self.0 / rhs.0)
            }

            #[inline(always)]
            pub fn transp (self) -> Self {
                Self::new([self.xx(), self.yx(), self.xy(), self.yy()])
            }
        }
    };

    ($target:ident, $ty:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0.$fun(rhs.0))
                }
            }

            impl_mat2_scal!($target, $ty, $trait, $fun);
        )*
    };
}

macro_rules! impl_mat2_scal {
    ($target:ident, $ty:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl $trait<$ty> for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: $ty) -> Self::Output {
                    Self(self.0.$fun(rhs))
                }
            }

            impl $trait<$target> for $ty {
                type Output = $target;

                #[inline(always)]
                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self.$fun(rhs.0))
                }
            }
        )*
    };
}

flat_mod!(double, complex);
flat_mod!(vec2, vec3, vec4);
flat_mod!(mat2, mat3);