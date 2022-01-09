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
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct $name (pub(crate) $og);

            impl Copy for $name {}
            impl Eq for $name {}
        )*
    };
}

macro_rules! impl_eq {
    ($target:ident, $ty:ident, $eq:ident, $len:literal, $($tag:ident)?) => {
        impl PartialEq for $target {
            #[inline(always)]
            fn eq (&self, other: &Self) -> bool {
                unsafe { concat_idents!(vaddv, $($tag,)? _, $eq)(concat_idents!(vceq, $($tag,)? _, $ty)(self.0, other.0)) == $len }
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

            #[inline(always)]
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
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
                unsafe { Self(concat_idents!(v, $fun, $($tag,)? _, $ty)(self.0, concat_idents!(vld1, $($tag,)? _dup_, $ty)(&rhs))) }
            }
        }

        impl $trait<$target> for $ty {
            type Output = $target;

            #[inline(always)]
            fn $fun (self, rhs: $target) -> Self::Output {
                unsafe { $target(concat_idents!(v, $fun, $($tag,)? _, $ty)(concat_idents!(vld1, $($tag,)? _dup_, $ty)(&self), rhs.0)) }
            }
        }
    };
}

macro_rules! impl_vec2 {
    ($($target:ident, $ty:ident, $eq:ident),+) => {
        $(
            impl_vec2!(1, $target, $ty, );
            impl_eq!($target, $ty, $eq, 2, );
        )*
    };

    ($($target:ident, $ty:ident, $tag:ident, $eq:ident),+) => {
        $(
            impl_vec2!(1, $target, $ty, $tag);
            impl_eq!($target, $ty, $eq, 2, $tag);
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
            pub fn new (x: $ty, y: $ty) -> Self {
                unsafe { Self(transmute([x, y])) }
            }

            #[inline(always)]
            pub fn x (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 0) }
            }

            #[inline(always)]
            pub fn y (&self) -> $ty {
                unsafe { concat_idents!(vget, $($tag,)? _lane, _, $ty)(self.0, 1) }
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
    ($($target:ident, $ty:ident, $eq:ident),+) => {
        $(
            impl_vec3!(1, $target, $ty,);
            impl_eq!($target, $ty, $eq, 3,);
        )*
    };

    ($($target:ident, $ty:ident, $tag:ident, $eq:ident),+) => {
        $(
            impl_vec3!(1, $target, $ty, $tag);
            impl_eq!($target, $ty, $eq, 3, $tag);
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
            pub fn new (x: $ty, y: $ty, z: $ty) -> Self {
                unsafe { Self(transmute([x, y, z, 0.])) }
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
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
            }

            #[inline]
            pub fn cross (self, rhs: Self) -> Self {
                todo!()
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
    ($($target:ident, $ty:ident, $eq:ident),+) => {
        $(
            impl_vec4!(1, $target, $ty,);
            impl_eq!($target, $ty, $eq, 4,);
        )*
    };

    ($($target:ident, $ty:ident, $tag:ident, $eq:ident),+) => {
        $(
            impl_vec4!(1, $target, $ty, $tag);
            impl_eq!($target, $ty, $eq, 4, $tag);
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
            pub fn new (x: $ty, y: $ty, z: $ty, w: $ty) -> Self {
                unsafe { Self(transmute([x, y, z, w])) }
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
            pub fn dot (self, rhs: Self) -> $ty {
                (self * rhs).sum()
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

flat_mod!(vec2, vec3, vec4);
flat_mod!(mat2);