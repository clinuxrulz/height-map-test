use height_map_test::main2;

pub fn main() {
    let mut screen: [u32; 64000] = [0; 64000];
    main2(
        |offset, colour| {
            screen[offset] = colour;
        },
        0.0,
    );
}