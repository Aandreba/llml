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
    EucVecf2, EucVecd2,
    Matf2, Matd2
);

serialize_len3!(
    EucVecf3, EucVecd3,
    Matf3, Matd3
);

serialize_len4!(
    EucVecf4, EucVecd4
);

