mod algorithms;
mod utils;

use algorithms::median_cut::{median_cut, quantize_values};
use clap::{Arg, Command};
use image::GenericImageView;
use std::{
    cmp::Ordering,
    collections::HashMap,
    path::{Path, PathBuf},
};

use rand::prelude::*;

// Easier struct to work with than to work with the Image crates Rgba struct value of a u8 array
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
}

impl RGBColor {
    fn build_color(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
#[derive(Copy, Clone, Debug)]
enum Rgb {
    Undefined,
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cube {
    name: String,
    color: Rgb,
    range: u8,
}

impl Cube {
    fn new(name: &str, color: Rgb, range: u8) -> Self {
        Self {
            name: name.to_string(),
            color,
            range,
        }
    }
}

const PALETTE_SIZE: i32 = 5;

fn main() -> Result<(), std::io::Error> {
    // Open image from command line argument
    let image_path = build_image_path();

    // Make sure we have a path that is something
    if let Some(path) = image_path {
        let colors = get_pixels_from_image(&path);

        let mut cut_images = HashMap::<String, Vec<RGBColor>>::new();
        let mut cubes = Vec::<Cube>::new();

        // We should have the base value here --> Original colors/image added first
        // Remove the entry later (after it's been cut)
        cut_images.insert("Original".to_string(), colors);

        for i in 0..PALETTE_SIZE {
            // need to go over the cubes to find largest range
            for (key, val) in cut_images.iter() {
                // Determine largest range of color in the values (cubes)
                let cube = determine_cube_cut_criteria(key, val);
                cubes.push(cube);
            }

            // Iterate over the cubes to determine which cube to use, IE which has the
            // largest range for a color
            let val = cubes.iter().max_by_key(|x| x.range).unwrap();

            let cube_to_cut = cut_images.get(&val.name).unwrap();
            let cube = median_cut(cube_to_cut, val.color, i);

            let _ = cut_images.remove_entry(&val.name);

            cut_images.extend(cube.into_iter());

            // Clear cubes at the end of the X
            cubes.clear();
        }

        for colors in cut_images.values() {
            quantize_values(colors);
        }
    }

    Ok(())
}

fn get_pixels_from_image(image_path: &Path) -> Vec<RGBColor> {
    let image = image::open(image_path);

    let mut colors = Vec::<RGBColor>::new();

    if let Ok(image_result) = image {
        for element in image_result.pixels() {
            colors.push(RGBColor::build_color(
                element.2 .0[0],
                element.2 .0[1],
                element.2 .0[2],
            ));
        }
    }
    colors
}

fn build_image_path() -> Option<PathBuf> {
    // Get the commands
    // TODO: Add palette size
    let matches = Command::new("myapp")
        .arg(Arg::new("image").short('i').long("image"))
        .get_matches();

    if let Some(image_path) = matches.get_one::<String>("image") {
        let path = PathBuf::from(image_path);
        if path.is_relative() {
            println!("Please use absolute path");
            return None;
        }
        Some(path)
    } else {
        println!("Nothing was passed in");
        None
    }
}

fn determine_cube_cut_criteria(cube_name: &str, cube: &[RGBColor]) -> Cube {
    assert!(
        !cube.is_empty(),
        "We want to confirm that the cube is not empty. We're always expecting values -- {}:{}",
        cube.len(),
        cube_name
    );

    // Iterate over the color cube to find the min and max values for each channel
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
        color_choice = Rgb::Red;
        range = r_range;
    } else if g_range > r_range && g_range > b_range {
        // Green is the largest range
        color_choice = Rgb::Green;
        range = g_range;
    } else if b_range > r_range && b_range > g_range {
        // blue is the largest
        color_choice = Rgb::Blue;
        range = b_range;
    } else {
        let mut rng = rand::rng();
        let choice: i32;
        // This should be when all of them are equal -> 255 or otherwise
        if r_range.cmp(&g_range) == Ordering::Equal {
            // This is where either Green or Red are equal
            choice = rng.random_range(0..2);
            color_choice = match choice {
                0 => Rgb::Red,
                1 => Rgb::Green,
                _ => Rgb::Undefined,
            };
            range = match color_choice {
                Rgb::Red => r_range,
                Rgb::Green => g_range,
                Rgb::Blue => b_range,
                Rgb::Undefined => 5, // error cause of sign
            };
        } else if r_range.cmp(&b_range) == Ordering::Equal {
            // This is where either Blue or Red are Equal
            // Pick one between the two at random
            choice = rng.random_range(0..2);
            color_choice = match choice {
                0 => Rgb::Red,
                1 => Rgb::Blue,
                _ => Rgb::Undefined,
            };
            range = match color_choice {
                Rgb::Red => r_range,
                Rgb::Green => g_range,
                Rgb::Blue => b_range,
                Rgb::Undefined => 5, // error cause of sign
            };
        } else if g_range.cmp(&b_range) == Ordering::Equal {
            // This is where Green or Blue are equal
            choice = rng.random_range(0..3);
            color_choice = match choice {
                0 => Rgb::Green,
                1 => Rgb::Blue,
                _ => Rgb::Undefined,
            };
            range = match color_choice {
                Rgb::Red => r_range,
                Rgb::Green => g_range,
                Rgb::Blue => b_range,
                Rgb::Undefined => 5, // error cause of sign
            };
        } else {
            // All are equally large pick one are random
            choice = rng.random_range(0..3);
            color_choice = match choice {
                0 => Rgb::Red,
                1 => Rgb::Green,
                2 => Rgb::Blue,
                _ => Rgb::Undefined,
            };
            range = match color_choice {
                Rgb::Red => r_range,
                Rgb::Green => g_range,
                Rgb::Blue => b_range,
                Rgb::Undefined => 5, // error cause of sign
            };
        }
    }

    Cube::new(cube_name, color_choice, range)
}
