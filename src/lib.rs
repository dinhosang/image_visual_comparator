use compare::compare_pair_of_images;
use errors::IVCError;
use models::{ImageHolder, PixelCoord};
use utils::files::get_pair_of_images_from_file_locations;

mod compare;
mod errors;
mod models;
mod utils;

mod test_utils;

pub fn run() -> Result<Vec<PixelCoord>, IVCError> {
    // TODO: temp hard-coding
    let pixel_tolerance = 5_f32;
    let image_location_one = "./images/image_one.png";
    let image_location_two = "./images/image_two.png";

    handle_pair_of_images(image_location_one, image_location_two, pixel_tolerance)
}

fn handle_pair_of_images(
    image_location_one: &str,
    image_location_two: &str,
    pixel_tolerance: f32,
) -> Result<Vec<PixelCoord>, IVCError> {
    let images: (ImageHolder, ImageHolder) =
        get_pair_of_images_from_file_locations(image_location_one, image_location_two)?;

    Ok(compare_pair_of_images(&images, pixel_tolerance)?)
}
