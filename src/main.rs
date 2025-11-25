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
    Red,
    Green,
    Blue,
}

// default palette size
const DEFAULT_PALETTE_SIZE: u8 = 6;

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
                println!("Range two: {range:?}");

                median_split.extend(find_median(&range, cube.clone(), i));
            }
        }
        println!("Median Split: {:?}", median_split.keys());
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

    match color_to_cut {
        RGB::Red => {
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
        RGB::Green => {
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
        RGB::Blue => {
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

    if r_range >= g_range && r_range >= b_range {
        RGB::Red
    } else if g_range >= r_range && g_range >= b_range {
        RGB::Green
    } else {
        RGB::Blue
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
