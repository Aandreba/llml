#[macro_export]
macro_rules! frac_arith {
    ($t:ident, $i:ty, $n:ident, $f:expr) => {
        frac_arith!($t, $i, $n, $f, );
    };

    ($t:ident, $i:ty, $o:ty, $n:ident, $f:expr) => {
        frac_arith!($t, $i, $o, $n, $f, );
    };

    ($t:ident, $i:ty, $n:ident, $f:expr, $($id:ident),*) => {
        frac_arith!($t, $i, $i, $n, $f $(,$id)*);
    };

    ($t:ident, $i:ty, $o:ty, $n: ident, $f:expr, $($id:ident),*) => {
        impl<T: Num + Copy + $($id + )* $t<T>> $t<$i> for $i {
            type Output = $o;
        
            fn $n(self, rhs: $i) -> Self::Output {
                $f(&self, &rhs)
            }
        }

        impl<T: Num + Copy + $($id + )* $t<T>> $t<&$i> for $i {
            type Output = $o;
        
            fn $n(self, rhs: &$i) -> Self::Output {
                $f(&self, rhs)
            }
        }

        impl<T: Num + Copy + $($id + )* $t<T>> $t<$i> for &$i {
            type Output = $o;
        
            fn $n(self, rhs: $i) -> Self::Output {
                $f(self, &rhs)
            }
        }

        impl<T: Num + Copy + $($id + )* $t<T>> $t<&$i> for &$i {
            type Output = $o;
        
            fn $n(self, rhs: &$i) -> Self::Output {
                $f(self, rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! frac_scal_arith {
    ($t:ident, $i:ty, $n:ident, $f:expr) => {
        frac_scal_arith!($t, $i, $n, $f, );
    };

    ($t:ident, $i:ty, $o:ty, $n:ident, $f:expr) => {
        frac_scal_arith!($t, $i, $o, $n, $f, );
    };

    ($t:ident, $i:ty, $n:ident, $f:expr, $($id:ident),*) => {
        frac_scal_arith!($t, $i, $i, $n, $f $(,$id)*);
    };

    ($t:ident, $i:ty, $o:ty, $n: ident, $f:expr, $($id:ident),*) => {
        impl<T: Num + Copy + $($id + )* $t<T>> $t<T> for $i {
            type Output = $o;
        
            fn $n(self, rhs: T) -> Self::Output {
                $f(&self, &rhs)
            }
        }

        impl<T: Num + Copy + $($id + )* $t<T>> $t<&T> for $i {
            type Output = $o;
        
            fn $n(self, rhs: &T) -> Self::Output {
                $f(&self, rhs)
            }
        }

        impl<T: Num + Copy + $($id + )* $t<T>> $t<T> for &$i {
            type Output = $o;
        
            fn $n(self, rhs: T) -> Self::Output {
                $f(self, &rhs)
            }
        }

        impl<T: Num + Copy + $($id + )* $t<T>> $t<&T> for &$i {
            type Output = $o;
        
            fn $n(self, rhs: &T) -> Self::Output {
                $f(self, rhs)
            }
        }
    };
}