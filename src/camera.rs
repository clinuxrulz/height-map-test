use crate::{Ray2, Transform3, Vec2, Vec3};

pub struct Camera<T> {
    pub space: Transform3<T>,
    pub screen_width: T,
    pub screen_height: T,
    pub screen_dist: T,
}

impl<T> Camera<T> {
    pub fn screen_x_to_ray2(x: T) -> Option<Ray2<T>> {
        todo!();
    }

    pub fn project(pt: Vec3<T>) -> Vec2<T> {
        todo!();
    }
}
