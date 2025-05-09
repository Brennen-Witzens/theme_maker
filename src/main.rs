use core::num;
use std::{
    cell::RefCell,
    cmp::{max, min, min_by},
    collections::{self, hash_map::Entry, HashMap},
    io::Cursor,
    ops::Index,
    rc::Rc,
    usize,
};

use base64::{engine::general_purpose, prelude::*};
use image::{io::Reader, DynamicImage, GenericImage, GenericImageView, ImageFormat, Rgba};

const PALETTE_SIZE: u8 = 16;
const ESCAPE_KEY: &str = "\u{001b}[";
const RESET_KEY: &str = "\u{001b}[m";

// Print out colors
// println!("Average color is: {average:?}");
// println!(
//    "\u{001b}[38;2;{};{};{}mHello\u{001b}[m",
//    average.0[0], average.0[1], average.0[2]
// );

// Easier struct to work with than to work with the Image crates Rgba struct value of a u8 array
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RGBAColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    x: u32,
    y: u32,
}

impl RGBAColor {
    fn new(r: u8, g: u8, b: u8, a: u8, x: u32, y: u32) -> Self {
        Self { r, g, b, a, x, y }
    }
}

// TODO: Need to properly get error handling going
fn main() -> Result<(), std::io::Error> {
    let mut unique_colors = HashMap::<Rgba<u8>, i32>::new();

    // Open Image
    // TODO:
    // 1. Get image via command line args
    // 2. should scale image
    let mut img = image::open(".\\peach-blossom.png").unwrap();
    let mut colors = Vec::<RGBAColor>::new();

    for (x, y, color) in img.pixels() {
        //if let Entry::Vacant(unique_colors) = unique_colors.entry(color) {
        //    unique_colors.insert(1);
        //} else {
        //    unique_colors.entry(color).and_modify(|count| *count += 1);
        //}
        colors.push(RGBAColor::new(
            color.0[0], color.0[1], color.0[2], color.0[3], x, y,
        ));
    }

    //println!("Unique Colors: {}", unique_colors.keys().count());
    //let colors: Vec<Rgba<u8>> = unique_colors.keys().map(|x| *x).collect();
    median_cut(&mut img, &mut colors, 4);
    //let val = parse_unique_colors(&colors, &img);

    // let mut palettes = Vec::<Vec<Rgba<u8>>>::new();
    //
    //    let val = find_ten_unique_colors(&colors);
    //
    // let chunked: Vec<_> = val.chunks(16).collect();
    // let mut chunked_palettes: Vec<_> = chunked.iter().map(|x| x.to_vec()).collect();

    // for c_palettes in chunked_palettes.iter() {
    //     let mut palette = Vec::<Rgba<u8>>::new();
    //     for (j, color) in c_palettes.iter().enumerate() {
    //         if !is_color_too_similar(c_palettes, NUM_COLORS, *color) {
    //             palette.push(*color);
    //         } else {
    //             let blend = blend_colors(*color, c_palettes[j], 0.5);
    //             palette.push(blend);
    //         }
    //     }

    //     palettes.push(palette.clone());
    // }

    //print_palettes(palettes);
    //let base64_img = image_to_base64(&img);
    //println!("Base 64 Image: {base64_img:?}");
    //let back_to_img = base64_to_image(base64_img);
    //back_to_img.save("./test_img.png").unwrap();
    Ok(())
}

fn median_cut_quantize(img: &mut DynamicImage, colors: &Vec<RGBAColor>) {
    // When it reaches the end, color quantize
    //println!("To Quantize: {}", colors.len());

    let r_average = (colors.iter().map(|x| x.r as u32).sum::<u32>() / colors.len() as u32) as u8;
    let g_average = (colors.iter().map(|x| x.g as u32).sum::<u32>() / colors.len() as u32) as u8;
    let b_average = (colors.iter().map(|x| x.b as u32).sum::<u32>() / colors.len() as u32) as u8;

    println!("Red Avg: {r_average} -- Green Avg: {g_average} -- Blue Avg: {b_average}");

    // This replicates the image by using the colors we found :)
    //for data in colors.iter() {
    //    img.put_pixel(
    //        data.x,
    //        data.y,
    //        Rgba::<u8> {
    //            0: [r_average, g_average, b_average, data.a],
    //        },
    //    );
    //}
    //img.save("./color_test.png").unwrap();

    // What we want though is just the colors
    // Print out colors
    println!(
        "Color averages: Red Avg: {r_average} -- Green Avg: {g_average} -- Blue Avg: {b_average} ->"
    );
    println!(
        "\u{001b}[48;2;{};{};{}m    \u{001b}[m",
        r_average, g_average, b_average
    );
}

