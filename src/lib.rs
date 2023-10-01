use height_map::TimeHeight;
use wasm_bindgen::prelude::*;
use js_sys::Uint32Array;

mod aabb;
mod acos;
mod camera;
mod complexplanet;
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
pub use complexplanet::make_planet;
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
pub fn main(height_map: *const HeightMap, screen: &Uint32Array, angle: f64) {
    init_panic_hook();
    //
    let height_map = unsafe { &*height_map };
    //
    main2(
        height_map,
        |offset, colour| {
            screen.set_index(offset as u32, colour);
        },
        angle,
    );
}

#[wasm_bindgen]
pub fn create_height_map() -> *mut HeightMap {
    Box::into_raw(Box::new(HeightMap::new(8)))
}

#[wasm_bindgen]
pub fn free_height_map(height_map: *mut HeightMap) {
    let _ = unsafe { Box::from_raw(height_map) };
}

pub fn main2<WriteScreen: FnMut(usize,u32)>(height_map: &HeightMap, mut write_screen: WriteScreen, angle: f64) {
    //
    //let height_map = HeightMap::new(8);
    if false {
        for level in 0..8 {
            let width = 1 << level;
            let mut offset: usize = width;
            for y in 0..width {
                for x in 0..width {
                    let height = height_map.read(level, x, y);
                    let c = ((height as i32).abs() as u32) & 0xFF;
                    write_screen(offset + x, 0xFF808000 | c);
                }
                offset += 320;
            }
        }
        return;
    }
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
    let cam_pos = q.rotate(Vec3::new(1600.0, 0.0, 0.0));
    let w = cam_pos.normalize();
    let up = Vec3::new(0.0, 1.0, 0.0);
    let u = up.cross(w).normalize();
    let camera = Camera {
        space: Transform3::new(
            cam_pos + Vec3::new(0.0, 500.0, 0.0),
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
        let mut y_max = screen_height as i32;
        height_map.ray_xz_insection_2pt5d(
            ray_xz,
            |TimeHeight { t, height }, early_bail_test, color_op| {
                let pt = ray_xz.position_from_time(t);
                let y1 = camera.project_y(Vec3::new(pt.x, height, pt.y));
                let yi = (y1 as i32).max(0).min(199);
                if early_bail_test {
                    return yi > y_max;
                }
                if yi < y_max {
                    for y in yi..y_max {
                        let y = y as u32;
                        let offset = (y << 8) + (y << 6) + x;
                        let color2: u32;
                        if let Some(color) = color_op {
                            color2 = 0xFF000000 | ((color[2] as u32) << 16) | ((color[1] as u32) << 8) | (color[0] as u32);
                        } else {
                            let c = ((height as i32).abs() as u32) & 0xFF;
                            color2 = 0xFF808000 | c;
                        }
                        write_screen(offset as usize, color2);
                    }
                    y_max = yi;
                }
                return false;
            }
        );

        // TODO
    }
    // TODO    
}
