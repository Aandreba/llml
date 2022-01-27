use std::intrinsics::transmute;

use serde::{Deserialize, de::{Visitor, self}};
use crate::{vec::*, mat::*};

macro_rules! impl_vec2 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl<'de> Deserialize<'de> for $target {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                    struct LocalVisitor;
                    impl<'de> Visitor<'de> for LocalVisitor {
                        type Value = $target;
                    
                        fn expecting (&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str("a struct with params 'x' & 'y'")
                        }
            
                        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: de::Error, {
                            let array = *bytemuck::try_from_bytes::<[$ty;2]>(v).map_err(|_| E::invalid_length(v.len(), &self))?;
                            Ok($target::new(array))
                        }
            
                        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: de::Error, {
                            self.visit_bytes(v.as_slice())
                        }
                    
                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de>, {
                            let x : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                            let y : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            Ok($target::new([x, y]))
                        }
            
                        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: de::MapAccess<'de>, {
                            let mut x = None;
                            let mut y = None;
            
                            while let Some(key) = map.next_key::<&str>()? {
                                match key {
                                    "x" => {
                                        if x.is_some() { return Err(de::Error::duplicate_field("x")) }
                                        x = Some(map.next_value::<$ty>()?)
                                    }
            
                                    "y" => {
                                        if y.is_some() { return Err(de::Error::duplicate_field("y")) }
                                        y = Some(map.next_value::<$ty>()?)
                                    }
            
                                    _ => {}
                                }
                            }
                            
                            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
                            Ok($target::new([x, y]))
                        }
                    }
            
                    deserializer.deserialize_struct(stringify!($target), &["x", "y"], LocalVisitor)
                }
            }
        )*
    }
}

macro_rules! impl_vec3 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl<'de> Deserialize<'de> for $target {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                    struct LocalVisitor;
                    impl<'de> Visitor<'de> for LocalVisitor {
                        type Value = $target;
                    
                        fn expecting (&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str("a struct with params 'x', 'y' & 'z'")
                        }
            
                        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: de::Error, {
                            let array = *bytemuck::try_from_bytes::<[$ty;3]>(v).map_err(|_| E::invalid_length(v.len(), &self))?;
                            Ok($target::new(array))
                        }
            
                        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: de::Error, {
                            self.visit_bytes(v.as_slice())
                        }
                    
                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de>, {
                            let x : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                            let y : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            let z : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                            Ok($target::new([x, y, z]))
                        }
            
                        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: de::MapAccess<'de>, {
                            let mut x = None;
                            let mut y = None;
                            let mut z = None;
            
                            while let Some(key) = map.next_key::<&str>()? {
                                match key {
                                    "x" => {
                                        if x.is_some() { return Err(de::Error::duplicate_field("x")) }
                                        x = Some(map.next_value::<$ty>()?)
                                    }
            
                                    "y" => {
                                        if y.is_some() { return Err(de::Error::duplicate_field("y")) }
                                        y = Some(map.next_value::<$ty>()?)
                                    }

                                    "z" => {
                                        if z.is_some() { return Err(de::Error::duplicate_field("z")) }
                                        z = Some(map.next_value::<$ty>()?)
                                    }
            
                                    _ => {}
                                }
                            }
                            
                            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
                            let z = z.ok_or_else(|| de::Error::missing_field("z"))?;
                            Ok($target::new([x, y, z]))
                        }
                    }
            
                    deserializer.deserialize_struct(stringify!($target), &["x", "y", "z"], LocalVisitor)
                }
            }
        )*
    }
}

macro_rules! impl_vec4 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl<'de> Deserialize<'de> for $target {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                    struct LocalVisitor;
                    impl<'de> Visitor<'de> for LocalVisitor {
                        type Value = $target;
                    
                        fn expecting (&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str("a struct with params 'x', 'y', 'z' & 'w'")
                        }
            
                        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: de::Error, {
                            let array = *bytemuck::try_from_bytes::<[$ty;4]>(v).map_err(|_| E::invalid_length(v.len(), &self))?;
                            Ok($target::new(array))
                        }
            
                        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: de::Error, {
                            self.visit_bytes(v.as_slice())
                        }
                    
                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de>, {
                            let x : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                            let y : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            let z : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                            let w : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
                            Ok($target::new([x, y, z, w]))
                        }
            
                        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: de::MapAccess<'de>, {
                            let mut x = None;
                            let mut y = None;
                            let mut z = None;
                            let mut w = None;
            
                            while let Some(key) = map.next_key::<&str>()? {
                                match key {
                                    "x" => {
                                        if x.is_some() { return Err(de::Error::duplicate_field("x")) }
                                        x = Some(map.next_value::<$ty>()?)
                                    }
            
                                    "y" => {
                                        if y.is_some() { return Err(de::Error::duplicate_field("y")) }
                                        y = Some(map.next_value::<$ty>()?)
                                    }

                                    "z" => {
                                        if z.is_some() { return Err(de::Error::duplicate_field("z")) }
                                        z = Some(map.next_value::<$ty>()?)
                                    }

                                    "w" => {
                                        if w.is_some() { return Err(de::Error::duplicate_field("w")) }
                                        w = Some(map.next_value::<$ty>()?)
                                    }
            
                                    _ => {}
                                }
                            }
                            
                            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
                            let z = z.ok_or_else(|| de::Error::missing_field("z"))?;
                            let w = w.ok_or_else(|| de::Error::missing_field("w"))?;
                            Ok($target::new([x, y, z, w]))
                        }
                    }
            
                    deserializer.deserialize_struct(stringify!($target), &["x", "y", "z"], LocalVisitor)
                }
            }
        )*
    }
}

