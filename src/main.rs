use std::{
    collections::HashMap, error::Error, intrinsics::drop_in_place, io::Cursor, num, ops::Div,
    sync::mpsc::channel, usize, vec,
};

use base64::{engine::general_purpose, prelude::*};
use image::{
    io::Reader, DynamicImage, GenericImage, GenericImageView, ImageFormat, Pixel, Rgb, Rgba,
};

const PALETTE_SIZE: u8 = 16;
const NUM_COLORS: u8 = 8;
const ESCAPE_KEY: &str = "\u{001b}[";
const RESET_KEY: &str = "\u{001b}[m";

// Print out colors
//println!("Average color is: {average:?}");
//println!(
//    "\u{001b}[38;2;{};{};{}mHello\u{001b}[m",
//    average.0[0], average.0[1], average.0[2]
//);

fn main() -> Result<(), std::io::Error> {
    // Open Image
    // TODO: should scale image
    let img = image::open(".\\peach-blossom.png").unwrap();

    let mut colors = Vec::<Rgba<u8>>::new();
    for (_, _, color) in img.pixels() {
        colors.push(color);
    }

    let mut palettes = Vec::<Vec<Rgba<u8>>>::new();

    let val = find_ten_unique_colors(&colors);

    let chunked: Vec<_> = val.chunks(16).collect();
    let mut chunked_palettes: Vec<_> = chunked.iter().map(|x| x.to_vec()).collect();

    for c_palettes in chunked_palettes.iter() {
        let mut palette = Vec::<Rgba<u8>>::new();
        for (j, color) in c_palettes.iter().enumerate() {
            if !is_color_too_similar(c_palettes, NUM_COLORS, *color) {
                palette.push(*color);
            } else {
                let blend = blend_colors(*color, c_palettes[j], 0.5);
                palette.push(blend);
            }
        }

        palettes.push(palette.clone());
    }

    print_palettes(palettes);
    //let base64_img = image_to_base64(&img);
    //println!("Base 64 Image: {base64_img:?}");
    //let back_to_img = base64_to_image(base64_img);
    //back_to_img.save("./test_img.png").unwrap();
    Ok(())
}

fn print_palettes(palettes: Vec<Vec<Rgba<u8>>>) {
    // We got a fuck ton of palettes generated (most are probably similar in some way)
    // Will print 2 or 3 palettes
    // TODO: can get random values
    // - A start for printing the colors
    for i in 0..palettes[0].len() {
        println!(
            "{}48;2;{};{};{}m    {}",
            ESCAPE_KEY, palettes[0][i].0[0], palettes[0][i].0[1], palettes[0][i].0[2], RESET_KEY
        );
        println!(
            "{}48;2;{};{};{}m    {}",
            ESCAPE_KEY, palettes[0][i].0[0], palettes[0][i].0[1], palettes[0][i].0[2], RESET_KEY
        );
    }
}

fn average_color(palette: &Vec<Rgba<u8>>, end: u8) -> Rgba<u8> {
    let mut sum_r: i32 = 0;
    let mut sum_g: i32 = 0;
    let mut sum_b: i32 = 0;

    for color in palette.iter() {
        // If we've gone through end number values, break out
        sum_r += color.0[0] as i32;
        sum_g += color.0[1] as i32;
        sum_b += color.0[2] as i32;

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

fn image_to_base64(img: &DynamicImage) -> String {
    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), ImageFormat::Jpeg)
        .unwrap();

    let res_base64 = general_purpose::STANDARD.encode(image_data);
    format!("{}", res_base64)
}

fn base64_to_image(str: String) -> DynamicImage {
    let decoded_string = general_purpose::STANDARD.decode(str).unwrap();
    let reader = Reader::new(Cursor::new(decoded_string))
        .with_guessed_format()
        .expect("Should of found a image format");

    let image = reader.decode().unwrap();
    image
}

