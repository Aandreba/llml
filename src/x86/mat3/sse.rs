use std::{ops::{Add, Sub, Mul, Div}, intrinsics::transmute};
use crate::{mat::Matf3, vec::{EucVecf3, EucVecf4}};
use_arch_x86!(_mm_set_ps, _mm_set1_ps, _mm_mul_ps, _mm_add_ps, _mm_sub_ps, _mm_div_ps, __m128);

macro_rules! impl_arith {
    ($($trait:ident, $fn:ident, $sy:tt),+) => {
        $(
            impl $trait for Matf3 {
                type Output = Matf3;
            
                #[inline(always)]
                fn $fn (self, rhs: Self) -> Self::Output {
                    unsafe {
                        let s1 = _mm_set_ps(self.y.x, self.x.z, self.x.y, self.x.x);
                        let s2 = _mm_set_ps(self.z.y, self.z.x, self.y.z, self.y.y);
            
                        let r1 = _mm_set_ps(rhs.y.x, rhs.x.z, rhs.x.y, rhs.x.x);
                        let r2 = _mm_set_ps(rhs.z.y, rhs.z.x, rhs.y.z, rhs.y.y);
            
                        Self::from(concat_idents!(_mm_, $fn, _ps)(s1, r1), concat_idents!(_mm_, $fn, _ps)(s2, r2), self.z.z $sy rhs.z.z)
                    }
                }
            }

            impl_scal_arith!($trait, $fn, $sy);
        )*
    };
}

macro_rules! impl_scal_arith {
    ($($trait:ident, $fn:ident, $sy:tt),+) => {
        $(
            impl $trait<f32> for Matf3 {
                type Output = Matf3;
            
                #[inline(always)]
                fn $fn (self, rhs: f32) -> Self::Output {
                    unsafe {
                        let s1 = _mm_set_ps(self.y.x, self.x.z, self.x.y, self.x.x);
                        let s2 = _mm_set_ps(self.z.y, self.z.x, self.y.z, self.y.y);
                        let r = _mm_set1_ps(rhs);        

                        Self::from(concat_idents!(_mm_, $fn, _ps)(s1, r), concat_idents!(_mm_, $fn, _ps)(s2, r), self.z.z $sy rhs)
                    }
                }
            }

            impl $trait<Matf3> for f32 {
                type Output = Matf3;
            
                #[inline(always)]
                fn $fn (self, rhs: Matf3) -> Self::Output {
                    unsafe {
                        let s = _mm_set1_ps(self);        
                        let r1 = _mm_set_ps(rhs.y.x, rhs.x.z, rhs.x.y, rhs.x.x);
                        let r2 = _mm_set_ps(rhs.z.y, rhs.z.x, rhs.y.z, rhs.y.y);

                        Matf3::from(concat_idents!(_mm_, $fn, _ps)(s, r1), concat_idents!(_mm_, $fn, _ps)(s, r2), self $sy rhs.z.z)
                    }
                }
            }
        )*
    }
}

impl_arith!(
    Add, add, +,
    Sub, sub, -
);

impl_scal_arith!(
    Mul, mul, *,
    Div, div, /
);

impl Mul for Matf3 {
    type Output = Matf3;

    #[inline(always)]
    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            // SECTION 1 (DONE)
            let v1 = _mm_set_ps(self.y.x, self.x.x, self.x.x, self.x.x);
            let v2 = _mm_set_ps(rhs.x.x, rhs.x.z, rhs.x.y, rhs.x.x);
            let m11 = _mm_mul_ps(v1, v2);   

            let v3 = _mm_set_ps(self.y.y, self.x.y, self.x.y, self.x.y);
            let v4 = _mm_set_ps(rhs.y.x, rhs.y.z, rhs.y.y, rhs.y.x);
            let m12 = _mm_mul_ps(v3, v4);

            let v5 = _mm_set_ps(self.y.z, self.x.z, self.x.z, self.x.z);
            let v6 = _mm_set_ps(rhs.z.x, rhs.z.z, rhs.z.y, rhs.z.x);
            let m13 = _mm_mul_ps(v5, v6);

