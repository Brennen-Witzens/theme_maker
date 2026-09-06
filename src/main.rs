mod algorithms;
mod utils;

use clap::{Arg, ArgAction, Command};
use image::{GenericImageView, RgbImage};
use std::{
    collections::{btree_map::Keys, HashMap},
    fmt::format,
    io::Error,
    path::{Path, PathBuf},
};

const ESCAPE_KEY: &str = "\u{001b}[";
const RESET_KEY: &str = "\u{001b}[m";

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

#[derive(Clone, Debug)]
enum ColorExtractionType {
    Mean,
    Median,
    Mode,
}

// TODO: Put both of these in an app config struct and have that be the default
// Default palette size
const DEFAULT_PALETTE_SIZE: u8 = 6;
// default color extraction type
const DEFAULT_COLOR_EXTRACTION_METHOD: ColorExtractionType = ColorExtractionType::Mode;

struct App {
    palette_size: u8,
    color_extraction_method: ColorExtractionType,
    image_path: PathBuf,
    // Export Path?
}

#[derive(Clone, Default, Debug)]
struct AppBuilder {
    palette_size: Option<u8>,
    color_extraction_method: Option<ColorExtractionType>,
    image_path: Option<PathBuf>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_image_path(mut self, image_path: &str) -> Self {
        self.image_path = Some(PathBuf::from(image_path));
        self
    }

    pub fn build(self) -> Result<App, std::io::Error> {
        let palette_size = self.palette_size.unwrap_or_else(|| DEFAULT_PALETTE_SIZE);

        let color_extraction_method = self
            .color_extraction_method
            .unwrap_or_else(|| ColorExtractionType::Mean);

        let image_path = self.image_path.unwrap();
        if image_path.is_relative() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Please enter an absolute path",
            ));
        }

        Ok(App {
            palette_size,
            color_extraction_method,
            image_path,
        })
    }
}

fn main() {
    let cmd = setup_app_command(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let arg_matches = cmd.get_matches();

    let image_path = arg_matches.get_one("image").map(|v: &String| v.to_string());

    let app_builder = AppBuilder::new()
        .with_image_path(&image_path.unwrap_or_default())
        .build();

    let app = app_builder.unwrap();

    let image = image::open(app.image_path.as_path()).unwrap();
    let resized_image = resize_image(&image.to(), 300);
    let sample_pixels = get_sample_pixels_from_image(&resized_image, 2);

    let centroids = kmeans(&sample_pixels, 6, 8);

    let mut colors: Vec<RGBColor> = centroids
        .into_iter()
        .map(|x| RGBColor::build_color(x[0] as u8, x[1] as u8, x[2] as u8))
        .collect();

    for color in colors {
        println!(
            "\u{001b}[48;2;{};{};{}m    \u{001b}[m",
            color.red, color.green, color.blue
        );
    }
}

fn resize_image(image: &RgbImage, target_width: u32) -> RgbImage {
    let (w, h) = image.dimensions();

    if w <= target_width {
        return image.clone();
    }

    let scale = target_width as f32 / w as f32;
    let target_height = (h as f32 * scale).round() as u32;

    image::imageops::resize(
        image,
        target_width,
        target_height,
        image::imageops::FilterType::Triangle,
    )
}

fn kmeans(samples: &[[f32; 3]], clusters: usize, iterations: usize) -> Vec<[f32; 3]> {
    let mut centroids: Vec<[f32; 3]> = samples.iter().take(clusters).cloned().collect();

    for _ in 0..iterations {
        let mut buckets: Vec<Vec<[f32; 3]>> = vec![Vec::new(); clusters];

        for sample in samples {
            let mut best = 0;
            let mut best_dist = f32::MAX;

            for (i, centroid) in centroids.iter().enumerate() {
                let d = (sample[0] - centroid[0]).powi(2)
                    + (sample[1] - centroid[1]).powi(2)
                    + (sample[2] - centroid[2]).powi(2);

                if d < best_dist {
                    best = i;
                    best_dist = d;
                }
            }

            buckets[best].push(*sample);
        }

        for (i, bucket) in buckets.iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }

            let mut sum = [0.0; 3];
            for k in bucket {
                sum[0] += k[0];
                sum[1] += k[1];
                sum[2] += k[2];
            }

            centroids[i] = [
                sum[0] / bucket.len() as f32,
                sum[1] / bucket.len() as f32,
                sum[2] / bucket.len() as f32,
            ];
        }
    }

    centroids
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