// Colors -> 'Image Array'
// Depth -> How many colors are needed in the power of 2. So example: 4 -> 2^4 = 16, so 16 colors
// would be "found"
fn median_cut(img: &mut DynamicImage, colors: &mut Vec<RGBAColor>, depth: i32) {
    if colors.len() == 0 {
        return;
    }

    if depth == 0 {
        // Call 'median_cut_quantize'
        // Basically Actually average the colors and print them
        median_cut_quantize(img, colors);
        return;
    }

    // Find out which color channel has the greatest range
    let mut r_range;
    let mut g_range;
    let mut b_range;
    let mut r_max = 0;
    let mut r_min = 0;
    let mut g_max = 0;
    let mut g_min = 0;
    let mut b_max = 0;
    let mut b_min = 0;

    r_max = colors.iter().max().map(|x| x.r).expect("found no max");
    r_min = colors.iter().min().map(|x| x.r).expect("found no min");
    r_range = r_max - r_min;
    g_max = colors.iter().max().map(|x| x.g).expect("found no max");
    g_min = colors.iter().min().map(|x| x.g).expect("found no min");
    g_range = g_max - g_min;
    b_max = colors.iter().max().map(|x| x.b).expect("found no max");
    b_min = colors.iter().min().map(|x| x.b).expect("found no min");
    b_range = b_max - b_min;

    //print!("Red Range: {r_range} -- Green Range: {g_range} -- Blue Range: {b_range}");

    // Found range with largest distance
    //  Now sort by that.

    if r_range >= b_range && r_range >= g_range {
        colors.sort_by(|a, b| a.r.cmp(&b.r));
    }
    if g_range >= r_range && g_range >= b_range {
        colors.sort_by(|a, b| a.g.cmp(&b.g));
    }
    if b_range >= r_range && b_range >= g_range {
        colors.sort_by(|a, b| a.b.cmp(&b.b));
    }

    // Find the median and split
    let median_index = (colors.len() + 1) / 2;
    println!("Median index: {median_index}");

    // Split into buckets (upper and lower side)
    // **Recursive call
    let mut colors_lower = colors.split_off(median_index);

    // Call median cut "twice"
    // -> colors
    // -> colors_lower
    median_cut(img, colors, depth - 1);
    median_cut(img, &mut colors_lower, depth - 1);
}

// TODO:
// 1. Need to get this to properly print out in 2 rows
// 2. Need to work on getting the palette size cut down (i do not need to have 500+ values)
fn print_palettes(palettes: Vec<Vec<Rgba<u8>>>) {
    // We got a fuck ton of palettes generated (most are probably similar in some way)
    // Will print 2 or 3 palettes
    // TODO: can get random values
    // - A start for printing the colors

    let rand_a = 100;
    let rand_b = 700;
    let rand_c = 1200;

    for (i, palette) in palettes.iter().enumerate() {
        if i == rand_a {
            println!("Palette One");
            for x in 0..palette.len() {
                println!(
                    "{}48;2;{};{};{}m    {}",
                    ESCAPE_KEY, palette[x].0[0], palette[x].0[1], palette[x].0[2], RESET_KEY
                );
                println!(
                    "{}48;2;{};{};{}m    {}",
                    ESCAPE_KEY, palette[x].0[0], palette[x].0[1], palette[x].0[2], RESET_KEY
                );
            }
        } else if i == rand_b {
            println!("Palette Two");
            for x in 0..palette.len() {
                println!(
                    "{}48;2;{};{};{}m    {}",
                    ESCAPE_KEY, palette[x].0[0], palette[x].0[1], palette[x].0[2], RESET_KEY
                );
                println!(
                    "{}48;2;{};{};{}m    {}",
                    ESCAPE_KEY, palette[x].0[0], palette[x].0[1], palette[x].0[2], RESET_KEY
                );
            }
        } else if i == rand_c {
            println!("Palette Three");
            for x in 0..palette.len() {
                println!(
                    "{}48;2;{};{};{}m    {}",
                    ESCAPE_KEY, palette[x].0[0], palette[x].0[1], palette[x].0[2], RESET_KEY
                );
                println!(
                    "{}48;2;{};{};{}m    {}",
                    ESCAPE_KEY, palette[x].0[0], palette[x].0[1], palette[x].0[2], RESET_KEY
                );
            }
        }
    }
}

