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

macro_rules! depr_names {
    ($($from:ident, $to:ident),+) => {
        $(
            #[deprecated(note = "use new naming convention instead")]    
            pub type $from = $to; 
        )*
    }
}

/// Euclidian vector types
pub mod vec {
    import!(EucVec2f, EucVec3f, EucVec4f);
    import!(EucVec2d, EucVec3d, EucVec4d);
    //import!(EucVec2sb, EucVec2ss, EucVec2si, EucVec2sl);
    
    depr_names!(
        EucVecf2, EucVec2f,
        EucVecf3, EucVec3f,
        EucVecf4, EucVec4f
    );

    depr_names!(
        EucVecd2, EucVec2d,
        EucVecd3, EucVec3d,
        EucVecd4, EucVec4d
    );
}

/// Matrix types
pub mod mat {
    import!(Mat2f, Mat3f);
    import!(Mat2d, Mat3d);

    depr_names!(
        Matf2, Mat2f,
        Matf3, Mat3f
    );

    depr_names!(
        Matd2, Mat2d,
        Matd3, Mat3d
    );
}

/// Other data types & traits
pub mod others {
    pub use crate::generics::Complxf;
    pub use crate::generics::Complxd;
    pub use crate::traits::*;
}