use crate::Transform3;

pub struct Camera<T> {
    pub space: Transform3<T>,
    pub screen_width: T,
    pub screen_height: T,
    pub screen_dist: T,
}
