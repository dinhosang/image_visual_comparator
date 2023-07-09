use image::DynamicImage;

pub fn is_dimension_matching_for_images(
    image_one: &DynamicImage,
    image_two: &DynamicImage,
) -> bool {
    image_one.height() == image_two.height() && image_one.width() == image_two.width()
}
