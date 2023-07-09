use image::{DynamicImage, GenericImageView};
use lab::Lab;

pub fn get_lab_colour_for_img_pixel(img: &DynamicImage, x: u32, y: u32) -> Lab {
    let pixel = img.get_pixel(x, y);
    let rgba_value = pixel.0;
    Lab::from_rgba(&rgba_value)
}
