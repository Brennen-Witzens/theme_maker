mod algorithms;
mod utils;

use clap::{Arg, Command};
use image::GenericImageView;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, PartialOrd)]
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

fn main() {
    let image_path = build_image_path();

    if let Some(path) = image_path {
        let colors = get_pixels_from_image(&path);

        // Take the colors from the image and calculate the range of each component
        let range = find_color_range(&colors);
        println!("Range: {range:?}");

        // Once we have the range, we need to split the values on the largest component and then
        // find the median value and split the cubes to upper and lower values.
        // NOTE: might be worth using a map for this
    }
}

/// Takes in a vector of colors, and returns a color with the max range of each value for the cube
fn find_color_range(cube: &Vec<RGBColor>) -> RGBColor {
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

    let color_range = RGBColor::build_color(r_range, g_range, b_range);
    return color_range;
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
        return Some(path);
    } else {
        println!("Nothing was passed in");
        return None;
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
