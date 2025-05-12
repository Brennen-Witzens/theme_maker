use std::collections::HashMap;

use crate::{utils::find_ranges_for_cube, RGBColor, RGB};

// After having found the ranges, we want to sort and cut the color buckets along that color
// channel.
// Ex: RGB(12, 6, 19) and RGB(5, 23, 56) sorting along the blue would be 19 < 56
// After the values have been sorted, we find the median value and move the upper half into a new
// bucket. (Thus resulting in 2 buckets from a single cut - Above and Below values)
// This can continue, to further subdivide the pixels. Find the bucket with the largest range, cut
// that bucket along that color range, etc. After the number of buckets equals the desired number,
// average the pixels (RGB values) in the buckets to get the color palette
// TODO:
// 1. Does this or should this be recursive?
// 2. Two variants should be there, 1 recursive and 1 not i think works best
pub fn median_cut_recursive(color_cube: &Vec<RGBColor>, median_cut_round: i32) {
    if median_cut_round == 0 {
        quantize_values(color_cube);
        return;
    }

    let mut cube: Vec<_> = color_cube.iter().cloned().collect();
    // Sort cube by color values
    let median: u8;
    let median_index = cube.len() / 2;
    let mut above_pixels = Vec::<RGBColor>::new();
    let mut below_pixels = Vec::<RGBColor>::new();
    let color_to_cut = find_ranges_for_cube(&cube);
    match color_to_cut {
        RGB::Red => {
            cube.sort_by(|x, y| x.r.cmp(&y.r));
            if cube.len() % 2 == 0 {
                median =
                    ((cube[median_index - 1].r as u16 + cube[median_index].r as u16) / 2) as u8;
            } else {
                median = cube[median_index].r;
            }

            // Now we split the values into above and below buckets
            for i in 0..cube.len() {
                if cube[i].r >= median {
                    above_pixels.push(cube[i]);
                } else {
                    below_pixels.push(cube[i]);
                }
            }
        }
        RGB::Green => {
            cube.sort_by(|x, y| x.g.cmp(&y.g));
            if cube.len() % 2 == 0 {
                median =
                    ((cube[median_index - 1].g as u16 + cube[median_index].g as u16) / 2) as u8;
            } else {
                median = cube[median_index].g;
            }

            // Now we split the values into above and below buckets
            for i in 0..cube.len() {
                if cube[i].g >= median {
                    above_pixels.push(cube[i]);
                } else {
                    below_pixels.push(cube[i]);
                }
            }
        }
        RGB::Blue => {
            cube.sort_by(|x, y| x.b.cmp(&y.b));
            if cube.len() % 2 == 0 {
                median =
                    ((cube[median_index - 1].b as u16 + cube[median_index].b as u16) / 2) as u8;
            } else {
                median = cube[median_index].b;
            }

            // Now we split the values into above and below buckets
            for i in 0..cube.len() {
                if cube[i].b >= median {
                    above_pixels.push(cube[i]);
                } else {
                    below_pixels.push(cube[i]);
                }
            }
        }
        RGB::Undefined => {}
    };

    median_cut_recursive(&above_pixels, median_cut_round - 1);
    median_cut_recursive(&below_pixels, median_cut_round - 1);
}

// Default here is 'mean' color values. IE Averaging the colors and then returning them.
fn quantize_values(colors: &Vec<RGBColor>) {
    //// average them
    //let mut r_avg;
    //let mut g_avg;
    //let mut b_avg;

    //let mut r_sum: u32 = 0;
    //let mut g_sum: u32 = 0;
    //let mut b_sum: u32 = 0;
    //for color in colors.iter() {
    //    r_sum += color.r as u32;
    //    g_sum += color.g as u32;
    //    b_sum += color.b as u32;
    //}

    //r_avg = r_sum / colors.len() as u32;
    //g_avg = g_sum / colors.len() as u32;
    //b_avg = b_sum / colors.len() as u32;

    //print!("Color is: {r_avg} - {g_avg} - {b_avg} ->");
    //println!("\u{001b}[48;2;{};{};{}m    \u{001b}[m", r_avg, g_avg, b_avg);

    // NOTE: Try doing mode
    let mut most_common = HashMap::<RGBColor, i32>::new();

    for &color in colors {
        *most_common.entry(color).or_insert(0) += 1;
    }

    let most_common_color = most_common
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap();

    print!(
        "Color is: {} - {} - {} ->",
        most_common_color.r, most_common_color.g, most_common_color.b
    );
    println!(
        "\u{001b}[48;2;{};{};{}m    \u{001b}[m",
        most_common_color.r, most_common_color.g, most_common_color.b
    );
}
