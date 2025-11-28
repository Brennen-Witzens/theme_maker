use std::collections::HashMap;

use crate::{RGBColor, Rgb};

pub fn median_cut(
    color_cube: &[RGBColor],
    color_to_cut: Rgb,
    iter: i32,
) -> HashMap<String, Vec<RGBColor>> {
    let mut cube: Vec<_> = color_cube.to_vec();
    // Sort cube by color values
    let median: u8;
    let median_index = cube.len() / 2;
    let mut above_pixels = Vec::<RGBColor>::new();
    let mut below_pixels = Vec::<RGBColor>::new();
    let mut split_cubes = HashMap::<String, Vec<RGBColor>>::new();

    match color_to_cut {
        Rgb::Red => {
            cube.sort_by(|x, y| x.r.cmp(&y.r));
            if cube.len().is_multiple_of(2) {
                median =
                    ((cube[median_index - 1].r as u16 + cube[median_index].r as u16) / 2) as u8;
            } else {
                median = cube[median_index].r;
            }

            // Now we split the values into above and below buckets
            for item in &cube {
                if item.r >= median {
                    above_pixels.push(item.to_owned());
                } else {
                    below_pixels.push(item.to_owned());
                }
            }
        }
        Rgb::Green => {
            cube.sort_by(|x, y| x.g.cmp(&y.g));
            if cube.len().is_multiple_of(2) {
                median =
                    ((cube[median_index - 1].g as u16 + cube[median_index].g as u16) / 2) as u8;
            } else {
                median = cube[median_index].g;
            }

            // Now we split the values into above and below buckets
            for item in &cube {
                if item.g >= median {
                    above_pixels.push(item.to_owned());
                } else {
                    below_pixels.push(item.to_owned());
                }
            }
        }
        Rgb::Blue => {
            cube.sort_by(|x, y| x.b.cmp(&y.b));
            if cube.len().is_multiple_of(2) {
                median =
                    ((cube[median_index - 1].b as u16 + cube[median_index].b as u16) / 2) as u8;
            } else {
                median = cube[median_index].b;
            }

            // Now we split the values into above and below buckets
            for item in &cube {
                if item.b >= median {
                    above_pixels.push(item.to_owned());
                } else {
                    below_pixels.push(item.to_owned());
                }
            }
        }
        Rgb::Undefined => {}
    };

    let above = format!("cube-upper{}", iter);
    let below = format!("cube-lower{}", iter);
    split_cubes.insert(above, above_pixels);
    split_cubes.insert(below, below_pixels);
    split_cubes
}

pub fn quantize_values(colors: &[RGBColor]) {
    assert!(
        !colors.is_empty(),
        "We're always expecting some amount of values in here: {}",
        colors.len()
    );

    let mut r_sum: u32 = 0;
    let mut g_sum: u32 = 0;
    let mut b_sum: u32 = 0;
    for color in colors.iter() {
        r_sum += color.r as u32;
        g_sum += color.g as u32;
        b_sum += color.b as u32;
    }

    let r_avg = r_sum / colors.len() as u32;
    let g_avg = g_sum / colors.len() as u32;
    let b_avg = b_sum / colors.len() as u32;

    print!("Avg Color is: {r_avg} - {g_avg} - {b_avg} ->");
    println!("\u{001b}[48;2;{};{};{}m    \u{001b}[m", r_avg, g_avg, b_avg);
}
