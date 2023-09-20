pub trait Min {
    fn min(self, b: Self) -> Self;
}

impl Min for f32 {
    fn min(self, b: Self) -> Self {
        f32::min(self, b)
    }
}

impl Min for f64 {
    fn min(self, b: Self) -> Self {
        f64::min(self, b)
    }
}
