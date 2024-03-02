use lab::Lab;

use crate::models::{ImageHolder, PixelCoord};

use super::colour::get_lab_colour_for_img_pixel;

pub fn are_dimensions_matching_for_images(
    (image_one_holder, image_two_holder): &(ImageHolder, ImageHolder),
) -> bool {
    let image_one = &image_one_holder.image;
    let image_two = &image_two_holder.image;
    image_one.height() == image_two.height() && image_one.width() == image_two.width()
}

pub fn is_pixel_for_images_matching(
    (image_one_holder, image_two_holder): &(ImageHolder, ImageHolder),
    pixel_coord: &PixelCoord,
    tolerance: f32,
) -> bool {
    let lab_colour: Lab = get_lab_colour_for_img_pixel(&image_one_holder.image, pixel_coord);
    let lab_colour_two: Lab = get_lab_colour_for_img_pixel(&image_two_holder.image, pixel_coord);
    let difference: f32 = lab_colour.squared_distance(&lab_colour_two);

    difference <= tolerance
}

#[cfg(test)]
mod tests {
    mod are_dimensions_matching_for_images {
        mod returns_false {
            use crate::{
                test_utils::image::create_dynamic_image,
                utils::image::{
                    are_dimensions_matching_for_images, tests::test_helpers::create_image_holders,
                },
            };

            const EXPECTED_RESULT: bool = false;

            #[test]
            fn when_heights_do_not_match() {
                let image_one = create_dynamic_image(4, 4);
                let image_two = create_dynamic_image(4, 5);

                let input = create_image_holders(image_one, image_two);

                assert_eq!(EXPECTED_RESULT, are_dimensions_matching_for_images(&input))
            }

            #[test]
            fn when_widths_do_not_match() {
                let image_one = create_dynamic_image(4, 4);
                let image_two = create_dynamic_image(3, 4);

                let input = create_image_holders(image_one, image_two);

                assert_eq!(EXPECTED_RESULT, are_dimensions_matching_for_images(&input))
            }
        }

        mod returns_true {
            use crate::{
                test_utils::image::{change_pixel_on_img, create_dynamic_image},
                utils::image::{
                    are_dimensions_matching_for_images, tests::test_helpers::create_image_holders,
                },
            };

            const EXPECTED_RESULT: bool = true;

            #[test]
            fn when_height_and_width_match_but_content_does_not() {
                let image_one = create_dynamic_image(4, 4);
                let mut image_two = create_dynamic_image(4, 4);

                change_pixel_on_img(&mut image_two, 2, 1);

                let input = create_image_holders(image_one, image_two);

                assert_eq!(EXPECTED_RESULT, are_dimensions_matching_for_images(&input))
            }

            #[test]
            fn when_height_and_width_match_and_content_does_as_well() {
                let image_one = create_dynamic_image(4, 4);
                let image_two = create_dynamic_image(4, 4);

                let input = create_image_holders(image_one, image_two);

                assert_eq!(EXPECTED_RESULT, are_dimensions_matching_for_images(&input))
            }
        }
    }

    mod is_pixel_for_images_matching {
        mod returns_false {
            use image::{GenericImage, GenericImageView};
            use lab::Lab;

            use crate::{
                models::PixelCoord,
                test_utils::image::create_dynamic_image,
                utils::image::{
                    is_pixel_for_images_matching, tests::test_helpers::create_image_holders,
                },
            };

            const EXPECTED_RESULT: bool = false;
            const X_OF_PIXEL: u8 = 2;
            const Y_OF_PIXEL: u8 = 1;

            #[test]
            fn when_pixels_differ_past_the_desired_tolerance() {
                const TOLERANCE: f32 = 5_f32;

                let image_one = create_dynamic_image(4, 4);
                let mut image_two = create_dynamic_image(4, 4);
                let pixel_coord = PixelCoord::new(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);

                let image_one_pixel = image_one.get_pixel(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);
                let mut image_two_pixel = image_two.get_pixel(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);

                image_two_pixel[0] = 44;
                image_two.put_pixel(X_OF_PIXEL as u32, Y_OF_PIXEL as u32, image_two_pixel);

                let lab_colour: Lab = Lab::from_rgba(&image_one_pixel.0);
                let lab_colour_two: Lab = Lab::from_rgba(&image_two_pixel.0);
                let difference: f32 = lab_colour.squared_distance(&lab_colour_two);

                let images = create_image_holders(image_one, image_two);

                assert!(difference > TOLERANCE);
                assert_eq!(
                    EXPECTED_RESULT,
                    is_pixel_for_images_matching(&images, &pixel_coord, TOLERANCE)
                );
            }
        }

        mod returns_true {
            use image::{GenericImage, GenericImageView};
            use lab::Lab;

            use crate::{
                models::PixelCoord,
                test_utils::image::create_dynamic_image,
                utils::image::{
                    is_pixel_for_images_matching, tests::test_helpers::create_image_holders,
                },
            };

            const EXPECTED_RESULT: bool = true;
            const X_OF_PIXEL: u8 = 2;
            const Y_OF_PIXEL: u8 = 1;

            #[test]
            fn when_pixels_match_exactly() {
                let image_one = create_dynamic_image(4, 4);
                let image_two = create_dynamic_image(4, 4);
                let pixel_coord = PixelCoord::new(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);

                let image_one_pixel = image_one.get_pixel(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);
                let image_two_pixel = image_two.get_pixel(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);

                let lab_colour: Lab = Lab::from_rgba(&image_one_pixel.0);
                let lab_colour_two: Lab = Lab::from_rgba(&image_two_pixel.0);
                let difference: f32 = lab_colour.squared_distance(&lab_colour_two);

                let images = create_image_holders(image_one, image_two);

                assert_eq!(0_f32, difference);
                assert_eq!(
                    EXPECTED_RESULT,
                    is_pixel_for_images_matching(&images, &pixel_coord, 5_f32)
                )
            }

            #[test]
            fn when_pixels_differ_but_are_within_desired_tolerance() {
                const TOLERANCE: f32 = 8_f32;

                let image_one = create_dynamic_image(4, 4);
                let mut image_two = create_dynamic_image(4, 4);
                let pixel_coord = PixelCoord::new(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);

                let image_one_pixel = image_one.get_pixel(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);
                let mut image_two_pixel = image_two.get_pixel(X_OF_PIXEL as u32, Y_OF_PIXEL as u32);

                image_two_pixel[0] = 49;
                image_two.put_pixel(X_OF_PIXEL as u32, Y_OF_PIXEL as u32, image_two_pixel);

                let lab_colour: Lab = Lab::from_rgba(&image_one_pixel.0);
                let lab_colour_two: Lab = Lab::from_rgba(&image_two_pixel.0);
                let difference: f32 = lab_colour.squared_distance(&lab_colour_two);

                let images = create_image_holders(image_one, image_two);

                assert!(difference < TOLERANCE);
                assert_eq!(
                    EXPECTED_RESULT,
                    is_pixel_for_images_matching(&images, &pixel_coord, TOLERANCE)
                );
            }
        }
    }

    mod test_helpers {
        use image::DynamicImage;

        use crate::models::ImageHolder;

        pub fn create_image_holders(
            image_one: DynamicImage,
            image_two: DynamicImage,
        ) -> (ImageHolder, ImageHolder) {
            (
                ImageHolder::new(image_one, "one.png"),
                ImageHolder::new(image_two, "two.png"),
            )
        }
    }
}
