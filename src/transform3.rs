use std::ops::{Add, Sub, Mul, Neg};

use crate::{Quaternion, Vec3, Zero};

pub struct Transform3<T> {
    pub origin: Vec3<T>,
    pub orientation: Quaternion<T>,
}

impl<T: Copy> Transform3<T> {
    pub fn new(origin: Vec3<T>, orientation: Quaternion<T>) -> Transform3<T> {
        Transform3 {
            origin,
            orientation,
        }
    }

    pub fn point_from_space(&self, pt: Vec3<T>) -> Vec3<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Neg<Output=T> + Zero
    {
        self.vector_from_space(pt) + self.origin
    }

    pub fn point_to_space(&self, pt: Vec3<T>) -> Vec3<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Neg<Output=T> + Zero
    {
        self.vector_to_space(pt - self.origin)
    }

    pub fn vector_from_space(&self, v: Vec3<T>) -> Vec3<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Neg<Output=T> + Zero
    {
        self.orientation.rotate(v)
    }

    pub fn vector_to_space(&self, v: Vec3<T>) -> Vec3<T>
    where
        T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Neg<Output=T> + Zero
    {
        self.orientation.conjugate().rotate(v)
    }
}
