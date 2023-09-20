pub trait One {
    fn one() -> Self;
}

impl One for f32 {
    fn one() -> Self {
        1.0f32
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0f64
    }
}