// TODO: Need to re-work this potentially
fn average_color(palette: &Vec<Rgba<u8>>, end: u8) -> Rgba<u8> {
    let mut sum_r: i32 = 0;
    let mut sum_g: i32 = 0;
    let mut sum_b: i32 = 0;

    for i in 0..end {
        // If we've gone through end number values, break out
        sum_r += palette[i as usize].0[0] as i32;
        sum_g += palette[i as usize].0[1] as i32;
        sum_b += palette[i as usize].0[2] as i32;

        //let test: Vec<i32> = img.pixels().map(|g| g.2 .0[1].into()).collect();
        //sum_g = test.iter().sum();
        //let test: Vec<i32> = img.pixels().map(|b| b.2 .0[2].into()).collect();
        //sum_b = test.iter().sum();
    }

    let red_avg = sum_r / end as i32;
    let green_avg = sum_g / end as i32;
    let blue_avg = sum_b / end as i32;

    let avg = Rgba::<u8> {
        0: [red_avg as u8, green_avg as u8, blue_avg as u8, 255],
    };
    return avg;
}

// TODO: still want to keep these but will probably move functions and modules around
fn image_to_base64(img: &DynamicImage) -> String {
    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), ImageFormat::Jpeg)
        .unwrap();

    let res_base64 = general_purpose::STANDARD.encode(image_data);
    format!("{}", res_base64)
}

// TODO: still want to keep these but will probably move functions and modules around
fn base64_to_image(str: String) -> DynamicImage {
    let decoded_string = general_purpose::STANDARD.decode(str).unwrap();
    let reader = Reader::new(Cursor::new(decoded_string))
        .with_guessed_format()
        .expect("Should of found a image format");

    let image = reader.decode().unwrap();
    image
}

