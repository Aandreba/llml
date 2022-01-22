macro_rules! import {
    ($($i:ident),+) => {
        $(
            #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
            pub use crate::arm::$i;

            #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
            pub use crate::x86::$i;
        )*
    };
}

pub mod vec {
    import!(EucVecf2, EucVecf3, EucVecf4);
    import!(EucVecd2, EucVecd3, EucVecd4);
}

pub mod mat {
    import!(Matf2, Matf3);
    import!(Matd2, Matd3);
}

pub mod others {
    pub use crate::generics::Complxf;
    pub use crate::generics::Complxd;
    pub use crate::polar::Polar;
    pub use crate::traits::*;
}