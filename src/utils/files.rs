use image::DynamicImage;

use std::path::Path;

use super::validation::is_dimension_matching_for_images;

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

pub fn get_pair_of_images_from_file_locations(
    image_location_one: &str,
    image_location_two: &str,
) -> Result<(DynamicImage, DynamicImage), String> {
    let image_one = get_image_from_file_path(image_location_one)?;
    let image_two = get_image_from_file_path(image_location_two)?;

    if !is_dimension_matching_for_images(&image_one, &image_two) {
        return Err(format!("ERROR: when comparing '{image_location_one}' and '{image_location_two}'. Message: image dimensions do not match"));
    }

    Ok((image_one, image_two))
}
