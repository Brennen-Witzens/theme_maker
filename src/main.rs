mod algorithms;
mod utils;

use algorithms::median_cut::median_cut_recursive;
use image::GenericImageView;
use std::hash::Hash;

// Easier struct to work with than to work with the Image crates Rgba struct value of a u8 array
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
}

impl RGBColor {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
#[derive(Copy, Clone)]
enum RGB {
    Undefined,
    Red,
    Green,
    Blue,
}

fn main() -> Result<(), std::io::Error> {
    // Open the image
    // TODO:
    // This needs to change to have better error handling
    // This also needs to use command line arguments to pass in the image path
    let image = image::open(".\\peach-blossom.png").unwrap();
    let mut colors = Vec::<RGBColor>::new();

    // Iterate the pixels to get the colors and pixels
    for (_, _, color) in image.pixels() {
        colors.push(RGBColor::new(color.0[0], color.0[1], color.0[2]));
    }

    median_cut_recursive(&colors, 2);

    Ok(())
}
