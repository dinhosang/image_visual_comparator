use image::{DynamicImage, GenericImageView};
use lab::Lab;

use crate::{
    models::ImagePixel,
    utils::{colour::get_lab_colour_for_img_pixel, files::get_pair_of_images_from_file_locations},
};

pub fn compare_pair_of_images(
    image_location_one: &str,
    image_location_two: &str,
    pixel_tolerance: f32,
) -> Result<Vec<(u32, u32)>, String> {
    let images = get_pair_of_images_from_file_locations(image_location_one, image_location_two)?;

    let (width, height) = images.0.dimensions();

    let mut mismatched_pixels: Vec<(u32, u32)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = ImagePixel { x, y };
            let is_matching = is_pixel_for_images_matching(&images, pixel, pixel_tolerance);
            if !is_matching {
                mismatched_pixels.push((x, y));
            }
        }
    }

    Ok(mismatched_pixels)
}

fn is_pixel_for_images_matching(
    images: &(DynamicImage, DynamicImage),
    pixel: ImagePixel,
    tolerance: f32,
) -> bool {
    let lab_colour: Lab = get_lab_colour_for_img_pixel(&images.0, pixel.x, pixel.y);
    let lab_colour_two: Lab = get_lab_colour_for_img_pixel(&images.1, pixel.x, pixel.y);
    let difference: f32 = lab_colour.squared_distance(&lab_colour_two);

    difference <= tolerance
}
