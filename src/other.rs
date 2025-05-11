use rand::prelude::*;
use std::{
    cmp::max,
    collections::{hash_map::Entry, HashMap},
    fmt::format,
    hash::Hash,
    io::Cursor,
    usize,
};

use base64::{engine::general_purpose, prelude::*};
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat, Rgba};

const PALETTE_SIZE: u8 = 16;
const ESCAPE_KEY: &str = "\u{001b}[";
const RESET_KEY: &str = "\u{001b}[m";

// Print out colors
// println!("Average color is: {average:?}");
// println!(
//    "\u{001b}[38;2;{};{};{}mHello\u{001b}[m",
//    average.0[0], average.0[1], average.0[2]
// );

// Save image
//let base64_img = image_to_base64(&img);
//println!("Base 64 Image: {base64_img:?}");
//let back_to_img = base64_to_image(base64_img);
//back_to_img.save("./test_img.png").unwrap();

// Easier struct to work with than to work with the Image crates Rgba struct value of a u8 array
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
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

    fn new_without_location(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct ColorInformation {
    r_min: u8,
    r_max: u8,
    r_range: u8,
    r_median: u8,
    g_min: u8,
    g_max: u8,
    g_range: u8,
    g_median: u8,
    b_min: u8,
    b_max: u8,
    b_range: u8,
    b_median: u8,
    volume: u32,
}

impl ColorInformation {
    fn new(
        r_min: u8,
        r_max: u8,
        r_range: u8,
        r_median: u8,
        g_min: u8,
        g_max: u8,
        g_range: u8,
        g_median: u8,
        b_min: u8,
        b_max: u8,
        b_range: u8,
        b_median: u8,
        volume: u32,
    ) -> Self {
        Self {
            r_min,
            r_max,
            r_range,
            r_median,
            g_min,
            g_max,
            g_range,
            g_median,
            b_min,
            b_max,
            b_range,
            b_median,
            volume,
        }
    }
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

// TODO:
// 1. Clean up code
//      - Split code into more than just main
// 2. Remove commented code where not needed

// TODO: Need to properly get error handling going
fn main() -> Result<(), std::io::Error> {
    // Open Image
    // TODO:
    // 1. Get image via command line args
    // 2. should scale image
    let mut img = image::open(".\\peach-blossom.png").unwrap();
    let mut colors = Vec::<RGBAColor>::new();

    let mut unique_colors = HashMap::<RGBAColor, i32>::new();
    let mut palette = Vec::<RGBAColor>::new();

    // TODO:
    // - Move unique color check into own function
    for (x, y, color) in img.pixels() {
        //let new_color =
        //    RGBAColor::new_without_location(color.0[0], color.0[1], color.0[2], color.0[3]);

        //if let Entry::Vacant(u_color) = unique_colors.entry(new_color) {
        //    u_color.insert(1);
        //} else {
        //    unique_colors
        //        .entry(new_color)
        //        .and_modify(|count| *count += 1);
        //}

        colors.push(RGBAColor::new(
            color.0[0], color.0[1], color.0[2], color.0[3], x, y,
        ));
    }

    //colors = unique_colors.keys().copied().collect();

    let mut another_color: Vec<_> = colors.iter().copied().collect();
    // TODO: allow for higher than 4 right now
    //median_cut(&mut img, &mut colors, &mut palette, 4);
    gen_palette(&mut another_color, 6);

    Ok(())
}

fn median_cut_quantize(
    _img: &mut DynamicImage,
    colors: &Vec<RGBAColor>,
    palette: &mut Vec<RGBAColor>,
) {
    // When it reaches the end, color quantize

    let r_average = (colors.iter().map(|x| x.r as u32).sum::<u32>() / colors.len() as u32) as u8;
    let g_average = (colors.iter().map(|x| x.g as u32).sum::<u32>() / colors.len() as u32) as u8;
    let b_average = (colors.iter().map(|x| x.b as u32).sum::<u32>() / colors.len() as u32) as u8;

    //println!("Red Avg: {r_average} -- Green Avg: {g_average} -- Blue Avg: {b_average}");

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
    let new_color = RGBAColor::new(r_average, g_average, b_average, 255, 0, 0);
    //palette.push(new_color);

    //parse_colors(palette);
    print_palettes(new_color);
}

fn mean_color(colors: &Vec<RGBAColor>) {
    let r_average = (colors.iter().map(|x| x.r as u32).sum::<u32>() / colors.len() as u32) as u8;
    let g_average = (colors.iter().map(|x| x.g as u32).sum::<u32>() / colors.len() as u32) as u8;
    let b_average = (colors.iter().map(|x| x.b as u32).sum::<u32>() / colors.len() as u32) as u8;

    // What we want though is just the colors
    // Print out colors
    let new_color = RGBAColor::new(r_average, g_average, b_average, 255, 0, 0);
    print_palettes(new_color);
}

// TODO:
// - Change name
// - Use cycles to 'name' the lists being returned
// - Can I improve the spliting?
fn more_median_cut(
    color_list: &Vec<RGBAColor>,
    color_info: &ColorInformation,
    cycles: i32,
) -> HashMap<String, Vec<RGBAColor>> {
    // TODO: Change this to use the color info
    let mut color_items: Vec<_> = color_list.iter().copied().collect();
    // Determine largest range to cut on for color
    let ranges = get_range(&mut color_items);
    let largest_ranges = ranges.iter().max().unwrap();

    let mut rng = rand::rng();
    let mut color_to_cut = 0;
    // Use rng to pick slot
    if ranges[0] == 255 && ranges[1] == 255 && ranges[1] == 255 {
        let idx = *ranges.choose(&mut rng).unwrap();
        color_to_cut = ranges.iter().position(|x| *x == idx).unwrap();
    } else {
        color_to_cut = ranges.iter().position(|x| x == largest_ranges).unwrap();
    }
    println!("Color to cut is: {color_to_cut}");

    let mut above_list = Vec::<RGBAColor>::new();
    let mut below_list = Vec::<RGBAColor>::new();
    // TODO: Find a better way to work with it if multiple values are at 255
    match color_to_cut {
        0 => {
            // Above list
            for i in 0..color_list.len() {
                if color_list[i].r >= color_info.r_median {
                    above_list.push(color_list[i]);
                } else {
                    below_list.push(color_list[i]);
                }
            }
            above_list.sort_by(|a, b| a.r.cmp(&b.r));
            below_list.sort_by(|a, b| a.r.cmp(&b.r));
        }
        1 => {
            // Above list
            for i in 0..color_list.len() {
                if color_list[i].g >= color_info.g_median {
                    above_list.push(color_list[i]);
                } else {
                    below_list.push(color_list[i]);
                }
            }

            above_list.sort_by(|a, b| a.g.cmp(&b.g));
            below_list.sort_by(|a, b| a.g.cmp(&b.g));
        }
        2 => {
            for i in 0..color_list.len() {
                if color_list[i].b >= color_info.b_median {
                    above_list.push(color_list[i]);
                } else {
                    below_list.push(color_list[i]);
                }
            }

            above_list.sort_by(|a, b| a.b.cmp(&b.b));
            below_list.sort_by(|a, b| a.b.cmp(&b.b));
        }
        _ => {}
    }

    //println!("Above and below: {} -- {}", above.len(), below.len());
    // Split above and below
    //let colors_above = colors.split_off(median_index);
    //let colors_below = colors.iter().copied().collect::<Vec<_>>();
    //// Clear colors just to make sure we don't have anything weird??
    //colors.clear();

    let mut cut_image = HashMap::<String, Vec<RGBAColor>>::new();
    let above = format!("A{}", cycles);
    let below = format!("B{}", cycles);
    cut_image.insert(above, above_list);
    cut_image.insert(below, below_list);
    return cut_image;
}

// Generate the palette based on the colors of the image passed in
fn gen_palette(image_colors: &mut Vec<RGBAColor>, cycles: i32) {
    let mut cut_img_list = HashMap::<String, Vec<RGBAColor>>::new();
    let mut color_info = ColorInformation::default();
    let mut color_infos = HashMap::<i32, ColorInformation>::new();

    let mut iter = 1;
    while iter < cycles * 2 && (cut_img_list.len() < (cycles as usize)) {
        // Cut the image using the median cut algorithm
        if iter == 1 {
            color_info = get_color_information(image_colors);
            color_infos.insert(iter, color_info);
            cut_img_list = more_median_cut(&image_colors, &color_info, iter);
        } else {
            // Prepare for next loop
            // Find what we want to cut against (range or something else)
            // Then calculate what color to cut against for a 'cube'
            // if both have a range of 255, pick one between the two
            // Ex: First iter -> Range of red is 255, we pick red to cut along
            // Ex: Returns with 2 cubes. Find new color ranges of the 2 cubes
            // Ex: Range of blue is 255 for both, pick cube 2
            // Ex: Keep Cube 1 uncut, and cut on cube 2. Returning a total of 3 cubes.
            // Ex: Rinse and repeat until iter(n) equals the color amount we want

            for (_, colors) in cut_img_list.iter_mut() {
                color_info = get_color_information(colors);
                if !color_infos.contains_key(&iter) {
                    color_infos.insert(iter, color_info);
                }
            }

            // Cut against RANGE for now
            // Determine which is the largest range between the colors
            let cut_criteria = determine_cut_criteria(&color_infos, iter - 1);

            println!("Cut criteria: {cut_criteria}");
            println!("Img cut list: {:?}", cut_img_list.keys());
            // Determine which 'color info' to cut on
            // NOTE: We'll just cut on the first box for now
            // Probably worth using HashMap for identification purposes
            // TODO: we want to determine between 1 or 2 (or more), based on the cut criteria
            let mut cube_to_cut = cut_img_list[&cut_criteria].clone();

            // Cut the box with the 'highest volume' using median cut
            let mut median_cut = more_median_cut(&cube_to_cut, &color_infos[&iter], iter);

            // Remove vectors with 0 length
            let h = median_cut
                .iter()
                .find_map(|(x, y)| if y.len() == 0 { Some(x.clone()) } else { None });

            if let Some(key) = h {
                let _ = median_cut.remove(&key);
            };

            // Remove the cube we cut on
            cut_img_list.extend(median_cut.into_iter());
            let _ = cut_img_list.remove(&cut_criteria);
        }
        iter += 1;
    }

    println!("Cut img list len: {}", cut_img_list.len());
    // Now we want to get the palette generated for colors :)

    let mut palette = Vec::<RGBAColor>::new();

    for (_, item) in cut_img_list.iter() {
        //let mean_color: RGBAColor = {
        //    let r_average =
        //        (item.iter().map(|x| x.r as u32).sum::<u32>() / item.len() as u32) as u8;
        //    let g_average =
        //        (item.iter().map(|x| x.g as u32).sum::<u32>() / item.len() as u32) as u8;
        //    let b_average =
        //        (item.iter().map(|x| x.b as u32).sum::<u32>() / item.len() as u32) as u8;

        //    // What we want though is just the colors
        //    // Print out colors
        //    let new_color = RGBAColor::new(r_average, g_average, b_average, 255, 0, 0);
        //    new_color
        //};

        let mode_color = mode_color(item);

        palette.push(mode_color);
    }

    for color in palette.iter() {
        print_palettes(*color);
    }
}

// Calculate the range of the boxes
// Returning the one with the largest range
// If multiple values have a range of 255, pick a random one
fn determine_cut_criteria(color_infos: &HashMap<i32, ColorInformation>, cycles: i32) -> String {
    let mut maxes = Vec::<u8>::new();
    for (_, info) in color_infos.iter() {
        let mx = max(
            info.r_range.max(info.g_range),
            info.r_range.max(info.b_range),
        );
        maxes.push(mx);
    }
    let f = maxes.iter().max().unwrap();
    // let mut rng = rand::rng();
    // let mut color_to_cut = 0;
    // // Use rng to pick slot
    // if ranges[0] == 255 || ranges[1] == 255 || ranges[1] == 255 {
    //     let idx = *ranges.choose(&mut rng).unwrap();
    //     color_to_cut = ranges.iter().position(|x| *x == idx).unwrap();
    // } else {
    //     color_to_cut = ranges.iter().position(|x| x == largest_ranges).unwrap();
    // }
    let pos = maxes.iter().position(|x| x == f).unwrap();

    if pos % 2 == 0 {
        return format!("A{}", cycles);
    } else {
        return format!("B{}", cycles);
    }
}

fn get_range(item: &mut Vec<RGBAColor>) -> Vec<u8> {
    // Sort by Color for easier checking first
    // Since max/min stuff has been coming back with some weird errors
    let r_max = item.iter().max_by(|x, y| x.r.cmp(&y.r)).unwrap();
    let r_min = item.iter().min_by(|x, y| x.r.cmp(&y.r)).unwrap();
    let r_range = r_max.r - r_min.r;

    let g_max = item.iter().max_by(|x, y| x.g.cmp(&y.g)).unwrap();
    let g_min = item.iter().min_by(|x, y| x.g.cmp(&y.g)).unwrap();
    let g_range = g_max.g - g_min.g;

    let b_max = item.iter().max_by(|x, y| x.b.cmp(&y.b)).unwrap();
    let b_min = item.iter().min_by(|x, y| x.b.cmp(&y.b)).unwrap();
    let b_range = b_max.b - b_min.b;

    let ranges = vec![r_range, g_range, b_range];
    ranges
}

fn mode_color(colors: &Vec<RGBAColor>) -> RGBAColor {
    let mut counted_colors = HashMap::<RGBAColor, i32>::new();

    for &color in colors {
        *counted_colors.entry(color).or_insert(0) += 1;
    }

    counted_colors
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap()
}

fn get_color_information(colors: &mut Vec<RGBAColor>) -> ColorInformation {
    // Find out which color channel has the greatest range
    let r_max = colors.iter().max().map(|x| x.r).expect("found no max");
    let r_min = colors.iter().min().map(|x| x.r).expect("found no min");
    let g_max = colors.iter().max().map(|x| x.g).expect("found no max");
    let g_min = colors.iter().min().map(|x| x.g).expect("found no min");
    let b_max = colors.iter().max().map(|x| x.b).expect("found no max");
    let b_min = colors.iter().min().map(|x| x.b).expect("found no min");
    let ranges = get_range(colors);

    // Find the median and split
    let median_index = colors.len() / 2;

    let r_median = {
        colors.sort_by(|a, b| a.r.cmp(&b.r));
        if colors.len() % 2 == 0 {
            (colors[median_index - 1].r as u16 + colors[median_index].r as u16) / 2
        } else {
            colors[median_index].r as u16
        }
    };
    let g_median = {
        colors.sort_by(|a, b| a.g.cmp(&b.g));
        if colors.len() % 2 == 0 {
            (colors[median_index - 1].g as u16 + colors[median_index].g as u16) / 2
        } else {
            colors[median_index].g as u16
        }
    };

    let b_median = {
        colors.sort_by(|a, b| a.b.cmp(&b.b));
        if colors.len() % 2 == 0 {
            (colors[median_index - 1].b as u16 + colors[median_index].b as u16) / 2
        } else {
            colors[median_index].b as u16
        }
    };

    let volume: u32 = (ranges[0] as u32 * ranges[1] as u32 * ranges[2] as u32).into();

    let color_info = ColorInformation::new(
        r_min,
        r_max,
        ranges[0],
        r_median as u8,
        g_min,
        g_max,
        ranges[1],
        g_median as u8,
        b_min,
        b_max,
        ranges[2],
        b_median as u8,
        volume,
    );
    return color_info;
}

// TODO: Better documentation comments
// Colors -> 'Image Array'
// Depth -> How many colors are needed in the power of 2. So example: 4 -> 2^4 = 16, so 16 colors
// would be "found"
// Recursive median cut
fn median_cut(
    img: &mut DynamicImage,
    colors: &mut Vec<RGBAColor>,
    palette: &mut Vec<RGBAColor>,
    depth: i32,
) {
    if colors.len() == 0 {
        return;
    }

    if depth == 0 {
        // Call 'median_cut_quantize'
        // Basically Actually average the colors and print them
        median_cut_quantize(img, colors, palette);
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

    // Found range with largest distance
    // Now sort by that.
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
    //println!("Median index: {median_index}");

    // Split into buckets (upper and lower side)
    let mut colors_upper = colors.split_off(median_index);

    // **Recursive call
    // Call median cut "twice"
    // -> colors
    // -> colors_upper
    median_cut(img, colors, palette, depth - 1);
    median_cut(img, &mut colors_upper, palette, depth - 1);
}

/*
* Outline of the algorithm:
* - Assuming we have input data points x1,x2,x3,…,xn and value of K (the number of clusters needed). We follow the below procedure:
* 1. Pick K points as the initial centroids from the dataset, either randomly or the first K.
* 2. Find the Euclidean distance of each point in the dataset with the identified K points (cluster centroids).
* 3. Assign each data point to the closest centroid using the distance found in the previous step.
* 4. Find the new centroid by taking the average of the points in each cluster group.
* 5. Repeat 2 to 4 for a fixed number of iteration or till the centroids don’t change.
*/
// TODO: work on this later
fn k_means(img: &DynamicImage, colors: &Vec<RGBAColor>) {
    // First pick K values (number of clusters needed; First will go with 3. Picking randomly.
    // TODO: Actually do a random value for 3 of them, for now go with 0 (top left corner), height/ 2 * width /2 and height * width
    let sample_points: Vec<u32> = vec![
        0,
        (img.height() / 2 * img.width() / 2),
        img.height() * img.width(),
    ];

    // Find the euclidian distance between two points in space
}

// TODO: work on later
fn find_distance(point_one: RGBAColor, point_two: RGBAColor) {}

// TODO:
// 1. Need to get this to properly print out in 2 rows
fn print_palettes(color: RGBAColor) {
    // What we want though is just the colors
    // Print out colors
    print!("RBG is: {} {} {} -> ", color.r, color.g, color.b);
    println!(
        "\u{001b}[48;2;{};{};{}m    \u{001b}[m",
        color.r, color.g, color.b
    );
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

// TODO: probably still want to keep this for k means, or some interesting colors for median cut
fn caclulate_color_distance(color_one: RGBAColor, color_two: RGBAColor) -> f32 {
    f32::sqrt(
        (i32::pow((color_one.r as i32 - color_two.r as i32).into(), 2)
            + i32::pow((color_one.g as i32 - color_two.g as i32).into(), 2)
            + i32::pow((color_one.b as i32 - color_two.b as i32).into(), 2)) as f32,
    )
}

fn is_color_too_similar(palette: &Vec<RGBAColor>, num_colors: u8, new_color: RGBAColor) -> bool {
    // We go until Num_colors
    for i in 0..num_colors {
        if caclulate_color_distance(palette[i as usize], new_color) < 35.0 {
            // Color is too similar, we need to blend
            return true;
        }
    }

    return false;
}

fn blend_colors(color_one: RGBAColor, color_two: RGBAColor, weight: f32) -> RGBAColor {
    let blended_color = RGBAColor::new_without_location(
        (color_one.r as f32 * (1.0 - weight) + color_two.r as f32 * weight) as u8,
        (color_one.g as f32 * (1.0 - weight) + color_two.g as f32 * weight) as u8,
        (color_one.b as f32 * (1.0 - weight) + color_two.b as f32 * weight) as u8,
        255,
    );
    return blended_color;
}
