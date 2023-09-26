use rand::prelude::random;

use crate::{QuadTree, Ray2, Vec2};

pub struct HeightMap {
    num_levels: usize,
    quad_tree: QuadTree<f64>,
}

impl HeightMap {
    pub fn new(num_levels: usize) -> HeightMap {
        let mut r = HeightMap {
            num_levels,
            quad_tree: QuadTree::new(num_levels, 0.0f64),
        };
        r.init_data();
        r
    }

    fn init_data(&mut self) {
        let size = 1 << (self.num_levels-1);
        for y in 0..size {
            for x in 0..size {
                let h: f64 = random::<f64>() * 100.0;
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

    pub fn ray_xz_insection_2pt5d(&self, ray_xz: Ray2<f64>) -> Option<(f64,f64,f64)> {
        self.ray_xz_insection_2pt5d_2(0, 0, 0, ray_xz)
    }

    fn ray_xz_insection_2pt5d_2(&self, depth: usize, x0: usize, y0: usize, ray_xz: Ray2<f64>) -> Option<(f64,f64,f64)> {
        let size: usize = 1 << (self.num_levels-1-depth);
        let size2 = size as f64;
        let t1 = (-0.5 * size2 - ray_xz.origin.x) / ray_xz.direction.x;
        let t2 = (0.5 * size2 - ray_xz.origin.x) / ray_xz.direction.x;
        let t3 = (-0.5 * size2 - ray_xz.origin.y) / ray_xz.direction.y;
        let t4 = (0.5 * size2 - ray_xz.origin.y) / ray_xz.direction.y;
        let t_min = t1.min(t2).max(t3.min(t4));
        let t_max = t1.max(t2).min(t3.max(t4));
        if t_min <= 0.0 {
            return None;
        }
        if t_max < t_min {
            return None;
        }
        if depth < self.num_levels-1 {
            let half_size = size >> 1;
            let offsets: [([f64; 2],[usize; 2]); 4] = [
                ([-0.5 * size2, -0.5 * size2], [0, 0]),
                ([0.5 * size2, -0.5 * size2], [half_size, 0]),
                ([0.5 * size2, 0.5 * size2], [half_size, half_size]),
                ([-0.5 * size2, 0.5 * size2], [0, half_size]),
            ];
            let mut closest: Option<(f64,f64,f64)> = None;
            for (ray_offset, index_offset) in offsets {
                let ray_xz2 = Ray2::new(ray_xz.origin + Vec2::new(ray_offset[0], ray_offset[1]), ray_xz.direction);
                let r = self.ray_xz_insection_2pt5d_2(depth + 1, index_offset[0], index_offset[1], ray_xz2);
                if let Some(r2) = r {
                    if closest.is_none() || r2.0 < closest.unwrap().0 {
                        closest = Some(r2);
                    }
                }
            }
            closest
        } else {
            let t = 0.5 * (t_min + t_max);
            let pt = ray_xz.position_from_time(t);
            let x = x0 + (((pt.x + 0.5 * size2) as i32).max(0).min((size-1) as i32) as usize);
            let y = y0 + (((pt.y + 0.5 * size2) as i32).max(0).min((size-1) as i32) as usize);
            let height = self.read(depth, x, y);
            Some((t_min,t_max,height))
        }
    }
}
