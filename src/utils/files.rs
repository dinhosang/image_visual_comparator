use std::path::Path;

use walkdir::WalkDir;

use crate::{config::AppConfig, errors::IOError, models::ImageHolder};

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

pub fn directory_exists(dir: &str) -> bool {
    Path::new(&dir).is_dir()
}

pub fn find_files(dir: &str, desired_ext: &str) -> Vec<String> {
    let mut files: Vec<String> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == desired_ext)
                .unwrap_or(false)
        })
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect();

    files.sort();

    files
}

pub fn are_file_paths_valid(config: &AppConfig, orig: &[String], latest: &[String]) -> bool {
    if orig.len() != latest.len() {
        println!("TEMP: number of files in orig and latest directories don't match");
        return false; // TODO: future throw error here
    }

    for (orig_path, latest_path) in orig.iter().zip(latest.iter()) {
        let orig_clean = orig_path
            .strip_prefix(&config.get_original_images_dir())
            .unwrap_or(orig_path);
        let latest_clean = latest_path
            .strip_prefix(&config.get_latest_images_dir())
            .unwrap_or(latest_path);

        if orig_clean != latest_clean {
            println!("TEMP: at least one of the following is missing a paired image with the same name: {} or {}", orig_clean, latest_clean);
            return false; // TODO: future throw error here
        }
    }

    true
}

pub fn get_pairs_of_file_paths_for_images(
    orig: Vec<String>,
    latest: Vec<String>,
) -> Vec<(String, String)> {
    let pairs: Vec<(String, String)> = orig
        .iter()
        .zip(latest.iter())
        .map(|(orig_elem, latest_elem)| (orig_elem.clone(), latest_elem.clone()))
        .collect();

    pairs
}

#[cfg(test)]
mod tests {

    pub mod helpers {
        pub fn create_file_names(base: &str) -> Vec<String> {
            vec![
                format!("{:}/another_dir/more_image.png", base),
                format!("{:}/image.png", base),
                format!("{:}/some_dir/some_image.png", base),
                format!("{:}/some_dir/some_image_two.png", base),
            ]
        }
    }

    mod directory_exists {
        use std::path::Path;

        use crate::{test_utils::files::create_temp_dir_handler, utils::files::directory_exists};

        #[test]
        fn returns_true_when_directory_exists() {
            let temp_dir_holder = create_temp_dir_handler();
            assert!(directory_exists(temp_dir_holder.get_temp_dir_path()));
        }

        #[test]
        fn returns_false_when_directory_does_not_exist() {
            assert!(!directory_exists(
                Path::new("something")
                    .join("another_ivc")
                    .join("again_ivc")
                    .to_str()
                    .unwrap()
            ));
        }
    }

    mod find_files {
        use std::{
            fs::{create_dir_all, File},
            path::Path,
        };

        use crate::{
            test_utils::files::{create_temp_dir_handler, TempDirHolder},
            utils::files::find_files,
        };

        fn create_sub_dirs_and_files_for_test(temp_dir_holder: &TempDirHolder, desired_ext: &str) {
            let base_dir = Path::new(temp_dir_holder.get_temp_dir_path());
            let sub_dir_first = base_dir.join("first");
            let sub_dir_second = base_dir.join("second");
            let sub_sub_dir_third = sub_dir_first.join("third");
            let sub_sub_dir_fourth = sub_dir_second.join("fourth");
            create_dir_all(sub_dir_first.clone()).unwrap();
            create_dir_all(sub_dir_second.clone()).unwrap();
            create_dir_all(sub_sub_dir_third.clone()).unwrap();
            create_dir_all(sub_sub_dir_fourth.clone()).unwrap();

            File::create(base_dir.join(format!("find_one.{:}", desired_ext))).unwrap();
            File::create(base_dir.join("ignore_one.js")).unwrap();
            File::create(sub_dir_first.join(format!("find_one.{:}", desired_ext))).unwrap();
            File::create(sub_dir_first.join(format!("find_two.{:}", desired_ext))).unwrap();
            File::create(sub_dir_first.join(format!("find_three.{:}", desired_ext))).unwrap();
            File::create(sub_dir_first.join("ignore_one.pdf")).unwrap();
            File::create(sub_dir_second.join("ignore_one.js")).unwrap();
            File::create(sub_dir_second.join("ignore_two.txt")).unwrap();
            File::create(sub_sub_dir_third.join(format!("find_one.{:}", desired_ext))).unwrap();
            File::create(sub_sub_dir_third.join("ignore_one.js")).unwrap();
            File::create(sub_sub_dir_fourth.join(format!("find_one.{:}", desired_ext))).unwrap();
        }

