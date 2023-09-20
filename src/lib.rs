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
mod two;
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
pub use two::Two;
pub use zero::Zero;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    let height_map = HeightMap::new(256);
    // TODO
}
