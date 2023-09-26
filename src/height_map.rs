use rand::prelude::random;

use crate::QuadTree;

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
                let h: f64 = random::<f64>() * 20.0 - 100.0;
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

    pub fn read(&mut self, level: usize, x: usize, y: usize) -> f64 {
        *self.quad_tree.get_value(level, x, y)
    }
}
