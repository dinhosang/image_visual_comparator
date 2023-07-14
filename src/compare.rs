use image::GenericImageView;

use crate::{
    models::PixelCoord,
    utils::{
        files::get_pair_of_images_from_file_locations,
        validation::{are_dimensions_matching_for_images, is_pixel_for_images_matching},
    },
};

// TODO: could do with a doc comment ?
pub fn compare_pair_of_images(
    image_location_one: &str,
    image_location_two: &str,
    pixel_tolerance: f32,
) -> Result<Vec<PixelCoord>, String> {
    let images = get_pair_of_images_from_file_locations(image_location_one, image_location_two)?;

    if !are_dimensions_matching_for_images(&images) {
        return Err(format!("ERROR: when comparing '{image_location_one}' and '{image_location_two}'. Message: 'image dimensions do not match'"));
    }

    let (width, height) = images.0.dimensions();

    let mut mismatched_pixels: Vec<PixelCoord> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel_coord = PixelCoord::new(x, y);
            let is_matching = is_pixel_for_images_matching(&images, &pixel_coord, pixel_tolerance);
            if !is_matching {
                mismatched_pixels.push(pixel_coord);
            }
        }
    }

    Ok(mismatched_pixels)
}

#[cfg(test)]
mod tests {
    mod returns_error {
        use crate::{
            compare::compare_pair_of_images,
            test_utils::{
                files::{create_temp_dir_handler, get_image_locations},
                image::create_dynamic_image,
            },
        };

        #[test]
        fn when_an_image_is_missing_in_file_system() {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let image_one = create_dynamic_image(5, 5);
            let _ = image_one.save(&image_one_location);

            let result = compare_pair_of_images(&image_one_location, &image_two_location, 5_f32);
            let expected = Err(format!("ERROR: when trying to open '{}'. Message: 'No such file or directory (os error 2)'", image_two_location));

            assert_eq!(expected, result);
        }

        #[test]
        fn when_images_do_not_have_matching_dimensions() {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let image_one = create_dynamic_image(5, 5);
            let image_two = create_dynamic_image(4, 5);

            let _ = image_one.save(&image_one_location);
            let _ = image_two.save(&image_two_location);

            let result = compare_pair_of_images(&image_one_location, &image_two_location, 5_f32);
            let expected = Err(format!(
                "ERROR: when comparing '{}' and '{}'. Message: 'image dimensions do not match'",
                image_one_location, image_two_location
            ));

            assert_eq!(expected, result);
        }
    }

    mod returns_vector {
        use image::{DynamicImage, GenericImage};

        use crate::{
            models::PixelCoord,
            test_utils::{
                files::{create_temp_dir_handler, get_image_locations, TempDirHolder},
                image::create_dynamic_image,
            },
        };

        const PIXEL_COLOUR_WITHIN_TOLERANCE: u8 = 250;
        const PIXEL_COLOUR_OUTSIDE_TOLERANCE: u8 = 249;

        fn setup_and_return_required_data() -> (
            TempDirHolder,
            String,
            String,
            DynamicImage,
            DynamicImage,
            PixelCoord,
            PixelCoord,
            PixelCoord,
        ) {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let image_one = create_dynamic_image(5, 5);
            let image_two = create_dynamic_image(5, 5);

            let pixel_coord_one = PixelCoord::new(4_u32, 0_u32);
            let pixel_coord_two = PixelCoord::new(1_u32, 2_u32);
            let pixel_coord_three = PixelCoord::new(3_u32, 4_u32);

            (
                temp_dir_holder,
                image_one_location,
                image_two_location,
                image_one,
                image_two,
                pixel_coord_one,
                pixel_coord_two,
                pixel_coord_three,
            )
        }

        fn update_image_for_pixels<const N: usize>(
            image: &mut DynamicImage,
            mut pixels: [(u32, u32, image::Rgba<u8>, u8); N],
        ) {
            pixels.iter_mut().for_each(|(x, y, pixel, colour_value)| {
                pixel[2] = *colour_value;
                image.put_pixel(*x, *y, *pixel)
            });
        }

        mod with_no_pixel_coords {
            use image::GenericImageView;

            use crate::{
                compare::{
                    compare_pair_of_images,
                    tests::returns_vector::{
                        setup_and_return_required_data, update_image_for_pixels,
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    },
                },
                test_utils::{
                    files::{create_temp_dir_handler, get_image_locations},
                    image::create_dynamic_image,
                },
            };

