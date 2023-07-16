use image::GenericImageView;

use crate::{
    errors::DimensionMismatchError,
    models::{ImageHolder, PixelCoord},
    utils::validation::{are_dimensions_matching_for_images, is_pixel_for_images_matching},
};

#[doc(hidden)]
/// Compares two images and returns a vector of pixel co-ordinates of pixels who differ past the desired tolerance.
///
/// The location strings passed in should be relative to the directory the program is being run in.
///
/// The passed in tolerance will be used as a reference of whether two pixels with the same co-ordinates
/// differ between two images. The comparison is performed by converting the pixels to lab colours and checking
/// their squared distance, it is that distance which is checked against the tolerance.
///
/// For instance if an exact match is desired then a value of 0_f32 should be passed in. To allow for more relaxed
/// standards simply use a higher number.
pub fn compare_pair_of_images(
    images: &(ImageHolder, ImageHolder),
    pixel_tolerance: f32,
) -> Result<Vec<PixelCoord>, DimensionMismatchError> {
    if !are_dimensions_matching_for_images(images) {
        return Err(DimensionMismatchError::new(
            images.0.location.to_string(),
            images.1.location.to_string(),
        ));
    }

    let (width, height) = images.0.image.dimensions();

    let mut mismatched_pixels: Vec<PixelCoord> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel_coord = PixelCoord::new(x, y);
            let is_matching = is_pixel_for_images_matching(images, &pixel_coord, pixel_tolerance);
            if !is_matching {
                mismatched_pixels.push(pixel_coord);
            }
        }
    }

    Ok(mismatched_pixels)
}

#[cfg(test)]
mod tests {
    pub mod helpers {
        use image::DynamicImage;

        use crate::models::ImageHolder;

        pub fn create_image_holders(
            image_one: DynamicImage,
            image_one_location: &String,
            image_two: DynamicImage,
            image_two_location: &String,
        ) -> (ImageHolder, ImageHolder) {
            (
                ImageHolder::new(image_one, image_one_location),
                ImageHolder::new(image_two, image_two_location),
            )
        }
    }
    mod returns_error {
        use crate::{
            compare::{compare_pair_of_images, tests::helpers::create_image_holders},
            test_utils::{
                files::{create_temp_dir_handler, get_image_locations},
                image::create_dynamic_image,
            },
        };

        #[test]
        fn when_images_do_not_have_matching_dimensions() {
            let temp_dir_holder = create_temp_dir_handler();
            let (image_one_location, image_two_location) = get_image_locations(&temp_dir_holder);

            let image_one = create_dynamic_image(5, 5);
            let image_two = create_dynamic_image(4, 5);

            let _ = image_one.save(&image_one_location);
            let _ = image_two.save(&image_two_location);

            let images = create_image_holders(
                image_one,
                &image_one_location,
                image_two,
                &image_two_location,
            );

            let result = compare_pair_of_images(&images, 5_f32);
            let expected = format!(
                "dimensions do not match: {} and {}",
                image_one_location, image_two_location
            );

            assert_eq!(expected, result.unwrap_err().to_string());
        }
    }

    mod returns_vector {
        use image::{DynamicImage, GenericImage};

        use crate::{
            models::{ImageHolder, PixelCoord},
            test_utils::{
                files::{create_temp_dir_handler, get_image_locations, TempDirHolder},
                image::create_dynamic_image,
            },
        };

        use super::helpers::create_image_holders;

        const PIXEL_COLOUR_WITHIN_TOLERANCE: u8 = 250;
        const PIXEL_COLOUR_OUTSIDE_TOLERANCE: u8 = 249;