macro_rules! impl_mat2 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl<'de> Deserialize<'de> for $target {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                    struct LocalVisitor;
                    impl<'de> Visitor<'de> for LocalVisitor {
                        type Value = $target;
                    
                        fn expecting (&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str("a struct with params 'x' & 'y'")
                        }
                    
                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de>, {
                            let x : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                            let y : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            Ok($target::new(concat([x.into(), y.into()])))
                        }
            
                        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: de::MapAccess<'de>, {
                            let mut x = None;
                            let mut y = None;
            
                            while let Some(key) = map.next_key::<&str>()? {
                                match key {
                                    "x" => {
                                        if x.is_some() { return Err(de::Error::duplicate_field("x")) }
                                        x = Some(map.next_value::<$ty>()?)
                                    }
            
                                    "y" => {
                                        if y.is_some() { return Err(de::Error::duplicate_field("y")) }
                                        y = Some(map.next_value::<$ty>()?)
                                    }
            
                                    _ => {}
                                }
                            }
                            
                            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
                            Ok($target::new(concat([x.into(), y.into()])))
                        }
                    }
            
                    deserializer.deserialize_struct(stringify!($target), &["x", "y"], LocalVisitor)
                }
            }
        )*
    }
}

macro_rules! impl_mat3 {
    ($($target:ident, $ty:ty),+) => {
        $(
            impl<'de> Deserialize<'de> for $target {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                    struct LocalVisitor;
                    impl<'de> Visitor<'de> for LocalVisitor {
                        type Value = $target;
                    
                        fn expecting (&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str("a struct with params 'x', 'y' & 'z'")
                        }
                    
                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de>, {
                            let x : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                            let y : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            let z : $ty = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                            Ok($target::new(concat([x.into(), y.into(), z.into()])))
                        }
            
                        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: de::MapAccess<'de>, {
                            let mut x = None;
                            let mut y = None;
                            let mut z = None;
            
                            while let Some(key) = map.next_key::<&str>()? {
                                match key {
                                    "x" => {
                                        if x.is_some() { return Err(de::Error::duplicate_field("x")) }
                                        x = Some(map.next_value::<$ty>()?)
                                    }
            
                                    "y" => {
                                        if y.is_some() { return Err(de::Error::duplicate_field("y")) }
                                        y = Some(map.next_value::<$ty>()?)
                                    }

                                    "z" => {
                                        if z.is_some() { return Err(de::Error::duplicate_field("z")) }
                                        z = Some(map.next_value::<$ty>()?)
                                    }
            
                                    _ => {}
                                }
                            }
                            
                            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
                            let z = z.ok_or_else(|| de::Error::missing_field("z"))?;
                            Ok($target::new(concat([x.into(), y.into(), z.into()])))
                        }
                    }
            
                    deserializer.deserialize_struct(stringify!($target), &["x", "y", "z"], LocalVisitor)
                }
            }
        )*
    }
}

impl_vec2!(
    EucVec2f, f32,
    EucVec2d, f64
);

impl_vec3!(
    EucVec3f, f32,
    EucVec3d, f64
);

impl_vec4!(
    EucVec4f, f32,
    EucVec4d, f64
);

impl_mat2!(
    Mat2f, EucVec2f,
    Mat2d, EucVec2d
);

impl_mat3!(
    Mat3f, EucVec3f,
    Mat3d, EucVec3d
);

#[inline(always)]
fn concat <T: Copy, const N1: usize, const N2: usize> (a: [[T;N1];N2]) -> [T;N1*N2] {
    unsafe { *(&a as *const [[T;N1];N2] as *const [T;N1*N2]) }
}