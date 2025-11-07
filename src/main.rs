mod algorithms;
mod utils;

use clap::{Arg, Command};
use std::path::PathBuf;

fn main() {
    let _ = build_command_matches();
}

fn build_command_matches() -> Option<PathBuf> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::value_parser;

    #[test]
    fn get_command_line_arg_path_none() {
        // Path variable
        let matches = Command::new("myapp")
            .arg(Arg::new("image").short('i').long("image"))
            .get_matches();

        let path = matches.get_one::<String>("image");
        assert_eq!(path, None);
    }
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

    #[test]
    fn return_with_please_use_absolute_path() {
        let path = PathBuf::from("./src/images/nessa.jpg");
        assert!(path.is_relative(), "Please use absolute path");
    }

    #[test]
    fn load_image_from_path() {
        let path = PathBuf::from("C:/Dev/rust/image_testing/src/images/nessa.jpg");
        let image = image::open(path.as_path());
        assert!(image.is_ok(), "Image did not load successfully");
    }

    #[test]
    fn get_rgb_pixels_from_image() {
        let path = PathBuf::from("C:/Dev/rust/image_testing/src/images/nessa.jpg");
        let image = image::open(path.as_path());

        if let Ok(image) = image {
            let colored_image = image.into_rgb16();
            let element = colored_image.pixels().next();

            assert!(element.is_some(), "Failed to get rgb pixels from image");
            let x = element.unwrap();
        };
    }
}
