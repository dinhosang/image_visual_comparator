use std::process;

use image_visual_comparator::{run, Config};

fn main() {
    let tolerance = 5_f32;
    let image_path_one = "./images/image_one.png";
    let image_path_two = "./images/image_two.png";

    let config = Config::build(tolerance, image_path_one, image_path_two);

    match run(config) {
        Ok(mismatched_pixels) => println!("Mismatched Pixels: {:#?}", mismatched_pixels),
        Err(message) => {
            println!("{message}");
            process::exit(1)
        }
    }
}
