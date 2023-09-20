use crate::Sqrt;
use std::ops::{Add, Sub, Mul, Div};

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn dot(self, rhs: Vec3<T>) -> T
    where
        T: Add<Output=T> + Mul<Output=T>
    {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(self, rhs: Vec3<T>) -> Vec3<T>
    where
        T: Sub<Output=T> + Mul<Output=T>
    {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length_squared(&self) -> T
    where
        T: Add<Output=T> + Mul<Output=T> + Clone
    {
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone()
    }

    pub fn length(&self) -> T
    where
        T: Add<Output=T> + Mul<Output=T> + Clone + Sqrt<Output=T>
    {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Vec3<T>
    where
        T: Add<Output=T> + Mul<Output=T> + Div<Output=T> + Clone + Sqrt<Output=T>
    {
        let len = self.length();
        Vec3::new(
            self.x / len,
            self.y / len,
            self.z / len,
        )
    }
}

impl<T> Clone for Vec3<T> where T: Clone {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone(), z: self.z.clone(), }
    }
}

impl<T> Copy for Vec3<T> where T: Copy {}

impl<T> Add for Vec3<T> where T: Add<Output=T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vec3<T> where T: Sub<Output=T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T> where T: Mul<Output=T> + Clone {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs,
        }
    }
}
