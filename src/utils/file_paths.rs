use crate::{
    config::AppConfig,
    errors::{handling::create_image_not_paired_error, ivc::IVCError},
};

use self::helpers::{are_file_path_pairs_valid, get_pairs_of_file_paths_for_images};

pub fn get_file_path_pairs_if_valid(
    config: &AppConfig,
    original_paths: Vec<String>,
    latest_paths: Vec<String>,
) -> Result<Vec<(String, String)>, IVCError> {
    let image_pairs = get_pairs_of_file_paths_for_images(original_paths, latest_paths);

    if are_file_path_pairs_valid(config, &image_pairs) {
        return Ok(image_pairs);
    }

    Err(create_image_not_paired_error())
}

#[cfg(test)]
mod tests {

    mod get_file_path_pairs_if_valid {

        mod returns_error {
            use crate::{
                test_utils::constants::TestConstants,
                utils::file_paths::{get_file_path_pairs_if_valid, tests::test_helpers::setup},
            };

            #[test]
            fn when_paths_do_not_match_when_ignoring_original_latest_path_prefixes() {
                let (config, mut original_file_names, mut latest_file_names) = setup();
                original_file_names.push(
                    format!(
                        "{}/{}/image-with-no-pair.png",
                        TestConstants::IMAGES,
                        TestConstants::ORIGINAL
                    )
                    .to_string(),
                );
                latest_file_names.push(
                    format!(
                        "{}/{}/also-image-with-no-pair.png",
                        TestConstants::IMAGES,
                        TestConstants::LATEST
                    )
                    .to_string(),
                );

                let expected =
                    get_file_path_pairs_if_valid(&config, original_file_names, latest_file_names);

                assert!(expected.is_err());
                assert_eq!(
                    "Not all images are paired up between original and latest. Please confirm image names are the same within the original and latest directories.",
                    expected.unwrap_err().to_string()
                )
            }
        }

        mod returns_pairs_of_images {
            use crate::{
                test_utils::constants::TestConstants,
                utils::file_paths::{get_file_path_pairs_if_valid, tests::test_helpers::setup},
            };

            #[test]
            fn when_paths_all_match() {
                let (config, original_file_names, latest_file_names) = setup();

                assert_eq!(original_file_names.len(), 4);
                assert_eq!(original_file_names.len(), latest_file_names.len());

                let expected =
                    get_file_path_pairs_if_valid(&config, original_file_names, latest_file_names);

                assert!(expected.is_ok());
                assert_eq!(
                    vec![
                        (
                            format!(
                                "{}/{}/another_dir/more_image.png",
                                TestConstants::IMAGES,
                                TestConstants::ORIGINAL
                            ),
                            format!(
                                "{}/{}/another_dir/more_image.png",
                                TestConstants::IMAGES,
                                TestConstants::LATEST
                            )
                        ),
                        (
                            format!(
                                "{}/{}/image.png",
                                TestConstants::IMAGES,
                                TestConstants::ORIGINAL
                            ),
                            format!(
                                "{}/{}/image.png",
                                TestConstants::IMAGES,
                                TestConstants::LATEST
                            )
                        ),
                        (
                            format!(
                                "{}/{}/some_dir/some_image.png",
                                TestConstants::IMAGES,
                                TestConstants::ORIGINAL
                            ),
                            format!(
                                "{}/{}/some_dir/some_image.png",
                                TestConstants::IMAGES,
                                TestConstants::LATEST
                            )
                        ),
                        (
                            format!(
                                "{}/{}/some_dir/some_image_two.png",
                                TestConstants::IMAGES,
                                TestConstants::ORIGINAL
                            ),
                            format!(
                                "{}/{}/some_dir/some_image_two.png",
                                TestConstants::IMAGES,
                                TestConstants::LATEST
                            )
                        ),
                    ],
                    expected.unwrap()
                )
            }
        }
    }

    mod test_helpers {
        use crate::{
            config::AppConfig,
            test_utils::{
                config::create_config_for_test, constants::TestConstants, files::create_file_names,
            },
        };

        pub fn setup() -> (
            AppConfig,
            Vec<std::string::String>,
            Vec<std::string::String>,
        ) {
            let config = create_config_for_test(TestConstants::IMAGES);
            let (original_file_names, latest_file_names) = create_input_for_test();
            (config, original_file_names, latest_file_names)
        }

        fn create_input_for_test() -> (Vec<std::string::String>, Vec<std::string::String>) {
            (
                create_file_names(
                    format!("{}/{}", TestConstants::IMAGES, TestConstants::ORIGINAL).as_str(),
                ),
                create_file_names(
                    format!("{}/{}", TestConstants::IMAGES, TestConstants::LATEST).as_str(),
                ),
            )
        }
    }
}

mod helpers {
    use crate::config::AppConfig;

    pub fn are_file_path_pairs_valid(config: &AppConfig, image_pairs: &[(String, String)]) -> bool {
        for (orig_image_location, lat_image_location) in image_pairs.iter() {
            let original_clean = orig_image_location
                .strip_prefix(&config.get_original_images_dir())
                .unwrap();

            let latest_clean = lat_image_location
                .strip_prefix(&config.get_latest_images_dir())
                .unwrap();

            if original_clean != latest_clean {
                // TODO: log here - test logged message to prove this outcome
                return false;
            }
        }

        true
    }

    pub fn get_pairs_of_file_paths_for_images(
        orig: Vec<String>,
        latest: Vec<String>,
    ) -> Vec<(String, String)> {
        orig.iter()
            .zip(latest.iter())
            .map(|(orig_elem, latest_elem)| (orig_elem.to_owned(), latest_elem.to_owned()))
            .collect()
    }
}
