use rand::prelude::random;
use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder, ColorGradient};

use crate::{QuadTree, Ray2, Vec2};

pub struct HeightMap {
    num_levels: usize,
    quad_tree: QuadTree<f64>,
    color_gradient_op: Option<ColorGradient>,
}

pub struct TimeHeight {
    pub t: f64,
    pub height: f64,
}

impl HeightMap {
    pub fn new(num_levels: usize) -> HeightMap {
        let mut r = HeightMap {
            num_levels,
            quad_tree: QuadTree::new(num_levels, 0.0f64),
            color_gradient_op: None,
        };
        r.init_data();
        r
    }

    fn init_data(&mut self) {
        let size = 1 << (self.num_levels-1);
        //let fbm = Fbm::<Perlin>::new(0);

        let (noise_map, color_gradient) = crate::make_planet();
        self.color_gradient_op = Some(color_gradient);

        /*
        let noise_map = PlaneMapBuilder::<_, 2>::new(&fbm)
                .set_size(size, size)
                .set_x_bounds(-2.0, 2.0)
                .set_y_bounds(-2.0, 2.0)
                .build();
        */
        
        for y in 0..size {
            for x in 0..size {
                //let h: f64 = random::<f64>() * 80.0 - 140.0;
                //let h = ((x as f64) * 0.1).cos() * ((y as f64) * 0.1).sin() * 40.0 - 100.0;
                let h = noise_map.get_value(x, y);
                self.quad_tree.set_value(self.num_levels-1, x, y, h);
            }
        }
        for lvl in (0..self.num_levels-1).rev() {
            let lvl2 = lvl + 1;
            let size = 1 << lvl;
            for y in 0..size {
                for x in 0..size {
                    let xx = x << 1;
                    let yy = y << 1;
                    let h1 = self.quad_tree.get_value(lvl2, xx, yy);
                    let h2 = self.quad_tree.get_value(lvl2, xx+1, yy);
                    let h3 = self.quad_tree.get_value(lvl2, xx+1, yy+1);
                    let h4 = self.quad_tree.get_value(lvl2, xx, yy+1);
                    let h = h1.max(*h2).max(h3.max(*h4));
                    self.quad_tree.set_value(lvl, x, y, h);
                }
            }
        }
    }

    pub fn write(&mut self, level: usize, x: usize, y: usize, val: f64) {
        self.quad_tree.set_value(level, x, y, val);
    }

    pub fn read(&self, level: usize, x: usize, y: usize) -> f64 {
        *self.quad_tree.get_value(level, x, y)
    }

    pub fn ray_xz_intersection_2pt5d<Callback: FnMut(TimeHeight,bool,Option<[u8;4]>)->bool>(&self, ray_xz: Ray2<f64>, mut callback: Callback) {
        const BLOCK_SIZE: f64 = 40.0;
        let size: usize = 1 << (self.num_levels-1);
        let size2 = (size as f64) * BLOCK_SIZE;
        let t1 = (-0.5 * size2 - ray_xz.origin.x) / ray_xz.direction.x;
        let t2 = (0.5 * size2 - ray_xz.origin.x) / ray_xz.direction.x;
        let t3 = (-0.5 * size2 - ray_xz.origin.y) / ray_xz.direction.y;
        let t4 = (0.5 * size2 - ray_xz.origin.y) / ray_xz.direction.y;
        let t_min = t1.min(t2).max(t3.min(t4));
        let t_max = t1.max(t2).min(t3.max(t4));
        fn scale_height(height: f64) -> f64 {
            if height < 0.0 {
                return 0.0;
            }
            return height * 1000.0;
        }
        if t_max < t_min {
            return;
        }
        let pos_xz: Vec2<f64>;
        if t_min >= 0.0 {
            pos_xz = ray_xz.position_from_time(t_min) + Vec2::new(0.5 * size2, 0.5 * size2);
        } else {
            pos_xz = ray_xz.origin + Vec2::new(0.5 * size2, 0.5 * size2);
        }
        let mut map_x = (pos_xz.x / BLOCK_SIZE) as i32;
        let mut map_z = (pos_xz.y / BLOCK_SIZE) as i32;
        let mut side_dist_x: f64;
        let mut side_dist_z: f64;
        let delta_dist_x = (BLOCK_SIZE / ray_xz.direction.x).abs();
        let delta_dist_z = (BLOCK_SIZE / ray_xz.direction.y).abs();
        let step_x: i32;
        let step_z: i32;
        
        if ray_xz.direction.x < 0.0 {
            step_x = -1;
            side_dist_x = (pos_xz.x / BLOCK_SIZE - (map_x as f64)) * delta_dist_x;
        } else {
            step_x = 1;
            side_dist_x = (((map_x + 1) as f64) - pos_xz.x / BLOCK_SIZE) * delta_dist_x;
        }
        if ray_xz.direction.y < 0.0 {
            step_z = -1;
            side_dist_z = (pos_xz.y / BLOCK_SIZE - (map_z as f64)) * delta_dist_z;
        } else {
            step_z = 1;
            side_dist_z = (((map_z + 1) as f64) - pos_xz.y / BLOCK_SIZE) * delta_dist_z;
        }
        loop {
            let dist: f64;
            if side_dist_x < side_dist_z {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                dist = t_min.max(0.0) + side_dist_x;
            } else {
                side_dist_z += delta_dist_z;
                map_z += step_z;
                dist = t_min.max(0.0) + side_dist_z;
            }
            if map_x < 0 && step_x < 0 {
                break;
            }
            if map_x >= size as i32 && step_x > 0 {
                break;
            }
            if map_z < 0 && step_z < 0 {
                break;
            }
            if map_z >= size as i32 && step_z > 0 {
                break;
            }
            if 0 <= map_x && map_x < size as i32 {
                if 0 <= map_z && map_z < size as i32 {
                    let height = self.read(self.num_levels-1, map_x as usize, map_z as usize);
                    let color: Option<[u8;4]>;
                    if let Some(color_gradient) = &self.color_gradient_op {
                        color = Some(color_gradient.get_color(height));
                    } else {
                        color = None;
                    }
                    let _ = callback(TimeHeight { t: dist, height: scale_height(height), }, false, color);
                }
            }
        }
    }