        fn setup_and_return_required_data() -> (
            TempDirHolder,
            (ImageHolder, ImageHolder),
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

            let images = create_image_holders(
                image_one,
                &image_one_location,
                image_two,
                &image_two_location,
            );

            (
                temp_dir_holder,
                images,
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
                    tests::{
                        helpers::create_image_holders,
                        returns_vector::{
                            setup_and_return_required_data, update_image_for_pixels,
                            PIXEL_COLOUR_WITHIN_TOLERANCE,
                        },
                    },
                },
                models::PixelCoord,
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

                let images = create_image_holders(
                    image_one,
                    &image_one_location,
                    image_two,
                    &image_two_location,
                );

                let result = compare_pair_of_images(&images, 5_f32);
                let expected: Vec<PixelCoord> = vec![];

                assert_eq!(
                    images.0.image, images.1.image,
                    "Images should have matched, but do not"
                );
                assert_eq!(expected, result.unwrap());
            }

            #[test]
            fn when_image_have_pixel_differences_within_the_chosen_tolerance_for_each_pixel() {
                let (
                    _temp_dir_holder,
                    mut images,
                    pixel_coord_one,
                    pixel_coord_two,
                    pixel_coord_three,
                ) = setup_and_return_required_data();

                let image_one_pixels = [
                    (
                        pixel_coord_one.x,
                        pixel_coord_one.y,
                        images
                            .0
                            .image
                            .get_pixel(pixel_coord_one.x, pixel_coord_one.y),
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    ),
                    (
                        pixel_coord_two.x,
                        pixel_coord_two.y,
                        images
                            .0
                            .image
                            .get_pixel(pixel_coord_two.x, pixel_coord_two.y),
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    ),
                    (
                        pixel_coord_three.x,
                        pixel_coord_three.y,
                        images
                            .0
                            .image
                            .get_pixel(pixel_coord_three.x, pixel_coord_three.y),
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    ),
                ];

                update_image_for_pixels(&mut images.0.image, image_one_pixels);
                let _ = images.0.image.save(&images.0.location);
                let _ = images.1.image.save(&images.1.location);

                let result = compare_pair_of_images(&images, 5_f32);
                let expected: Vec<PixelCoord> = vec![];

                assert_ne!(
                    images.0.image, images.1.image,
                    "Images should NOT have matched, but do"
                );
                assert_eq!(expected, result.unwrap());
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
                    mut images,
                    pixel_coord_one,
                    pixel_coord_two,
                    pixel_coord_three,
                ) = setup_and_return_required_data();

                let image_one_pixels = [
                    (
                        pixel_coord_one.x,
                        pixel_coord_one.y,
                        images
                            .0
                            .image
                            .get_pixel(pixel_coord_one.x, pixel_coord_one.y),
                        PIXEL_COLOUR_OUTSIDE_TOLERANCE,
                    ),
                    (
                        2,
                        1,
                        images.0.image.get_pixel(2_u32, 1_u32),
                        PIXEL_COLOUR_WITHIN_TOLERANCE,
                    ),
                    (
                        pixel_coord_two.x,
                        pixel_coord_two.y,
                        images
                            .0
                            .image
                            .get_pixel(pixel_coord_two.x, pixel_coord_two.y),
                        PIXEL_COLOUR_OUTSIDE_TOLERANCE,
                    ),
                    (
                        pixel_coord_three.x,
                        pixel_coord_three.y,
                        images
                            .0
                            .image
                            .get_pixel(pixel_coord_three.x, pixel_coord_three.y),
                        PIXEL_COLOUR_OUTSIDE_TOLERANCE,
                    ),
                ];
                update_image_for_pixels(&mut images.0.image, image_one_pixels);
                let _ = images.0.image.save(&images.0.location);
                let _ = images.1.image.save(&images.1.location);

                let result = compare_pair_of_images(&images, 5_f32);
                let expected = vec![pixel_coord_one, pixel_coord_two, pixel_coord_three];

                assert_ne!(
                    images.0.image, images.1.image,
                    "Images should NOT have matched, but do"
                );
                assert_eq!(expected, result.unwrap());
            }
        }
    }
}
