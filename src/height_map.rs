use rand::prelude::random;

pub struct HeightMap {
    size: u32,
    data: Vec<f64>,
    num_levels: u32,
}

impl HeightMap {
    pub fn new(size: u32) -> HeightMap {
        let data_len: usize = (size + (size + 2) / 3).try_into().unwrap();
        let mut data: Vec<f64> = Vec::with_capacity(data_len);
        for _i in 0..data_len {
            data.push(0.0);
        }
        let mut r = HeightMap { size, data, num_levels: 0, };
        r.num_levels = r.num_levels();
        r.init_data();
        r
    }

    fn init_data(&mut self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let h: f64 = random::<f64>() * 20.0 - 100.0;
                self.write(self.num_levels-1, x, y, h);
            }
        }
        for lvl in (0..self.num_levels-1).rev() {
            let lvl2 = lvl + 1;
            for y in 0..(self.size>>lvl) {
                for x in 0..(self.size>>lvl) {
                    let xx = x << 1;
                    let yy = y << 1;
                    let h1 = self.read(lvl2, xx, yy);
                    let h2 = self.read(lvl2, xx+1, yy);
                    let h3 = self.read(lvl2, xx+1, yy+1);
                    let h4 = self.read(lvl2, xx, yy+1);
                    let h = h1.max(h2).max(h3.max(h4));
                    self.write(lvl, x, y, h);
                }
            }
        }
    }

    pub fn num_levels(&self) -> u32 {
        let mut level: u32 = 0;
        let mut size: u32 = self.size;
        while size > 0 {
            size >>= 1;
            level += 1;
        }
        level
    }

    pub fn offset(&self, level: u32) -> u32 {
        let mut offset = 0;
        for _i in 0..level {
            offset *= 4;
            offset += 1;
        }
        offset
    }

    pub fn write(&mut self, level: u32, x: u32, y: u32, val: f64) {
        let offset = self.offset(level);
        let offset2 = offset + (y << level) + x;
        self.data[offset2 as usize] = val;
    }

    pub fn read(&mut self, level: u32, x: u32, y: u32) -> f64 {
        let offset = self.offset(level);
        let offset2 = offset + (y << level) + x;
        return self.data[offset2 as usize];
    }
}
