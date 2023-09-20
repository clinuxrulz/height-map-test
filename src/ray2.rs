use crate::Vec2;

use std::ops::{Add, Mul};

pub struct Ray2<T> {
    pub origin: Vec2<T>,
    pub direction: Vec2<T>,
}

impl<T> Ray2<T> {
    pub fn new(origin: Vec2<T>, direction: Vec2<T>) -> Ray2<T> {
        Ray2 {
            origin,
            direction,
        }
    }

    pub fn position_from_time(&self, t: T) -> Vec2<T>
    where
        T: Add<Output=T> + Mul<Output=T> + Clone + Copy,
    {
        return self.origin + self.direction * t;
    }
}

impl<T> Clone for Ray2<T> where T: Clone {
    fn clone(&self) -> Self {
        Self { origin: self.origin.clone(), direction: self.direction.clone() }
    }
}

impl<T> Copy for Ray2<T> where T: Copy {}
