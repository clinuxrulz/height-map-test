pub trait Max {
    fn max(self, b: Self) -> Self;
}

impl Max for f32 {
    fn max(self, b: Self) -> Self {
        f32::max(self, b)
    }
}

impl Max for f64 {
    fn max(self, b: Self) -> Self {
        f64::max(self, b)
    }
}
