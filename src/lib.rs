use wasm_bindgen::prelude::*;
use js_sys::Uint32Array;

mod aabb;
mod acos;
mod camera;
mod height_map;
mod vec2;
mod vec3;
mod max;
mod min;
mod one;
mod quad_tree;
mod quaternion;
mod ray2;
mod sin;
mod sqrt;
mod transform3;
mod zero;

pub use aabb::Aabb;
pub use acos::Acos;
pub use camera::Camera;
pub use height_map::HeightMap;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use max::Max;
pub use min::Min;
pub use one::One;
pub use quad_tree::QuadTree;
pub use quaternion::Quaternion;
pub use ray2::Ray2;
pub use sin::Sin;
pub use sqrt::Sqrt;
pub use transform3::Transform3;
pub use zero::Zero;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

use console_error_panic_hook;
use std::panic;

fn init_panic_hook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn main(screen: &Uint32Array, angle: f64) {
    init_panic_hook();
    //
    let height_map = HeightMap::new(8);
    //let aabb = Aabb::new(-128.0, -100.0, -128.0, 128.0, -80.0, 128.0);
    let screen_width = 320.0;
    let screen_height = 200.0;
    let fov_y: f64 = 45.0;
    let screen_dist = 0.5 * screen_height / (0.5 * fov_y).to_radians().tan();
    let angle2 = angle.to_radians();
    let q = Quaternion {
        w: (0.5 * angle2).cos(),
        x: 0.0,
        y: (0.5 * angle2).sin(),
        z: 0.0,
    };
    let cam_pos = q.rotate(Vec3::new(400.0, 0.0, 400.0));
    let w = cam_pos.normalize();
    let up = Vec3::new(0.0, 1.0, 0.0);
    let u = up.cross(w).normalize();
    let camera = Camera {
        space: Transform3::new(
            cam_pos,
            Quaternion::from_wu(w, u),
        ),
        screen_width,
        screen_height,
        screen_dist
    };
    for i in 0..64000 {
        screen.set_index(i, 0xFF000000);
    }
    for x in 0..screen_width as u32 {
        let ray_xz = camera.screen_x_to_ray_xz(x as f64);
        if ray_xz.is_none() {
            continue;
        }
        let mut ray_xz = ray_xz.unwrap();
        /*
        let t = aabb.ray_xz_insection_2pt5d(ray_xz);
        if t.is_none() {
            continue;
        }
        let t = t.unwrap();
        let (t_min, t_max) = t;
        let pt = ray_xz.position_from_time(t_min);
        let y1 = camera.project_y(Vec3::new(pt.x, aabb.min_y, pt.y));
        let y2 = camera.project_y(Vec3::new(pt.x, aabb.max_y, pt.y));
        let y1i = y1 as i32;
        let y2i = y2 as i32;
        let y_min = y1i.min(y2i).max(0).min(199);
        let y_max = y1i.max(y2i).max(0).min(199);
        for y in y_min..y_max+1 {
            let y = y as u32;
            let offset = (y << 8) + (y << 6) + x;
            screen.set_index(offset, 0xFF808080);
        }
        let pt = ray_xz.position_from_time(t_max);
        let y1 = camera.project_y(Vec3::new(pt.x, aabb.min_y, pt.y));
        let y2 = camera.project_y(Vec3::new(pt.x, aabb.max_y, pt.y));
        let y1i = y1 as i32;
        let y2i = y2 as i32;
        let y_min2 = y1i.min(y2i).max(0).min(199);
        for y in y_min2..y_min {
            let y = y as u32;
            let offset = (y << 8) + (y << 6) + x;
            screen.set_index(offset, 0xFF808000);
        }
        */
        let mut y_max = screen_height - 1.0;
        let mut first = true;
        loop {
            let t = height_map.ray_xz_insection_2pt5d(ray_xz);
            if t.is_none() {
                break;
            }
            let t = t.unwrap();
            let (t_min, t_max, height) = t;
            let pt = ray_xz.position_from_time(t_min);
            let y1 = camera.project_y(Vec3::new(pt.x, height, pt.y));
            if first {
                let y_max_max = camera.project_y(Vec3::new(pt.x, -100.0, pt.y));
                if y_max > y_max_max {
                    y_max = y_max_max;
                }
                first = false;
            }
            if y1 > y_max {
                ray_xz = Ray2::new(ray_xz.position_from_time(t_max - 0.001), ray_xz.direction);
                continue;
            }
            let c = ((height as i32).abs() as u32) & 0xFF;
            let to_y = (y_max as i32).max(0).min(199);
            let from_y = (y1 as i32).max(0).min(199);
            y_max = y1;
            for y in from_y..to_y+1 {
                let y = y as u32;
                let offset = (y << 8) + (y << 6) + x;
                screen.set_index(offset, 0xFF008000 | (c << 16));
            }
            let pt = ray_xz.position_from_time(t_max);
            let y1 = camera.project_y(Vec3::new(pt.x, height, pt.y));
            if y1 > y_max {
                ray_xz = Ray2::new(ray_xz.position_from_time(t_max - 0.001), ray_xz.direction);
                continue;
            }
            let to_y = (y_max as i32).max(0).min(199);
            let from_y = (y1 as i32).max(0).min(199);
            y_max = y1;
            for y in from_y..to_y+1 {
                let y = y as u32;
                let offset = (y << 8) + (y << 6) + x;
                screen.set_index(offset, 0xFF808000);
            }
        }

        // TODO
    }
    // TODO
}
