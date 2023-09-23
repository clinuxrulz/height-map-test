use crate::Max;
use crate::Min;
use crate::Ray2;
use crate::Zero;
use std::ops::{Sub, Div};

pub struct Aabb<T> {
    pub min_x: T,
    pub min_y: T,
    pub min_z: T,
    pub max_x: T,
    pub max_y: T,
    pub max_z: T,
}

impl<T> Aabb<T> {
    pub fn new(min_x: T, min_y: T, min_z: T, max_x: T, max_y: T, max_z: T) -> Aabb<T> {
        Aabb { min_x, min_y, min_z, max_x, max_y, max_z, }
    }

    pub fn ray_insection_2pt5d(&self, ray: Ray2<T>) -> Option<(T,T)>
    where
        T: Sub<Output=T> + Div<Output=T> + Copy + Min + Max + PartialOrd + Zero
    {
        let t1 = (self.min_x - ray.origin.x) / ray.direction.x;
        let t2 = (self.max_x - ray.origin.x) / ray.direction.x;
        let t3 = (self.min_y - ray.origin.y) / ray.direction.y;
        let t4 = (self.max_y - ray.origin.y) / ray.direction.y;
        let t_min = t1.min(t2).max(t3.min(t4));
        let t_max = t1.max(t2).min(t3.max(t4));
        if t_min < T::zero() {
            return None;
        }
        if t_max < t_min {
            return None;
        }
        Some((t_min,t_max))
    }
}
