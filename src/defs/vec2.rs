pub type EucVecf2 = EucVec2<f32>;
pub type EucVecd2 = EucVec2<f64>;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct EucVec2<T> {
    pub x: T,
    pub y: T
}

impl<T> EucVec2<T>  {
    pub fn new (x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl EucVecf2 {
    #[inline(always)]
    pub fn sum (self) -> f32 {
        self.x + self.y
    }

    #[inline(always)]
    pub fn norm2 (self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn norm (self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn unit (self) -> Self {
        self / self.norm()
    }
}