// TODO:
// Refactor plans
// - Do unique colors first
// - Iterate over unique colors, bring down the values to a smaller number
// - Average this smaller number
// - Then determine distance etc,

fn find_ten_unique_colors(colors: &Vec<Rgba<u8>>) -> Vec<Rgba<u8>> {
    let mut unique_colors = HashMap::<Rgba<u8>, i32>::new();
    let mut palette = Vec::<Rgba<u8>>::new();
    let mut avg_color_palette = Vec::<Rgba<u8>>::new();
    // TODO: Change to RC/ARC for better performance and better understanding of how to use them

    // Need to "average" the colors in the palette vec
    // Currently we have every pixel color in there. We need to bring that down a TON
    // We want to group them every 16 pixels?

    let chunked_palette: Vec<_> = colors.chunks(16).collect();

    for val in chunked_palette.iter() {
        let average = average_color(&val.to_vec(), PALETTE_SIZE);
        avg_color_palette.push(average);
    }

    for val in avg_color_palette.iter() {
        if !unique_colors.contains_key(val) {
            unique_colors.insert(*val, 1);
        } else {
            unique_colors.entry(*val).and_modify(|count| *count += 1);
        }
    }

    let test: Vec<Rgba<u8>> = unique_colors.keys().map(|x| *x).collect();

    for (i, key) in test.iter().enumerate() {
        if !is_color_too_similar(&test, NUM_COLORS, *key) {
            // Color is NOT too similar
            palette.push(*key);
        } else {
            // Color IS too similar
            let new_color = blend_colors(*key, test[i], 0.5);
            palette.push(new_color);
        }
    }
    palette

    //let mut sorted_unique_colors: Vec<_> = unique_colors.iter().collect();

    //sorted_unique_colors.sort_by(|a, b| b.1.cmp(a.1));

    //// Drain everything after top 10 out of sorted_unique_colors
    //// Ignore everything after the 10
    //let _: Vec<_> = sorted_unique_colors.drain(50..).collect();

    // Convert colors to hex
    // Organize print out
    // Display colors

    //for (color, _) in sorted_unique_colors.into_iter() {
    //    println!("Colors: {:?}", color);
    //    println!(
    //        "\u{001b}[38;2;{};{};{}m{}\u{001b}[m",
    //        color.0[0], color.0[1], color.0[2], "Hello"
    //    );
    //}

    //    for color in palette.iter() {
    //        println!("Palette: {:?}", color);
    //        println!(
    //            "\u{001b}[38;2;{};{};{}mHello\u{001b}[m",
    //            color.0[0], color.0[1], color.0[2]
    //        );
    //    }

    //for color in new_palette.iter() {
    //    println!("New_Palette: {:?}", color);
    //    println!(
    //        "\u{001b}[38;2;{};{};{}mHello\u{001b}[m",
    //        color.0[0], color.0[1], color.0[2]
    //    );
    //}
}

fn caclulate_color_distance(color_one: Rgba<u8>, color_two: Rgba<u8>) -> f32 {
    //println!("Color One: {color_one:?} -- Color Two: {color_two:?}");
    f32::sqrt(
        f32::powf((color_one.0[0] as f32 - color_two.0[0] as f32).into(), 2.0)
            + f32::powf((color_one.0[1] as f32 - color_two.0[1] as f32).into(), 2.0)
            + f32::powf((color_one.0[2] as f32 - color_two.0[2] as f32).into(), 2.0),
    )
}

fn is_color_too_similar(palette: &Vec<Rgba<u8>>, num_colors: u8, new_color: Rgba<u8>) -> bool {
    // We go until Num_colors
    for i in 0..num_colors {
        //println!(
        //    "Is Color too Similar: {:?} -- New_Color: {:?} -- Distance: {}",
        //    palette[i as usize],
        //    new_color,
        //    (caclulate_color_distance(palette[i as usize], new_color) < 35.0)
        //);
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
