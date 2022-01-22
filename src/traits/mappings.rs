macro_rules! map1 {
    (1, $name:ident, $fun:ident, $($in:ident),+) => {
        map1!(
            0, $name, $fun
            $(, $in, $in)*
        );
    };

    (0, $name:ident, $fun:ident, $($in:ident, $out:ident),+) => {
        pub trait $name {
            type Output;

            fn $fun (self) -> Self::Output;
        }

        $(
            impl $name for $in {
                type Output = $out;

                fn $fun (self) -> Self::Output {
                    $in::$fun(self)
                }
            }
        )*
    };
}

macro_rules! map1_float {
    ($name:ident, $fun:ident) => {
        map1!(1, $name, $fun, f32, f64);
    };

    ($name:ident, $fun:ident, $out32:ident, $out64:ident) => {
        map1!(
            0, $name, $fun,
            f32, $out32,
            f64, $out64  
        );
    };
}

map1_float!(
    Sqrt, sqrt
);