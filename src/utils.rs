use rand::prelude::*;
use std::cmp::Ordering;

use crate::{RGBColor, RGB};
// We can think of the RGB (colors) as a cube on a 3 dimensional plane.
// We want to find the ranges for each channel (red, green, blue)
// 255 the max range. If multiple values are 255, pick one randomly.
// Return the color with the largest range
pub fn find_ranges_for_cube(cube: &Vec<RGBColor>) -> RGB {
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
        color_choice = RGB::Red;
    } else if g_range > r_range && g_range > b_range {
        // Green is the largest range
        color_choice = RGB::Green;
    } else if b_range > r_range && b_range > g_range {
        // blue is the largest
        color_choice = RGB::Blue;
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
        } else if r_range.cmp(&b_range) == Ordering::Equal {
            // This is where either Blue or Red are Equal
            // Pick one between the two at random
            let choice = rng.random_range(0..2);
            color_choice = match choice {
                0 => RGB::Red,
                1 => RGB::Blue,
                _ => RGB::Undefined,
            };
        } else if g_range.cmp(&b_range) == Ordering::Equal {
            // This is where Green or Blue are equal
            let choice = rng.random_range(0..3);
            color_choice = match choice {
                0 => RGB::Green,
                1 => RGB::Blue,
                _ => RGB::Undefined,
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
        }
    }
    color_choice
}
