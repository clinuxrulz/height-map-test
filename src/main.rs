use height_map_test::{HeightMap, main2};

pub fn main() {
    let height_map = HeightMap::new(8);
    let mut screen: [u32; 64000] = [0; 64000];
    main2(
        &height_map,
        |offset, colour| {
            screen[offset] = colour;
        },
        0.0,
    );
}
