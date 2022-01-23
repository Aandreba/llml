macro_rules! impl_assign {
    ($target:ident, $($assign:ident, $assign_fun:ident, $fun:ident),+) => {
        $(
            impl_assign!(1, $target, Self, $assign, $assign_fun, $fun);
        )*
    };
    
    (1, $target:ident, $rhs:ty, $($assign:ident, $assign_fun:ident, $fun:ident),+) => {
        $(
            impl $assign<$rhs> for $target {
                #[inline(always)]
                fn $assign_fun (&mut self, rhs: $rhs) {
                    *self = self.$fun(rhs);
                }
            }
        )*
    }
}

flat_mod!(complex, vec, mat);

#[cfg(feature="llml_rand")]
pub mod rand;

#[cfg(feature="llml_serde")]
pub mod serde;