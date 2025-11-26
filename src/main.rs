mod algorithms;
mod utils;

use clap::{Arg, Command};
use image::GenericImageView;
use std::{
    collections::HashMap,
    fmt::format,
    path::{Path, PathBuf},
};

// NOTE: dont necessarily want to use clone...
#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGBColor {
    pub fn build_color(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Debug)]
enum RGB {
    Red(u8),
    Green(u8),
    Blue(u8),
}

enum ColorExtractionType {
    Mean,
    Median,
    Mode,
}

// default palette size
const DEFAULT_PALETTE_SIZE: u8 = 6;
// default color extraction type
const DEFAULT_COLOR_EXTRACTION_METHOD: ColorExtractionType = ColorExtractionType::Mode;

fn main() {
    let image_path = build_image_path();

    if let Some(path) = image_path {
        let colors = get_pixels_from_image(&path);

        // Take the colors from the image and calculate the range of each component
        let range = find_color_range(&colors);

        // Once we have the range, we need to split the values on the largest component and then
        // find the median value and split the cubes to upper and lower values.
        // NOTE: might be worth using a map for this
        let mut median_split = find_median(&range, colors, 1);

        // We have the first 2 cubes, now we need to get the rest
        for i in 2..DEFAULT_PALETTE_SIZE {
            let cube = median_split.get(&format(format_args!("Cube{i}")));
            if let Some(cube) = cube {
                let range = find_color_range(cube);

                median_split.extend(find_median(&range, cube.clone(), i));
            }
        }

        // After we've exited the loop, we need to get the overall color for each cube
        // We can determine what style to use -> Mean, Median, Mode for the cube, default will be
        // Mode (most common)
        for cube in median_split.values() {
            //println!("Length of cube: {}", cube.len());
            let extracted_color = extract_color_from_cube(cube, ColorExtractionType::Mean);
            convert_rgb_to_hex(&extracted_color);
        }
    }
}

fn convert_rgb_to_hex(color: &RGBColor) {
    let red_quotient = color.red / 16;
    let red_remainder = color.red - (red_quotient * 16);

    let green_quotient = color.green / 16;
    let green_remainder = color.green - (green_quotient * 16);

    let blue_quotient = color.blue / 16;
    let blue_remainder = color.blue - (blue_quotient * 16);

    print!("RBG is: {} {} {} -> ", color.red, color.green, color.blue);
    print!(
        "Hex is: {:x}{:x}{:x} -> ",
        color.red, color.green, color.blue
    );
    println!(
        "\u{001b}[48;2;{};{};{}m    \u{001b}[m",
        color.red, color.green, color.blue
    );
}

fn extract_color_from_cube(colors: &[RGBColor], extraction_style: ColorExtractionType) -> RGBColor {
    let color: RGBColor;
    match extraction_style {
        ColorExtractionType::Mean => {
            let red_sum: u32 = colors.iter().map(|x| x.red as u32).sum();
            let red_mean = red_sum / colors.len() as u32;

            let green_sum: u32 = colors.iter().map(|x| x.green as u32).sum();
            let green_mean = green_sum / colors.len() as u32;

            let blue_sum: u32 = colors.iter().map(|x| x.blue as u32).sum();
            let blue_mean = blue_sum / colors.len() as u32;

            color = RGBColor::build_color(red_mean as u8, green_mean as u8, blue_mean as u8);
            color
        }
        ColorExtractionType::Median => todo!(),
        ColorExtractionType::Mode => {
            let mut common_color = HashMap::<u8, u32>::new();
            let red_common = {
                for color in colors.iter() {
                    if !common_color.contains_key(&color.red) {
                        common_color.insert(color.red, 1);
                    } else {
                        common_color.entry(color.red).and_modify(|x| *x += 1);
                    }
                }

                common_color
                    .clone()
                    .into_iter()
                    .max_by_key(|&(_, count)| count)
                    .map(|(val, _)| val)
                    .unwrap()
            };

            common_color.clear();

            let green_common = {
                for color in colors.iter() {
                    if !common_color.contains_key(&color.green) {
                        common_color.insert(color.green, 1);
                    } else {
                        common_color.entry(color.green).and_modify(|x| *x += 1);
                    }
                }

                common_color
                    .clone()
                    .into_iter()
                    .max_by_key(|&(_, count)| count)
                    .map(|(val, _)| val)
                    .unwrap()
            };
            common_color.clear();
            let blue_common = {
                for color in colors.iter() {
                    if !common_color.contains_key(&color.blue) {
                        common_color.insert(color.blue, 1);
                    } else {
                        common_color.entry(color.blue).and_modify(|x| *x += 1);
                    }
                }
                common_color
                    .clone()
                    .into_iter()
                    .max_by_key(|&(_, count)| count)
                    .map(|(val, _)| val)
                    .unwrap()
            };
            println!("Most common values: {red_common:?} -- {green_common} -- {blue_common}");
            color = RGBColor::build_color(red_common, green_common, blue_common);
            color
        }
    }
}