            #[test]
            fn when_images_are_an_exact_match() {
                let temp_dir_holder = create_temp_dir_handler();
                let (image_one_location, image_two_location) =
                    get_image_locations(&temp_dir_holder);

                let image_one = create_dynamic_image(5, 5);
                let image_two = create_dynamic_image(5, 5);

                let _ = image_one.save(&image_one_location);
                let _ = image_two.save(&image_two_location);

                let result =
                    compare_pair_of_images(&image_one_location, &image_two_location, 5_f32);
                let expected = Ok(vec![]);

                assert_eq!(
                    image_one, image_two,
                    "Images should have matched, but do not"
                );
                assert_eq!(expected, result);
            }

            #[test]
            fn when_image_have_pixel_differences_within_the_chosen_tolerance_for_each_pixel() {
                let (
                    _temp_dir_holder,
                    image_one_location,
                    image_two_location,
                    mut image_one,
                    image_two,
                    pixel_coord_one,
                    pixel_coord_two,
                    pixel_coord_three,
                ) = setup_and_return_required_data();

                let image_one_pixels = [
                    (
                        pixel_coord_one.x,
                        pixel_coord_one.y,
                        image_one.get_pixel(pixel_coord_one.x, pixel_coord_one.y),
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    ),
                    (
                        pixel_coord_two.x,
                        pixel_coord_two.y,
                        image_one.get_pixel(pixel_coord_two.x, pixel_coord_two.y),
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    ),
                    (
                        pixel_coord_three.x,
                        pixel_coord_three.y,
                        image_one.get_pixel(pixel_coord_three.x, pixel_coord_three.y),
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    ),
                ];
                update_image_for_pixels(&mut image_one, image_one_pixels);
                let _ = image_one.save(&image_one_location);
                let _ = image_two.save(&image_two_location);

                let result =
                    compare_pair_of_images(&image_one_location, &image_two_location, 5_f32);
                let expected = Ok(vec![]);

                assert_ne!(
                    image_one, image_two,
                    "Images should NOT have matched, but do"
                );
                assert_eq!(expected, result);
            }
        }

        mod with_pixel_coords {
            use image::GenericImageView;

            use crate::compare::{
                compare_pair_of_images,
                tests::returns_vector::{
                    setup_and_return_required_data, update_image_for_pixels,
                    PIXEL_COLOUR_OUTSIDE_TOLERANCE, PIXEL_COLOUR_WITHIN_TOLERANCE,
                },
            };

            #[test]
            fn when_images_have_pixel_differences_that_breach_the_chosen_tolerance() {
                let (
                    _temp_dir_holder,
                    image_one_location,
                    image_two_location,
                    mut image_one,
                    image_two,
                    pixel_coord_one,
                    pixel_coord_two,
                    pixel_coord_three,
                ) = setup_and_return_required_data();

                let image_one_pixels = [
                    (
                        pixel_coord_one.x,
                        pixel_coord_one.y,
                        image_one.get_pixel(pixel_coord_one.x, pixel_coord_one.y),
                        PIXEL_COLOUR_OUTSIDE_TOLERANCE,
                    ),
                    (
                        2,
                        1,
                        image_one.get_pixel(2_u32, 1_u32),
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    ),
                    (
                        pixel_coord_two.x,
                        pixel_coord_two.y,
                        image_one.get_pixel(pixel_coord_two.x, pixel_coord_two.y),
                        PIXEL_COLOUR_OUTSIDE_TOLERANCE,
                    ),
                    (
                        pixel_coord_three.x,
                        pixel_coord_three.y,
                        image_one.get_pixel(pixel_coord_three.x, pixel_coord_three.y),
                        PIXEL_COLOUR_OUTSIDE_TOLERANCE,
                    ),
                ];
                update_image_for_pixels(&mut image_one, image_one_pixels);
                let _ = image_one.save(&image_one_location);
                let _ = image_two.save(&image_two_location);

                let result =
                    compare_pair_of_images(&image_one_location, &image_two_location, 5_f32);
                let expected = Ok(vec![pixel_coord_one, pixel_coord_two, pixel_coord_three]);

                assert_ne!(
                    image_one, image_two,
                    "Images should NOT have matched, but do"
                );
                assert_eq!(expected, result);
            }
        }
    }
}
