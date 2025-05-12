mod algorithms;
mod utils;

use algorithms::median_cut::{self, median_cut, median_cut_recursive, quantize_values};
use image::GenericImageView;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, ops::Deref};

use rand::prelude::*;

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
#[derive(Copy, Clone, Debug)]
enum RGB {
    Undefined,
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cube {
    name: String,
    color: RGB,
    range: u8,
}

impl Cube {
    fn new(name: &str, color: RGB, range: u8) -> Self {
        Self {
            name: name.to_string(),
            color,
            range,
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
        colors.push(RGBColor::new(color.0[0], color.0[1], color.0[2]));
    }

    let mut cut_images = HashMap::<String, Vec<RGBColor>>::new();
    let mut cubes = Vec::<Cube>::new();
    for i in 0..15 {
        if i == 0 {
            cut_images.extend(median_cut(&colors, i).into_iter());
        } else {
            // need to go over the cubes to find largest range
            for (key, val) in cut_images.iter() {
                // Determine largest range of color in the values (cubes)
                let cube = determine_cube_cut_criteria(&key, &val);
                cubes.push(cube);
            }

            // Iterate over the cubes to determine which cube to use, IE which has the
            // largest range for a color
            let val = cubes.iter().max_by_key(|x| x.range).unwrap();

            let cube_to_cut = cut_images.get(&val.name).unwrap();
            let cub = median_cut(cube_to_cut, i);

            let _ = cut_images.remove_entry(&val.name);

            cut_images.extend(cub.into_iter());

            // Clear cubes at the end of the X
            cubes.clear();
        }
    }

    for colors in cut_images.values() {
        quantize_values(colors);
    }

    //median_cut_recursive(&colors, 2);

    Ok(())
}

fn determine_cube_cut_criteria(cube_name: &str, cube: &Vec<RGBColor>) -> Cube {
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
    let range;
    // Determine which range is the largest
    // NOTE: is there a better way to do this?
    if r_range > g_range && r_range > b_range {
        // Red is the largest range
        color_choice = RGB::Red;
        range = r_range;
    } else if g_range > r_range && g_range > b_range {
        // Green is the largest range
        color_choice = RGB::Green;
        range = g_range;
    } else if b_range > r_range && b_range > g_range {
        // blue is the largest
        color_choice = RGB::Blue;
        range = b_range;
    } else {
        let mut rng = rand::rng();
        // This should be when all of them are equal -> 255 or otherwise
        if r_range.cmp(&g_range) == Ordering::Equal {
            // This is where either Green or Red are equal
            let choice = rng.random_range(0..2);
            color_choice = match choice {
                0 => RGB::Red,
                1 => RGB::Green,
                _ => RGB::Undefined,
            };
            range = match color_choice {
                RGB::Red => r_range,
                RGB::Green => g_range,
                RGB::Blue => b_range,
                RGB::Undefined => 5, // error cause of sign
            };
        } else if r_range.cmp(&b_range) == Ordering::Equal {
            // This is where either Blue or Red are Equal
            // Pick one between the two at random
            let choice = rng.random_range(0..2);
            color_choice = match choice {
                0 => RGB::Red,
                1 => RGB::Blue,
                _ => RGB::Undefined,
            };
            range = match color_choice {
                RGB::Red => r_range,
                RGB::Green => g_range,
                RGB::Blue => b_range,
                RGB::Undefined => 5, // error cause of sign
            };
        } else if g_range.cmp(&b_range) == Ordering::Equal {
            // This is where Green or Blue are equal
            let choice = rng.random_range(0..3);
            color_choice = match choice {
                0 => RGB::Green,
                1 => RGB::Blue,
                _ => RGB::Undefined,
            };
            range = match color_choice {
                RGB::Red => r_range,
                RGB::Green => g_range,
                RGB::Blue => b_range,
                RGB::Undefined => 5, // error cause of sign
            };
        } else {
            // All are equally large pick one are random
            let choice = rng.random_range(0..3);
            color_choice = match choice {
                0 => RGB::Red,
                1 => RGB::Green,
                2 => RGB::Blue,
                _ => RGB::Undefined,
            };
            range = match color_choice {
                RGB::Red => r_range,
                RGB::Green => g_range,
                RGB::Blue => b_range,
                RGB::Undefined => 5, // error cause of sign
            };
        }
    }

    let found_cube = Cube::new(cube_name, color_choice, range);
    return found_cube;
}
