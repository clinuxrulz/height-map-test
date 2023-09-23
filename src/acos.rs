pub trait Acos {
    fn acos(self) -> Self;
}

impl Acos for f32 {
    fn acos(self) -> Self {
        f32::acos(self)
    }
}

impl Acos for f64 {
    fn acos(self) -> Self {
        f64::acos(self)
    }
}