    pub fn ray_xz_insection_2pt5d<Callback: FnMut(TimeHeight,bool,Option<[u8;4]>)->bool>(&self, ray_xz: Ray2<f64>, mut callback: Callback) {
        self.ray_xz_insection_2pt5d_2(0, 0, 0, ray_xz, &mut callback)
    }

    fn ray_xz_insection_2pt5d_2<CALLBACK: FnMut(TimeHeight,bool,Option<[u8;4]>)->bool>(&self, depth: usize, x0: usize, y0: usize, ray_xz: Ray2<f64>, callback: &mut CALLBACK) {
        let size: usize = 1 << (self.num_levels-1-depth);
        let size2 = (size as f64) * 40.0;//* 2.0;
        let t1 = (-0.5 * size2 - ray_xz.origin.x) / ray_xz.direction.x;
        let t2 = (0.5 * size2 - ray_xz.origin.x) / ray_xz.direction.x;
        let t3 = (-0.5 * size2 - ray_xz.origin.y) / ray_xz.direction.y;
        let t4 = (0.5 * size2 - ray_xz.origin.y) / ray_xz.direction.y;
        let t_min = t1.min(t2).max(t3.min(t4));
        let t_max = t1.max(t2).min(t3.max(t4));
        fn scale_height(height: f64) -> f64 {
            if height < 0.0 {
                return 0.0;
            }
            return height * 1000.0;
        }
        if t_max < t_min {
            return;
        }
        if depth < self.num_levels-1 {
            let height = self.read(depth, x0 >> (self.num_levels-1-depth), y0 >> (self.num_levels-1-depth));
            if callback(TimeHeight { t: t_max, height: scale_height(height) }, true, None) {
                return;
            }
            let half_size = size >> 1;
            let mut offsets: [([f64; 2],[usize; 2]); 4] = [
                ([0.25 * size2, 0.25 * size2], [x0, y0]),
                ([-0.25 * size2, 0.25 * size2], [x0+half_size, y0+0]),
                ([-0.25 * size2, -0.25 * size2], [x0+half_size, y0+half_size]),
                ([0.25 * size2, -0.25 * size2], [x0, y0+half_size]),
            ];
            offsets.sort_by(|a, b| {
                let x = -a.0[0] * ray_xz.direction.x - a.0[1] * ray_xz.direction.y;
                let y = -b.0[0] * ray_xz.direction.x - b.0[1] * ray_xz.direction.y;
                return x.partial_cmp(&y).unwrap();
            });
            for (ray_offset, index_offset) in offsets {
                let ray_xz2 = Ray2::new(ray_xz.origin + Vec2::new(ray_offset[0], ray_offset[1]), ray_xz.direction);
                self.ray_xz_insection_2pt5d_2(depth + 1, index_offset[0], index_offset[1], ray_xz2, callback);
            }
        } else {
            if t_min <= 0.0 {
                return;
            }
            let height = self.read(depth, x0, y0);
            let color: Option<[u8;4]>;
            if let Some(color_gradient) = &self.color_gradient_op {
                color = Some(color_gradient.get_color(height));
            } else {
                color = None;
            }
            let _ = callback(TimeHeight { t: t_max, height: scale_height(height), }, false, color);
        }
    }
}
