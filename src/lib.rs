use wasm_bindgen::prelude::*;

mod aabb;
mod camera;
mod height_map;
mod vec2;
mod vec3;
mod max;
mod min;
mod one;
mod quaternion;
mod ray2;
mod sqrt;
mod transform3;
mod zero;

pub use aabb::Aabb;
pub use camera::Camera;
pub use height_map::HeightMap;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use max::Max;
pub use min::Min;
pub use one::One;
pub use quaternion::Quaternion;
pub use ray2::Ray2;
pub use sqrt::Sqrt;
pub use transform3::Transform3;
pub use zero::Zero;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    let height_map = HeightMap::new(256);
    let aabb = Aabb::new(-128.0, -128.0, -128.0, 128.0, 128.0, 128.0);
    let w = Vec3::new(-1.0, 1.0, 1.0).normalize();
    let up = Vec3::new(0.0, 1.0, 0.0);
    let u = up.cross(w).normalize();
    let screen_width = 320.0;
    let screen_height = 200.0;
    let fov_y: f64 = 60.0;
    let screen_dist = 0.5 * screen_height / fov_y.to_radians().tan();
    let camera = Camera {
        space: Transform3::new(
            Vec3::new(-500.0, 500.0, 500.0),
            Quaternion::from_wu(w, u),
        ),
        screen_width,
        screen_height,
        screen_dist
    };
    // TODO
}