        #[test]
        fn returns_all_files_with_matching_extension_in_dir_and_sub_dirs() {
            let desired_ext = "png";
            let temp_dir_holder = create_temp_dir_handler();

            create_sub_dirs_and_files_for_test(&temp_dir_holder, desired_ext);

            assert_eq!(
                vec![
                    Path::new(temp_dir_holder.get_temp_dir_path())
                        .join("find_one.png")
                        .to_str()
                        .unwrap(),
                    Path::new(temp_dir_holder.get_temp_dir_path())
                        .join("first")
                        .join("find_one.png")
                        .to_str()
                        .unwrap(),
                    Path::new(temp_dir_holder.get_temp_dir_path())
                        .join("first")
                        .join("find_three.png")
                        .to_str()
                        .unwrap(),
                    Path::new(temp_dir_holder.get_temp_dir_path())
                        .join("first")
                        .join("find_two.png")
                        .to_str()
                        .unwrap(),
                    Path::new(temp_dir_holder.get_temp_dir_path())
                        .join("first")
                        .join("third")
                        .join("find_one.png")
                        .to_str()
                        .unwrap(),
                    Path::new(temp_dir_holder.get_temp_dir_path())
                        .join("second")
                        .join("fourth")
                        .join("find_one.png")
                        .to_str()
                        .unwrap(),
                ],
                find_files(temp_dir_holder.get_temp_dir_path(), desired_ext)
            );
        }

        #[test]
        fn returns_no_files_if_none_found_with_matching_extension_in_dir_and_sub_dirs() {
            let desired_ext = "jpeg";
            let temp_dir_holder = create_temp_dir_handler();

            create_sub_dirs_and_files_for_test(&temp_dir_holder, "png");

            let expected: Vec<&str> = Vec::new(); // NOTE: could also do vec![""; 0]

            assert_eq!(
                expected,
                find_files(temp_dir_holder.get_temp_dir_path(), desired_ext)
            );
        }
    }

    mod are_file_paths_valid {
        use std::ffi::OsString;

        use clap::Parser;

        use crate::config::AppConfig;

        use super::helpers::create_file_names;

        fn create_input_for_test() -> (Vec<std::string::String>, Vec<std::string::String>) {
            (
                create_file_names("images/original"),
                create_file_names("images/latest"),
            )
        }

        fn create_config_for_test() -> AppConfig {
            let empty_iter: std::iter::Empty<OsString> = std::iter::empty();
            AppConfig::parse_from(empty_iter)
        }

        mod return_false {
            use crate::utils::files::are_file_paths_valid;

            use super::{create_config_for_test, create_input_for_test};

            #[test]
            fn returns_false_if_more_original_files_than_latest() {
                let (mut original, mut latest) = create_input_for_test();

                latest.pop();

                original.sort();
                latest.sort();

                assert!(!are_file_paths_valid(
                    &create_config_for_test(),
                    &original,
                    &latest
                ));
            }

            #[test]
            fn returns_false_if_more_latest_files_than_original() {
                let (mut original, mut latest) = create_input_for_test();

                original.pop();

                original.sort();
                latest.sort();

                assert!(!are_file_paths_valid(
                    &create_config_for_test(),
                    &original,
                    &latest
                ));
            }

            #[test]
            fn returns_false_if_file_names_do_not_all_match_when_ignoring_base_path() {
                let (mut original, mut latest) = create_input_for_test();

                original.pop();
                original.push("/some/new/path/to/solo/image.png".to_string());

                original.sort();
                latest.sort();

                assert_eq!(original.len(), latest.len(), "lengths should match");
                assert!(!are_file_paths_valid(
                    &create_config_for_test(),
                    &original,
                    &latest
                ));
            }
        }

        mod return_true {
            use crate::utils::files::{
                are_file_paths_valid,
                tests::are_file_paths_valid::{create_config_for_test, create_input_for_test},
            };

            #[test]
            fn returns_true_if_latest_and_original_contents_match_when_ignoring_base_path_to_those_folders(
            ) {
                let (original, latest) = create_input_for_test();

                assert!(!original.is_empty(), "should have elements");
                assert!(are_file_paths_valid(
                    &create_config_for_test(),
                    &original,
                    &latest
                ));
            }
        }
    }

    mod get_pairs_of_file_paths_for_images {
        use crate::utils::files::get_pairs_of_file_paths_for_images;

        use super::helpers::create_file_names;

        #[test]
        fn returns_pairs_of_file_paths_between_original_and_latest() {
            let original = "images/original";
            let latest = "images/latest";

            let pairs = get_pairs_of_file_paths_for_images(
                create_file_names(original),
                create_file_names(latest),
            );

            assert_eq!(
                vec![
                    (
                        format!("{:}/another_dir/more_image.png", original),
                        format!("{:}/another_dir/more_image.png", latest)
                    ),
                    (
                        format!("{:}/image.png", original),
                        format!("{:}/image.png", latest)
                    ),
                    (
                        format!("{:}/some_dir/some_image.png", original),
                        format!("{:}/some_dir/some_image.png", latest)
                    ),
                    (
                        format!("{:}/some_dir/some_image_two.png", original),
                        format!("{:}/some_dir/some_image_two.png", latest)
                    ),
                ],
                pairs,
            )
        }
    }

    mod get_pair_of_images_from_file_locations {

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
                let (image_one_location, image_two_location) =
                    get_image_locations(&temp_dir_holder);

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
                let (image_one_location, image_two_location) =
                    get_image_locations(&temp_dir_holder);

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
                let (image_one_location, image_two_location) =
                    get_image_locations(&temp_dir_holder);

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
                let (image_one_location, image_two_location) =
                    get_image_locations(&temp_dir_holder);

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
                let (image_one_location, image_two_location) =
                    get_image_locations(&temp_dir_holder);

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
}
