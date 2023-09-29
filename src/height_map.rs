use rand::prelude::random;

use crate::{QuadTree, Ray2, Vec2};

pub struct HeightMap {
    num_levels: usize,
    quad_tree: QuadTree<f64>,
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
        };
        r.init_data();
        r
    }

    fn init_data(&mut self) {
        let size = 1 << (self.num_levels-1);
        for y in 0..size {
            for x in 0..size {
                //let h: f64 = random::<f64>() * 80.0 - 140.0;
                let h = ((x as f64) * 0.1).cos() * ((y as f64) * 0.1).sin() * 40.0 - 100.0;
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

    pub fn ray_xz_insection_2pt5d<Callback: FnMut(TimeHeight,bool)->bool>(&self, ray_xz: Ray2<f64>, mut callback: Callback) {
        self.ray_xz_insection_2pt5d_2(0, 0, 0, ray_xz, &mut callback)
    }

    fn ray_xz_insection_2pt5d_2<CALLBACK: FnMut(TimeHeight,bool)->bool>(&self, depth: usize, x0: usize, y0: usize, ray_xz: Ray2<f64>, callback: &mut CALLBACK) {
        let size: usize = 1 << (self.num_levels-1-depth);
        let size2 = (size as f64) * 6.0;//* 2.0;
        let t1 = (-0.5 * size2 - ray_xz.origin.x) / ray_xz.direction.x;
        let t2 = (0.5 * size2 - ray_xz.origin.x) / ray_xz.direction.x;
        let t3 = (-0.5 * size2 - ray_xz.origin.y) / ray_xz.direction.y;
        let t4 = (0.5 * size2 - ray_xz.origin.y) / ray_xz.direction.y;
        let t_min = t1.min(t2).max(t3.min(t4));
        let t_max = t1.max(t2).min(t3.max(t4));
        if t_min <= 0.0 {
            return;
        }
        if t_max < t_min {
            return;
        }
        if depth < self.num_levels-1 {
            let height = self.read(depth, x0 >> (self.num_levels-1-depth), y0 >> (self.num_levels-1-depth));
            if callback(TimeHeight { t: t_max, height }, true) {
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
            let height = self.read(depth, x0, y0);
            let _ = callback(TimeHeight { t: t_max, height, }, false);
        }
    }
}
