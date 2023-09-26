pub struct QuadTree<T> {
    memory: Vec<T>,
    depth_locations: Vec<usize>,
}

impl<T> QuadTree<T> {
    pub fn new(num_levels: usize, default_value: T) -> QuadTree<T>
    where
        T: Copy
    {
        let mut location: usize = 0;
        let mut depth_locations = Vec::<usize>::new();
        for i in 0..num_levels {
            let map_size: usize = 1 << (i << 1);
            depth_locations.push(location);
            location += map_size;
        }
        let mut memory = Vec::with_capacity(location);
        for _i in 0..location {
            memory.push(default_value);
        }
        QuadTree {
            memory,
            depth_locations,
        }
    }

    pub fn get_value(&self, depth: usize, x: usize, y: usize) -> &T {
        let location = self.depth_locations[depth] + (y << depth) + x;
        &self.memory[location]
    }

    pub fn set_value(&mut self, depth: usize, x: usize, y: usize, value: T) {
        let location = self.depth_locations[depth] + (y << depth) + x;
        self.memory[location] = value;
    }
}