fn find_median(
    color_to_cut: &RGB,
    mut colors: Vec<RGBColor>,
    idx: u8,
) -> HashMap<String, Vec<RGBColor>> {
    let mut upper_values: Vec<RGBColor> = Vec::new();
    let mut lower_values: Vec<RGBColor> = Vec::new();
    let median: u8;
    let median_idx = colors.len() / 2;

    println!("Color to cut: {color_to_cut:?}");

    match color_to_cut {
        RGB::Red(val) => {
            println!("Red Value: {val}");
            colors.sort_by(|x, y| x.red.cmp(&y.red));
            if colors.len().is_multiple_of(2) {
                median = (colors[median_idx - 1].red + colors[median_idx].red) / 2;
            } else {
                median = colors[median_idx].red;
            }
            for color in colors {
                if color.red >= median {
                    upper_values.push(color);
                } else {
                    lower_values.push(color);
                }
            }
        }
        RGB::Green(val) => {
            println!("Green Value: {val}");
            colors.sort_by(|x, y| x.green.cmp(&y.green));
            if colors.len().is_multiple_of(2) {
                median = (colors[median_idx - 1].green + colors[median_idx].green) / 2;
            } else {
                median = colors[median_idx].green;
            }
            for color in colors {
                if color.green >= median {
                    upper_values.push(color);
                } else {
                    lower_values.push(color);
                }
            }
        }
        RGB::Blue(val) => {
            println!("Blue Value: {val}");
            colors.sort_by(|x, y| x.blue.cmp(&y.blue));
            if colors.len().is_multiple_of(2) {
                median = (colors[median_idx - 1].blue + colors[median_idx].blue) / 2;
            } else {
                median = colors[median_idx].blue;
            }
            for color in colors {
                if color.blue >= median {
                    upper_values.push(color);
                } else {
                    lower_values.push(color);
                }
            }
        }
    }

    let mut map: HashMap<String, Vec<RGBColor>> = HashMap::new();

    map.insert(format(format_args!("Cube{}", idx)), upper_values);
    map.insert(format(format_args!("Cube{}", idx + 1)), lower_values);
    map
}

/// Takes in a vector of colors, and returns a color with the max range of each value for the cube
fn find_color_range(cube: &[RGBColor]) -> RGB {
    // Iterate over the cube to find the min and max values for each channel
    let r_max = cube.iter().max_by(|x, y| x.red.cmp(&y.red)).unwrap();
    let r_min = cube.iter().min_by(|x, y| x.red.cmp(&y.red)).unwrap();
    let r_range = r_max.red - r_min.red;

    let g_max = cube.iter().max_by(|x, y| x.green.cmp(&y.green)).unwrap();
    let g_min = cube.iter().min_by(|x, y| x.green.cmp(&y.green)).unwrap();
    let g_range = g_max.green - g_min.green;

    let b_max = cube.iter().max_by(|x, y| x.blue.cmp(&y.blue)).unwrap();
    let b_min = cube.iter().min_by(|x, y| x.blue.cmp(&y.blue)).unwrap();
    let b_range = b_max.blue - b_min.blue;

    println!(
        "Red: {} -- Green: {} -- Blue: {}",
        r_range, g_range, b_range
    );
    if r_range >= g_range && r_range >= b_range {
        RGB::Red(r_range)
    } else if g_range >= r_range && g_range >= b_range {
        RGB::Green(g_range)
    } else {
        RGB::Blue(b_range)
    }
}

fn build_image_path() -> Option<PathBuf> {
    // Get the commands
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::value_parser;

    // NOTE:
    // While this was writing tests first, it wasn't TDD style. This was "forcing" the tests to
    // work in the intended way then writing the code logic to mirror this. We want to write a test
    // that fails first, then implement the function code that will make it pass. We would call the
    // function inside the test to make sure it works the way we want. Going forward I will be
    // working like that -- These first few might eventually get removed or refactored but for now
    // they're an example

    // Check to see if no command line args are passed in
    #[test]
    fn get_command_line_arg_path_none() {
        // Path variable
        let matches = Command::new("myapp")
            .arg(Arg::new("image").short('i').long("image"))
            .get_matches();

        let path = matches.get_one::<String>("image");
        assert_eq!(path, None);
    }

    // Check we have command line path -> hard coded relative path
    #[test]
    fn get_command_line_arg_path_some() {
        // Path variable
        let matches = Command::new("myapp")
            .arg(
                Arg::new("image")
                    .short('i')
                    .long("image")
                    .value_parser(value_parser!(String))
                    .default_value("./images/nessa.jpg"),
            )
            .get_matches();

        let path = matches.get_one::<String>("image");
        assert_eq!(path, Some(&"./images/nessa.jpg".to_string()));
    }

    // Another check if path is relative, this time from command line
    #[test]
    fn command_line_arg_path_is_relative() {
        let matches = Command::new("myapp")
            .arg(
                Arg::new("image")
                    .short('i')
                    .long("image")
                    .value_parser(value_parser!(String))
                    .default_value("./images/nessa.jpg"),
            )
            .get_matches();

        let path_string = matches.get_one::<String>("image");
        let path = PathBuf::from(path_string.unwrap());
        assert!(path.is_relative());
    }

    // Test if a path is relative. Makes it easier to use an absolute path
    #[test]
    fn return_with_please_use_absolute_path() {
        let path = PathBuf::from("./src/images/nessa.jpg");
        assert!(path.is_relative(), "Please use absolute path");
    }

    // Load the image from path
    #[test]
    fn load_image_from_path() {
        let path = PathBuf::from("C:/Dev/rust/image_testing/src/images/nessa.jpg");
        let image = image::open(path.as_path());
        assert!(image.is_ok(), "Image did not load successfully");
    }

    // Just determine that we have managed to get pixel colors into a vector here
    #[test]
    fn get_rgb_pixels_from_image() {
        let path = PathBuf::from("C:/Dev/rust/image_testing/src/images/nessa.jpg");

        assert!(get_pixels_from_image(path.as_path()).len() > 0);
    }
}
