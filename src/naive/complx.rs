macro_rules! impl_inverse {
    ($trait:ident, $fn:ident, $symbol:tt, $($target:ident),*) => {
        $(
            impl $trait<Complxf<$target>> for $target {
                type Output = Complxf<$target>;
            
                fn $fn(self, rhs: Complxf<$target>) -> Self::Output {
                    Complxf::new(self $symbol rhs.x, self $symbol rhs.y)
                }
            }
        )*
    };
}

macro_rules! impl_arith {
    ($trait:ident, $fn:ident, $symbol:tt) => {
        impl<T: $trait> $trait for Complxf<T> {
            type Output = Complxf<<T as $trait>::Output>;
        
            fn $fn(self, rhs: Self) -> Self::Output {
                Complxf::new(self.x $symbol rhs.x, self.y $symbol rhs.y)
            }
        }

        impl<T: $trait + Clone> $trait<T> for Complxf<T> {
            type Output = Complxf<<T as $trait>::Output>;
        
            fn $fn(self, rhs: T) -> Self::Output {
                Complxf::new(self.x $symbol rhs.clone(), self.y $symbol rhs)
            }
        }

        impl_inverse!($trait, $fn, $symbol, u8, u16, u32, u64, u128);
        impl_inverse!($trait, $fn, $symbol, i8, i16, i32, i64, i128);
        impl_inverse!($trait, $fn, $symbol, f32, f64);
    };
}

impl_arith!(
    Add, add,
    Sub, sub
);