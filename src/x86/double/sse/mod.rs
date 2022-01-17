macro_rules! impl_arith_x2 {
    ($target:ident) => {
        impl_arith_x2!(
            $target,
            Add, add,
            Sub, sub,
            Mul, mul,
            Div, div
        );

        impl Eq for $target {}
        impl Copy for $target {}

        impl Neg for $target {
            type Output = Self;

            #[inline(always)]
            fn neg (self) -> Self::Output {
                Self(-self.0, -self.1)
            }
        }

        impl Clone for $target {
            fn clone (&self) -> Self {
                Self(self.0.clone(), self.1.clone())
            }
        }

        impl PartialEq for $target {
            fn eq (&self, rhs: &Self) -> bool {
                (self.0 == rhs.0) && (self.1 == rhs.1)
            }
        }
    };

    ($target:ident, $($trait:ident, $fun:ident),+) => {
        $(
            impl $trait for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: Self) -> Self::Output {
                    Self(self.0.$fun(rhs.0), self.1.$fun(rhs.1))
                }
            }

            impl $trait<f64> for $target {
                type Output = Self;

                #[inline(always)]
                fn $fun (self, rhs: f64) -> Self::Output {
                    Self(self.0.$fun(rhs), self.1.$fun(rhs))
                }
            }

            impl $trait<$target> for f64 {
                type Output = $target;

                #[inline(always)]
                fn $fun (self, rhs: $target) -> Self::Output {
                    $target(self.$fun(rhs.0), self.$fun(rhs.1))
                }
            }
        )*
    };
}

flat_mod!(vec3, vec4);
flat_mod!(mat2);