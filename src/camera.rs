use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::{Ray2, Sqrt, Transform3, Vec2, Vec3, Zero, One};

pub struct Camera<T> {
    pub space: Transform3<T>,
    pub screen_width: T,
    pub screen_height: T,
    pub screen_dist: T,
}

impl<T: Copy> Camera<T> {
    pub fn screen_x_to_ray2(&self, x: T) -> Option<Ray2<T>>
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + Zero + One + Sqrt<Output=T>
    {
        let half = T::one() / (T::one() + T::one());
        let rd = Vec3::new(x - half * self.screen_width, T::zero(), -self.screen_dist);
        let rd2 = self.space.vector_from_space(rd);
        Some(
            Ray2::new(
                Vec2::new(self.space.origin.x, self.space.origin.z),
                Vec2::new(rd2.x, rd2.z).normalize(),
            )
        )
    }

    pub fn project_y(&self, pt: Vec3<T>) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + Zero + One
    {
        let pt2 = self.space.point_to_space(pt);
        let y = pt2.y * self.screen_dist / -pt2.z;
        let half = T::one() / (T::one() + T::one());
        return half * self.screen_height - y;
    }

    pub fn project(&self, pt: Vec3<T>) -> Vec2<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + Zero + One
    {
        let pt2 = self.space.point_to_space(pt);
        let pt3 = Vec2::new(
            pt2.x * self.screen_dist / -pt2.z,
            pt2.y * self.screen_dist / -pt2.z,
        );
        let half = T::one() / (T::one() + T::one());
        return Vec2::new(
            half * self.screen_width + pt3.x,
            half * self.screen_height - pt3.y,
        );
    }
}
