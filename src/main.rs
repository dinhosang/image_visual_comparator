use image::{self, DynamicImage, GenericImageView};
use lab::Lab;

fn main() {
    const TOLERANCE_PERCENTAGE: f32 = 5_f32;

    let img = image::open("./images/image_one.png").expect("file not found");
    let img_two = image::open("./images/image_two.png").expect("file not found");

    let (width, height) = img.dimensions();

    let mut mismatched_pixels: Vec<(u32, u32)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let lab_colour: Lab = get_lab_colour_for_img_pixel(&img, x, y);
            let lab_colour_two: Lab = get_lab_colour_for_img_pixel(&img_two, x, y);
            let difference: f32 = lab_colour.squared_distance(&lab_colour_two);
            if difference > TOLERANCE_PERCENTAGE {
                mismatched_pixels.push((x, y));
            }
        }
    }
    println!("Mismatched Pixels: {:#?}", mismatched_pixels);
}

fn get_lab_colour_for_img_pixel(img: &DynamicImage, x: u32, y: u32) -> Lab {
    let pixel = img.get_pixel(x, y);
    let rgba_value = pixel.0;
    Lab::from_rgba(&rgba_value)
}
