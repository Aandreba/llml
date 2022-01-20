import_derives!();

/// Polar coordinates
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "llml_serde", derive(Serialize, Deserialize))]
pub struct Polar<T> {
    pub radius: T,
    pub angle: T
}

impl<T> Polar<T> {
    pub fn new (radius: T, angle: T) -> Self {
        Self { radius, angle }
    }
}