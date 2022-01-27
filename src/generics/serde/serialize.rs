use serde::ser::{Serialize, SerializeStruct};
use crate::{vec::*, mat::*};

macro_rules! serialize_len2 {
    ($($target:ident),+) => {
        $(
            impl Serialize for $target {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                   let mut s = serializer.serialize_struct(stringify!($target), 2)?;
                   s.serialize_field("x", &self.x())?;
                   s.serialize_field("y", &self.y())?;
                   s.end()
                }
            }
        )*
    };
}

macro_rules! serialize_len3 {
    ($($target:ident),+) => {
        $(
            impl Serialize for $target {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                   let mut s = serializer.serialize_struct(stringify!($target), 3)?;
                   s.serialize_field("x", &self.x())?;
                   s.serialize_field("y", &self.y())?;
                   s.serialize_field("z", &self.z())?;
                   s.end()
                }
            }
        )*
    };
}

macro_rules! serialize_len4 {
    ($($target:ident),+) => {
        $(
            impl Serialize for $target {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                   let mut s = serializer.serialize_struct(stringify!($target), 4)?;
                   s.serialize_field("x", &self.x())?;
                   s.serialize_field("y", &self.y())?;
                   s.serialize_field("z", &self.z())?;
                   s.serialize_field("w", &self.w())?;
                   s.end()
                }
            }
        )*
    };
}

serialize_len2!(
    EucVec2f, EucVec2d,
    Mat2f, Mat2d
);

serialize_len3!(
    EucVec3f, EucVec3d,
    Mat3f, Mat3d
);

serialize_len4!(
    EucVec4f, EucVec4d
);

