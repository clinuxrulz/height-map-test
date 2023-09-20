use crate::{Quaternion, Vec3};

pub struct Transform3<T> {
    pub origin: Vec3<T>,
    pub orientation: Quaternion<T>,
}

impl<T> Transform3<T> {
    pub fn new(origin: Vec3<T>, orientation: Quaternion<T>) -> Transform3<T> {
        Transform3 {
            origin,
            orientation,
        }
    }
}
