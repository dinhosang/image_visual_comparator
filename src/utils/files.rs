use std::path::Path;

use crate::{errors::IOError, models::ImageHolder};

fn get_image_from_file_location(location: &str) -> Result<ImageHolder, IOError> {
    let path = Path::new(location);

    match image::open(path) {
        Ok(image) => Ok(ImageHolder::new(image, location)),
        Err(error) => Err(IOError::Read {
            location: location.to_string(),
            source: error,
        }),
    }
}

pub fn get_pair_of_images_from_file_locations(
    image_location_one: &str,
    image_location_two: &str,
) -> Result<(ImageHolder, ImageHolder), IOError> {
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

            let expected_message = format!(
                "when reading location: '{}'. Message: 'No such file or directory (os error 2)'",
                image_one_location,
            );

            assert_eq!(expected_message, result.unwrap_err().to_string());
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
                "when reading location: '{}'. Message: 'No such file or directory (os error 2)'",
                image_one_location,
            );

            assert_eq!(expected_err_msg, result.unwrap_err().to_string());
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
                "when reading location: '{}'. Message: 'No such file or directory (os error 2)'",
                image_two_location,
            );

            assert_eq!(expected_err_msg, result.unwrap_err().to_string());
        }
    }

    mod returns_images {
        use crate::{
            models::ImageHolder,
            test_utils::{
                files::{create_temp_dir_handler, get_image_locations},
                image::{change_pixel_on_img, create_dynamic_image},
            },
            utils::files::get_pair_of_images_from_file_locations,
        };

        #[test]
        fn when_images_exist() {
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

            let expected = (
                ImageHolder::new(image_one, &image_one_location),
                ImageHolder::new(image_two, &image_two_location),
            );

            assert_eq!(expected, result.unwrap());
        }

        #[test]
        fn when_images_exist_but_do_not_match_in_content() {
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

            let expected = (
                ImageHolder::new(image_one, &image_one_location),
                ImageHolder::new(image_two, &image_two_location),
            );

            assert_eq!(expected, result.unwrap());
        }
    }
}
