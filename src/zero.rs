pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0f32
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0f64
    }
}