            // SECTION 2 (DONE)
            let v1 = _mm_set_ps(self.z.x, self.z.x, self.y.x, self.y.x);
            let v2 = _mm_set_ps(rhs.x.y, rhs.x.x, rhs.x.z, rhs.x.y);
            let m21 = _mm_mul_ps(v1, v2);

            let v3 = _mm_set_ps(self.z.y, self.z.y, self.y.y, self.y.y);
            let v4 = _mm_set_ps(rhs.y.y, rhs.y.x, rhs.y.z, rhs.y.y);
            let m22 = _mm_mul_ps(v3, v4);

            let v5 = _mm_set_ps(self.z.z, self.z.z, self.y.z, self.y.z);
            let v6 = _mm_set_ps(rhs.z.y, rhs.z.x, rhs.z.z, rhs.z.y);
            let m23 = _mm_mul_ps(v5, v6);

            // SUMS
            let s1 = _mm_add_ps(m11, _mm_add_ps(m12, m13));
            let s2 = _mm_add_ps(m21, _mm_add_ps(m22, m23));
            let s3 = EucVecf3::raw_dot(self.z.casted(), transmute([rhs.x.z, rhs.y.z, rhs.z.z, 0.]));
            Self::from(s1, s2, s3)
        }
    }
}

impl Mul<EucVecf3> for Matf3 {
    type Output = EucVecf3;

    #[inline(always)]
    fn mul (self, rhs: EucVecf3) -> Self::Output {
        unsafe {
            let v1 = _mm_set_ps(self.y.x, self.x.z, self.x.y, self.x.x);
            let v2 = _mm_set_ps(rhs.x, rhs.z, rhs.y, rhs.x);

            let v3 = _mm_set_ps(self.z.y, self.z.x, self.y.z, self.y.y);
            let v4 = _mm_set_ps(rhs.y, rhs.x, rhs.z, rhs.y);
            
            let m1 = &_mm_mul_ps(v1, v2) as *const __m128 as *const f32;
            let m2 = &_mm_mul_ps(v3, v4) as *const __m128 as *const f32;
            let m3 = self.z.z * rhs.z;
            
            let v5 = _mm_set_ps(0., *m2.add(2), *m1.add(3), *m1);
            let v6 = _mm_set_ps(0., *m2.add(3), *m2, *m1.add(1));
            let v7 = _mm_set_ps(0., m3, *m2.add(1), *m1.add(2));

            EucVecf3::unsafe_from(_mm_add_ps(v5, _mm_add_ps(v6, v7)))
        }
    }
}

impl Matf3 {
    #[inline(always)]
    pub(crate) unsafe fn from (alpha: __m128, beta: __m128, gamma: f32) -> Self {
        let alpha = &alpha as *const __m128 as *const f32;
        let beta = &beta as *const __m128 as *const f32;

        Self::new(
            *(alpha as *const EucVecf3), 
            EucVecf3::new(*alpha.add(3), *beta, *beta.add(1)), 
            EucVecf3::new(*beta.add(2), *beta.add(3), gamma)
        )
    }

    #[inline(always)]
    pub fn det (self) -> f32 {
        unsafe {
            let v1 = _mm_set_ps(self.x.y, -self.x.y, -self.x.x, self.x.x);
            let v2 = _mm_set_ps(self.y.z, self.y.x, self.y.z, self.y.y);
            let v3 = _mm_set_ps(self.z.x, self.z.z, self.z.y, self.z.z);
            
            let m1 = _mm_mul_ps(v1, _mm_mul_ps(v2, v3));
            let s1 = EucVecf4::raw_sum(m1);

            let v4 = _mm_set_ps(0., s1, -self.x.z, self.x.z);
            let v5 = _mm_set_ps(0., 1., self.y.y, self.y.x);
            let v6 = _mm_set_ps(0., 1., self.z.x, self.z.y);

            let m2 = _mm_mul_ps(v4, _mm_mul_ps(v5, v6));
            EucVecf4::raw_sum(m2)
        }
    }
}