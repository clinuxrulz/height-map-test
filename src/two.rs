pub trait Two {
    fn two() -> Self;
}

impl Two for f32 {
    fn two() -> Self {
        2.0f32
    }
}

impl Two for f64 {
    fn two() -> Self {
        2.0f64
    }
}