// fn find_median(
//     color_to_cut: &RGB,
//     mut colors: Vec<RGBColor>,
//     idx: u8,
// ) -> HashMap<String, Vec<RGBColor>> {
//     let mut upper_values: Vec<RGBColor> = Vec::new();
//     let mut lower_values: Vec<RGBColor> = Vec::new();
//     let median: u8;
//     let median_idx = colors.len() / 2;

//     println!("Color to cut: {color_to_cut:?}");

//     match color_to_cut {
//         RGB::Red(val) => {
//             println!("Red Value: {val}");
//             colors.sort_by(|x, y| x.red.cmp(&y.red));
//             if colors.len().is_multiple_of(2) {
//                 median = (colors[median_idx - 1].red + colors[median_idx].red) / 2;
//             } else {
//                 median = colors[median_idx].red;
//             }
//             for color in colors {
//                 if color.red >= median {
//                     upper_values.push(color);
//                 } else {
//                     lower_values.push(color);
//                 }
//             }
//         }
//         RGB::Green(val) => {
//             println!("Green Value: {val}");
//             colors.sort_by(|x, y| x.green.cmp(&y.green));
//             if colors.len().is_multiple_of(2) {
//                 median = (colors[median_idx - 1].green + colors[median_idx].green) / 2;
//             } else {
//                 median = colors[median_idx].green;
//             }
//             for color in colors {
//                 if color.green >= median {
//                     upper_values.push(color);
//                 } else {
//                     lower_values.push(color);
//                 }
//             }
//         }
//         RGB::Blue(val) => {
//             println!("Blue Value: {val}");
//             colors.sort_by(|x, y| x.blue.cmp(&y.blue));
//             if colors.len().is_multiple_of(2) {
//                 median = (colors[median_idx - 1].blue + colors[median_idx].blue) / 2;
//             } else {
//                 median = colors[median_idx].blue;
//             }
//             for color in colors {
//                 if color.blue >= median {
//                     upper_values.push(color);
//                 } else {
//                     lower_values.push(color);
//                 }
//             }
//         }
//     }

//     let mut map: HashMap<String, Vec<RGBColor>> = HashMap::new();

//     map.insert(format(format_args!("Cube{}", idx)), upper_values);
//     map.insert(format(format_args!("Cube{}", idx + 1)), lower_values);
//     map
// }

// /// Takes in a vector of colors, and returns a color with the max range of each value for the cube
// fn find_color_range(cube: &[RGBColor]) -> RGB {
//     // Iterate over the cube to find the min and max values for each channel
//     let r_max = cube.iter().max_by(|x, y| x.red.cmp(&y.red)).unwrap();
//     let r_min = cube.iter().min_by(|x, y| x.red.cmp(&y.red)).unwrap();
//     let r_range = r_max.red - r_min.red;

//     let g_max = cube.iter().max_by(|x, y| x.green.cmp(&y.green)).unwrap();
//     let g_min = cube.iter().min_by(|x, y| x.green.cmp(&y.green)).unwrap();
//     let g_range = g_max.green - g_min.green;

//     let b_max = cube.iter().max_by(|x, y| x.blue.cmp(&y.blue)).unwrap();
//     let b_min = cube.iter().min_by(|x, y| x.blue.cmp(&y.blue)).unwrap();
//     let b_range = b_max.blue - b_min.blue;

//     println!(
//         "Red: {} -- Green: {} -- Blue: {}",
//         r_range, g_range, b_range
//     );
//     if r_range >= g_range && r_range >= b_range {
//         RGB::Red(r_range)
//     } else if g_range >= r_range && g_range >= b_range {
//         RGB::Green(g_range)
//     } else {
//         RGB::Blue(b_range)
//     }
// }

// TODO: Add Palette size, Execution Method, and Web Request style commands
fn setup_app_command(app_name: &'static str, app_version: &'static str) -> clap::Command {
    clap::Command::new(app_name).version(app_version).arg(
        Arg::new("image")
            .long("image")
            .help("The image path that you want to create a theme for")
            .short('i')
            .action(ArgAction::Set),
    )
}

fn get_sample_pixels_from_image(image: &RgbImage, step: u32) -> Vec<[f32; 3]> {
    let mut colors = Vec::new();

    for element in image.pixels().step_by(step as usize) {
        colors.push([
            element.0[0] as f32,
            element.0[1] as f32,
            element.0[2] as f32,
        ]);
    }
    colors
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::value_parser;

    // Look at TDD again soon to better understand it. Write notes
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
