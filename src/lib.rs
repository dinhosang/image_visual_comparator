use std::path::Path;

use image::{self, DynamicImage, GenericImageView};
use lab::Lab;

pub struct Config<'a> {
    tolerance: f32,
    image_paths: (&'a str, &'a str),
}

impl Config<'_> {
    pub fn build<'a>(
        tolerance: f32,
        image_path_one: &'a str,
        image_path_two: &'a str,
    ) -> Config<'a> {
        Config {
            tolerance,
            image_paths: (image_path_one, image_path_two),
        }
    }
}

pub fn run(config: Config) -> Result<Vec<(u32, u32)>, String> {
    let images = ImagesHolder::new(config.image_paths.0, config.image_paths.1)?;

    let (width, height) = images.one.dimensions();

    let mut mismatched_pixels: Vec<(u32, u32)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = ImagePixel { x, y };
            let is_matching = is_pixel_for_images_matching(&images, pixel, config.tolerance);
            if !is_matching {
                mismatched_pixels.push((x, y));
            }
        }
    }

    Ok(mismatched_pixels)
}

struct ImagePixel {
    x: u32,
    y: u32,
}

struct ImagesHolder {
    one: DynamicImage,
    two: DynamicImage,
}

impl ImagesHolder {
    fn new<'a>(
        image_one_location: &'a str,
        image_two_location: &'a str,
    ) -> Result<ImagesHolder, String> {
        let image_one = get_image_from_file_path(image_one_location)?;
        let image_two = get_image_from_file_path(image_two_location)?;

        if !is_dimension_matching_for_images(&image_one, &image_two) {
            return Err(format!("ERROR: when comparing '{image_one_location}' and '{image_two_location}'. Message: image dimensions do not match"));
        }

        Ok(ImagesHolder {
            one: image_one,
            two: image_two,
        })
    }
}

fn is_dimension_matching_for_images(image_one: &DynamicImage, image_two: &DynamicImage) -> bool {
    image_one.height() == image_two.height() && image_one.width() == image_two.width()
}

fn get_image_from_file_path(image_location: &str) -> Result<DynamicImage, String> {
    let path = Path::new(image_location);

    match image::open(path) {
        Ok(img) => Ok(img),
        Err(error) => {
            let original_message = error.to_string();
            let message = format!(
                "ERROR: when trying to open '{image_location}'. Message: '{original_message}'"
            );
            Err(message)
        }
    }
}

fn is_pixel_for_images_matching(images: &ImagesHolder, pixel: ImagePixel, tolerance: f32) -> bool {
    let lab_colour: Lab = get_lab_colour_for_img_pixel(&images.one, pixel.x, pixel.y);
    let lab_colour_two: Lab = get_lab_colour_for_img_pixel(&images.two, pixel.x, pixel.y);
    let difference: f32 = lab_colour.squared_distance(&lab_colour_two);

    difference <= tolerance
}

fn get_lab_colour_for_img_pixel(img: &DynamicImage, x: u32, y: u32) -> Lab {
    let pixel = img.get_pixel(x, y);
    let rgba_value = pixel.0;
    Lab::from_rgba(&rgba_value)
}
