use std::process;

use image_visual_comparator::run;

fn main() {
    // TODO: some kind of input or config reading to determine locations to looks for images
    match run() {
        Ok(mismatched_pixels) => println!("Mismatched Pixels: {:#?}", mismatched_pixels),
        Err(message) => {
            println!("{message}");
            process::exit(1)
        }
    }
}
