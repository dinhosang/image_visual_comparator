use image::DynamicImage;

use std::path::Path;

fn get_image_from_file_location(image_location: &str) -> Result<DynamicImage, String> {
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
    let image_one = get_image_from_file_location(image_location_one)?;
    let image_two = get_image_from_file_location(image_location_two)?;

    Ok((image_one, image_two))
}

#[cfg(test)]
mod tests {
    mod returns_error {
        use crate::{
            test_utils::{
                files::{create_temp_dir_handler, get_image_locations},
                image::create_dynamic_image,
            },
            utils::files::get_pair_of_images_from_file_locations,
        };

        #[test]
        fn when_neither_image_exists() {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let result = get_pair_of_images_from_file_locations(
                image_one_location.as_str(),
                image_two_location.as_str(),
            );

            let expected_err_msg = format!(
                "ERROR: when trying to open '{image_one_location}'. Message: 'No such file or directory (os error 2)'"
            );

            assert_eq!(Err(expected_err_msg), result);
        }

        #[test]
        fn when_image_one_does_not_exist() {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let _ = create_dynamic_image(5, 5).save(&image_two_location);

            let result = get_pair_of_images_from_file_locations(
                image_one_location.as_str(),
                image_two_location.as_str(),
            );

            let expected_err_msg = format!(
                "ERROR: when trying to open '{image_one_location}'. Message: 'No such file or directory (os error 2)'"
            );

            assert_eq!(Err(expected_err_msg), result);
        }

        #[test]
        fn when_image_two_does_not_exist() {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let _ = create_dynamic_image(5, 5).save(&image_one_location);

            let result = get_pair_of_images_from_file_locations(
                image_one_location.as_str(),
                image_two_location.as_str(),
            );

            let expected_err_msg = format!(
                "ERROR: when trying to open '{image_two_location}'. Message: 'No such file or directory (os error 2)'"
            );

            assert_eq!(Err(expected_err_msg), result);
        }
    }

    mod returns_images {
        use crate::{
            test_utils::{
                files::{create_temp_dir_handler, get_image_locations},
                image::{change_pixel_on_img, create_dynamic_image},
            },
            utils::files::get_pair_of_images_from_file_locations,
        };

        #[test]
        fn when_images_exist_and_match_dimensions_and_match_content() {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let image_one = create_dynamic_image(5, 5);
            let image_two = create_dynamic_image(5, 5);

            let _ = image_one.save(&image_one_location);
            let _ = image_two.save(&image_two_location);

            let result = get_pair_of_images_from_file_locations(
                image_one_location.as_str(),
                image_two_location.as_str(),
            );

            assert_eq!(Ok((image_one, image_two)), result);
        }

        #[test]
        fn when_images_exist_and_match_dimensions_but_do_not_match_content() {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let image_one = create_dynamic_image(5, 5);
            let mut image_two = create_dynamic_image(5, 5);
            change_pixel_on_img(&mut image_two, 3, 3);

            let _ = image_one.save(&image_one_location);
            let _ = image_two.save(&image_two_location);

            let result = get_pair_of_images_from_file_locations(
                image_one_location.as_str(),
                image_two_location.as_str(),
            );

            assert_eq!(Ok((image_one, image_two)), result);
        }
    }
}
