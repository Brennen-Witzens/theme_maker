use rand::prelude::*;
use std::cmp::Ordering;

use base64::{engine::general_purpose, prelude::*};
use image::GenericImageView;

// Easier struct to work with than to work with the Image crates Rgba struct value of a u8 array
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
    x: u32,
    y: u32,
}

impl RGBColor {
    fn new(r: u8, g: u8, b: u8, x: u32, y: u32) -> Self {
        Self { r, g, b, x, y }
    }

    fn new_without_location(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            ..Default::default()
        }
    }
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
        colors.push(RGBColor::new_without_location(
            color.0[0], color.0[1], color.0[2],
        ));
    }

    // Image has been converted over to just it's pixels (it is probably not sorted)
    let largest_color_range = find_ranges_for_cube(&colors);
    println!("Largest color range: {largest_color_range}");
    Ok(())
}

// We can think of the RGB (colors) as a cube on a 3 dimensional plane.
// We want to find the ranges for each channel (red, green, blue)
// 255 the max range. If multiple values are 255, pick one randomly.
// Return the color with the largest range
fn find_ranges_for_cube(cube: &Vec<RGBColor>) -> &str {
    // Iterate over the color cube to find the min and max values for each channel

    // NOTE: Testing for now, will get better handling of unwraps
    let r_max = cube.iter().max_by(|x, y| x.r.cmp(&y.r)).unwrap();
    let r_min = cube.iter().min_by(|x, y| x.r.cmp(&y.r)).unwrap();
    let r_range = r_max.r - r_min.r;

    let g_max = cube.iter().max_by(|x, y| x.g.cmp(&y.g)).unwrap();
    let g_min = cube.iter().min_by(|x, y| x.g.cmp(&y.g)).unwrap();
    let g_range = g_max.g - g_min.g;

    let b_max = cube.iter().max_by(|x, y| x.b.cmp(&y.b)).unwrap();
    let b_min = cube.iter().min_by(|x, y| x.b.cmp(&y.b)).unwrap();
    let b_range = b_max.b - b_min.b;

    let color_choice;
    // Determine which range is the largest
    // NOTE: is there a better way to do this?
    if r_range > g_range && r_range > b_range {
        // Red is the largest range
        color_choice = "Red";
    } else if g_range > r_range && g_range > b_range {
        // Green is the largest range
        color_choice = "Green";
    } else if b_range > r_range && b_range > g_range {
        // blue is the largest
        color_choice = "Blue";
    } else {
        let mut rng = rand::rng();
        // This should be when all of them are equal -> 255 or otherwise
        if r_range.cmp(&g_range) == Ordering::Equal {
            // This is where either Green or Red are equal
            let choice = rng.random_range(0..2);
            color_choice = match choice {
                0 => "Red",
                1 => "Green",
                _ => "",
            };
        } else if r_range.cmp(&b_range) == Ordering::Equal {
            // This is where either Blue or Red are Equal
            // Pick one between the two at random
            let choice = rng.random_range(0..2);
            color_choice = match choice {
                0 => "Red",
                1 => "Blue",
                _ => "",
            };
        } else if g_range.cmp(&b_range) == Ordering::Equal {
            // This is where Green or Blue are equal
            let choice = rng.random_range(0..3);
            color_choice = match choice {
                0 => "Green",
                1 => "Blue",
                _ => "",
            };
        } else {
            // All are equally large pick one are random
            let choice = rng.random_range(0..3);
            color_choice = match choice {
                0 => "Red",
                1 => "Green",
                2 => "Blue",
                _ => "",
            };
        }
    }
    color_choice
}

