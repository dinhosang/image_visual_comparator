use compare::compare_pair_of_images;

mod compare;
mod models;
mod utils;

pub fn run() -> Result<Vec<(u32, u32)>, String> {
    // TODO: temp hard-coding
    let pixel_tolerance = 5_f32;
    let image_location_one = "./images/image_one.png";
    let image_location_two = "./images/image_two.png";

    compare_pair_of_images(image_location_one, image_location_two, pixel_tolerance)
}
