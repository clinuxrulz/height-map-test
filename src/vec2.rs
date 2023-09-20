use crate::Sqrt;
use std::ops::{Add, Sub, Mul};

pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x,
            y,
        }
    }

    pub fn dot(self, rhs: Vec2<T>) -> T
    where
        T: Add<Output=T> + Mul<Output=T>
    {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn length_squared(&self) -> <<T as Mul>::Output as Add>::Output
    where
        T: Add<Output=T> + Mul<Output=T> + Clone
    {
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone()
    }

    pub fn length(&self) -> <<<T as Mul>::Output as Add>::Output as Sqrt>::Output
    where
        T: Add<Output=T> + Mul<Output=T> + Clone + Sqrt<Output=T>
    {
        self.length_squared().sqrt()
    }
}

impl<T> Clone for Vec2<T> where T: Clone {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}

impl<T> Copy for Vec2<T> where T: Copy {}

impl<T> Add for Vec2<T> where T: Add<Output=T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T> where T: Sub<Output=T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T> where T: Mul<Output=T> + Clone {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}
