use height_map::TimeHeight;
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
    main2(
        |offset, colour| {
            screen.set_index(offset as u32, colour);
        },
        angle,
    );
}

pub fn main2<WriteScreen: FnMut(usize,u32)>(mut write_screen: WriteScreen, angle: f64) {
    //
    let height_map = HeightMap::new(6);
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
        write_screen(i, 0xFF000000);
    }
    for x in 0..screen_width as u32 {
        let ray_xz = camera.screen_x_to_ray_xz(x as f64);
        if ray_xz.is_none() {
            continue;
        }
        let ray_xz = ray_xz.unwrap();
        let mut y_max = (screen_height - 1.0) as i32;
        height_map.ray_xz_insection_2pt5d(ray_xz, |TimeHeight { t, height }| {
            let pt = ray_xz.position_from_time(t);
            let y1 = camera.project_y(Vec3::new(pt.x, height, pt.y));
            let yi = (y1 as i32).max(0).min(199);
            if yi < y_max {
                for y in yi..y_max {
                    let y = y as u32;
                    let offset = (y << 8) + (y << 6) + x;
                    let c = ((height as i32).abs() as u32) & 0xFF;
                    write_screen(offset as usize, 0xFF808000 | c);
                }
                y_max = yi;
            }
        });

        // TODO
    }
    // TODO    
}
