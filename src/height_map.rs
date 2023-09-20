use rand::prelude::random;

pub struct HeightMap {
    size: u32,
    data: Vec<f64>,
}

impl HeightMap {
    pub fn new(size: u32) -> HeightMap {
        let data_len: usize = (size + (size + 2) / 3).try_into().unwrap();
        let mut data: Vec<f64> = Vec::with_capacity(data_len);
        for _i in 0..data_len {
            data.push(0.0);
        }
        let levels: usize;
        {
            let mut levels2: usize = 0;
            let mut size2 = size;
            while size2 != 0 {
                size2 >>= 1;
                levels2 += 1;
            }
            levels = levels2;
        }
        {
            let mut idx = 1 << (levels - 1) - 1;
            let size2 = 1 << (levels - 1);
            for _y in 0..size2 {
                for _x in 0..size2 {
                    data[idx] = random();
                    idx += 1;
                }
            }
        }
        for level in (0..levels-1).rev() {
            let mut idx = 1 << (levels - 1) - 1;
            let size2 = 1 << level;
            for _y in 0..size2 {
                for _x in 0..size2 {
                    /*
                    Level 0:
                    0

                    Level 1:
                    1 2
                    3 4

                    Level 2:
                    5   6  7  8
                    9  10 11 12
                    13 14 15 16
                    17 18 19 20

                    Level 3:
                    21 22 23 24
                     */
                    let idx_11 = (idx << 2) + 1;
                    let idx_12 = (idx << 2) + 2;
                    let idx_21 = (idx << 2) + 3;
                    let idx_22 = (idx << 2) + 4;
                    idx += 1;
                }
            }
        }
        return HeightMap { size, data, };
    }
}