fn parse_unique_colors(colors: &Vec<Rgba<u8>>, img: &DynamicImage) -> Vec<Rgba<u8>> {
    let mut unique_colors = HashMap::<Rgba<u8>, i32>::new();
    let mut palette = Vec::<Rgba<u8>>::new();
    let mut avg_color_palette = Vec::<Rgba<u8>>::new();
    // TODO: Change to RC/ARC for better performance and better understanding of how to use them

    let mut num_colors = 0;
    let chunked_unique: Vec<_> = colors.chunks_exact(16).collect();

    palette = chunked_unique[0].to_vec().clone();

    let sample_points: Vec<u32> = vec![
        0,
        img.height(),
        img.width(),
        (img.height() - 250 + img.width() - 1000),
        (img.height() / 3 + img.width() / 3),
        ((img.height() / 4) + (img.width() / 4)),
        img.height() * 2,
        img.width() * 2,
    ];

    println!("Chunked unique length: {}", chunked_unique.len());
    for i in 0..PALETTE_SIZE {
        let mut new_color = colors[num_colors as usize];

        if !is_color_too_similar(&palette, num_colors, new_color) {
            // Color is NOT too similar
            //println!("Color is NOT too similar");
            //for color in chunked_unique[0] {
            //    print!(" Color is: {color:?} -> ");
            //    println!(
            //        "{}48;2;{};{};{}m    {}",
            //        ESCAPE_KEY, color.0[0], color.0[1], color.0[2], RESET_KEY
            //    );
            //}

            // Add it to the values... probably incorrect
            palette[i as usize] = new_color;
        } else {
            // Color IS too similar
            //println!("Color IS too similar");
            //for color in chunked_unique[0] {
            //    print!(" Color is: {color:?} -> ");
            //    println!(
            //        "{}48;2;{};{};{}m    {}",
            //        ESCAPE_KEY, color.0[0], color.0[1], color.0[2], RESET_KEY
            //    );
            //}

            new_color = blend_colors(new_color, palette[i as usize], 0.5);
            //println!("Blended: {:?}", blended);
            //println!(
            //    "\u{001b}[48;2;{};{};{}m    \u{001b}[m",
            //    blended.0[0], blended.0[1], blended.0[2]
            //);
            palette[i as usize] = new_color;
        }

        num_colors += 1;
    }

    //for color in chunked_unique[0].iter() {
    //    if !is_color_too_similar(
    //        &chunked_unique[0].to_vec(),
    //        PALETTE_SIZE,
    //        chunked_unique[0][0],
    //    ) {
    //        // Color is NOT too similar
    //        println!("Iterating Colors: {:?}", color);
    //        println!(
    //            "\u{001b}[38;2;{};{};{}mHello\u{001b}[m",
    //            color.0[0], color.0[1], color.0[2]
    //        );
    //    } else {
    //        let blended = blend_colors(chunked_unique[0][0], *color, 0.5);
    //        println!(
    //            "Iterating Colors: {:?} -- 'First color': {:?} -> Blended Color: {:?}",
    //            color, chunked_unique[0][0], blended
    //        );
    //        println!(
    //            "\u{001b}[38;2;{};{};{}mHello\u{001b}[m",
    //            blended.0[0], blended.0[1], blended.0[2]
    //        );
    //    }
    //}

    //let test: Vec<Rgba<u8>> = unique_colors.keys().map(|x| *x).collect();

    //// Grab x number of random colors from the Colors vec to be "picked" as new_color

    //for j in 0..PALETTE_SIZE {
    //    let mut i: u32 = 0;

    //    if j >= NUM_COLORS {
    //        i = (PALETTE_SIZE - j) as u32;
    //    } else {
    //        i = sample_colors[j as usize];
    //    }
    //    let mut new_color = colors[i as usize];
    //    println!("-> What is new color: {new_color:?}");
    //    if !is_color_too_similar(&test, NUM_COLORS, new_color) {
    //        // Color is NOT too similar
    //        palette.push(new_color);
    //    } else {
    //        // Color IS too similar
    //        new_color = blend_colors(new_color, test[j as usize], 0.5);
    //        palette.push(new_color);
    //    }
    //}

    for color in palette.iter() {
        println!("Palette color is: {color:?}");
        println!(
            "\u{001b}[48;2;{};{};{}m    \u{001b}[m",
            color.0[0], color.0[1], color.0[2]
        );
    }

    println!("Palette length: {}", palette.len());
    palette
}

fn caclulate_color_distance(color_one: Rgba<u8>, color_two: Rgba<u8>) -> f32 {
    //println!("Color One: {color_one:?} -- Color Two: {color_two:?}");
    f32::sqrt(
        (i32::pow((color_one.0[0] as i32 - color_two.0[0] as i32).into(), 2)
            + i32::pow((color_one.0[1] as i32 - color_two.0[1] as i32).into(), 2)
            + i32::pow((color_one.0[2] as i32 - color_two.0[2] as i32).into(), 2)) as f32,
    )
}

fn is_color_too_similar(palette: &Vec<Rgba<u8>>, num_colors: u8, new_color: Rgba<u8>) -> bool {
    // We go until Num_colors
    for i in 0..num_colors {
        if caclulate_color_distance(palette[i as usize], new_color) < 35.0 {
            // Color is too similar, we need to blend
            return true;
        }
    }

    return false;
}

fn blend_colors(color_one: Rgba<u8>, color_two: Rgba<u8>, weight: f32) -> Rgba<u8> {
    let blended_color: Rgba<u8> = Rgba::<u8> {
        0: [
            (color_one.0[0] as f32 * (1.0 - weight) + color_two.0[0] as f32 * weight) as u8,
            (color_one.0[1] as f32 * (1.0 - weight) + color_two.0[1] as f32 * weight) as u8,
            (color_one.0[2] as f32 * (1.0 - weight) + color_two.0[2] as f32 * weight) as u8,
            255.0 as u8,
        ],
    };
    return blended_color;
}

// TODO:
// Need to get rgb to hexadecimal working
fn convert_rgb_to_hexadecimal() {}
