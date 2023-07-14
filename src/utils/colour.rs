use image::{DynamicImage, GenericImageView};
use lab::Lab;

use crate::models::PixelCoord;

pub fn get_lab_colour_for_img_pixel(img: &DynamicImage, pixel_coord: &PixelCoord) -> Lab {
    let pixel = img.get_pixel(pixel_coord.x, pixel_coord.y);
    let rgba_value = pixel.0;
    Lab::from_rgba(&rgba_value)
}

#[cfg(test)]
mod tests {
    use image::GenericImageView;
    use lab::Lab;

    use crate::{
        models::PixelCoord, test_utils::image::create_dynamic_image,
        utils::colour::get_lab_colour_for_img_pixel,
    };

    #[test]
    fn lab_colour_is_accurately_returned() {
        const X: u32 = 3;
        const Y: u32 = 3;

        let img = create_dynamic_image(5, 5);
        let pixel = img.get_pixel(X, Y);
        let actual = Lab::from_rgba(&pixel.0);

        let expected = get_lab_colour_for_img_pixel(&img, &PixelCoord::new(X, Y));

        assert_eq!(
            actual, expected,
            "actual lab colour '{:?}', does NOT match expected lab colour '{:?}'",
            actual, expected
        );
    }
}
