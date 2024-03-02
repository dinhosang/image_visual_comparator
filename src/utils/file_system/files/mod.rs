use crate::{
    config::AppConfig,
    errors::{handling::create_image_count_mismatch_error, ivc::IVCError},
};

use self::helpers::find_files;

pub fn get_files_if_directories_match_in_file_count(
    config: &AppConfig,
    original_dir: String,
    latest_dir: String,
) -> Result<(Vec<std::string::String>, Vec<std::string::String>), IVCError> {
    let orig_image_file_paths = find_files(&original_dir, &config.image_extension);
    let latest_images_file_paths = find_files(&latest_dir, &config.image_extension);

    if orig_image_file_paths.len() == latest_images_file_paths.len() {
        return Ok((orig_image_file_paths, latest_images_file_paths));
    }

    Err(create_image_count_mismatch_error(
        orig_image_file_paths.len(),
        latest_images_file_paths.len(),
    ))
}

#[cfg(test)]
mod tests {
    mod get_files_if_directories_match_in_file_count {
        mod returns_error {
            use crate::{
                test_utils::{
                    config::create_config_for_test, constants::TestConstants,
                    files::create_temp_dir_handler,
                },
                utils::file_system::files::{
                    get_files_if_directories_match_in_file_count,
                    tests::test_helpers::create_dirs_and_files,
                },
            };

            #[test]
            fn when_latest_directory_has_a_lower_returned_files_count() {
                let temp_dir_handler = create_temp_dir_handler();
                let config = create_config_for_test(temp_dir_handler.get_temp_dir_path());
                create_dirs_and_files(&temp_dir_handler, TestConstants::ORIGINAL, "first", true);
                create_dirs_and_files(&temp_dir_handler, TestConstants::LATEST, "first", false);

                let result = get_files_if_directories_match_in_file_count(
                    &config,
                    format!(
                        "{}/{}",
                        temp_dir_handler.get_temp_dir_path(),
                        TestConstants::ORIGINAL
                    ),
                    format!(
                        "{}/{}",
                        temp_dir_handler.get_temp_dir_path(),
                        TestConstants::LATEST
                    ),
                );

                assert!(result.is_err());
                assert_eq!(
                    "Number of images in original and latest directories do not match. Original: '6', Latest: '5'.",
                    result.unwrap_err().to_string(),
                )
            }

            #[test]
            fn when_original_directory_has_a_lower_returned_files_count() {
                let temp_dir_handler = create_temp_dir_handler();
                let config = create_config_for_test(temp_dir_handler.get_temp_dir_path());
                create_dirs_and_files(&temp_dir_handler, TestConstants::ORIGINAL, "first", false);
                create_dirs_and_files(&temp_dir_handler, TestConstants::LATEST, "first", true);

                let result = get_files_if_directories_match_in_file_count(
                    &config,
                    format!(
                        "{}/{}",
                        temp_dir_handler.get_temp_dir_path(),
                        TestConstants::ORIGINAL
                    ),
                    format!(
                        "{}/{}",
                        temp_dir_handler.get_temp_dir_path(),
                        TestConstants::LATEST
                    ),
                );

                assert!(result.is_err());
                assert_eq!(
                    "Number of images in original and latest directories do not match. Original: '5', Latest: '6'.",
                    result.unwrap_err().to_string(),
                )
            }
        }

        mod returns_image_file_paths {
            use crate::{
                test_utils::{
                    config::create_config_for_test, constants::TestConstants,
                    files::create_temp_dir_handler,
                },
                utils::file_system::files::{
                    get_files_if_directories_match_in_file_count,
                    tests::test_helpers::{create_dirs_and_files, get_expected_file_paths_for_dir},
                },
            };

            #[test]
            fn when_both_original_and_latest_directory_have_matching_file_count() {
                let temp_dir_handler = create_temp_dir_handler();
                let config = create_config_for_test(temp_dir_handler.get_temp_dir_path());
                create_dirs_and_files(&temp_dir_handler, TestConstants::ORIGINAL, "first", true);
                create_dirs_and_files(&temp_dir_handler, TestConstants::LATEST, "first", true);

                let result = get_files_if_directories_match_in_file_count(
                    &config,
                    format!(
                        "{}/{}",
                        temp_dir_handler.get_temp_dir_path(),
                        TestConstants::ORIGINAL
                    ),
                    format!(
                        "{}/{}",
                        temp_dir_handler.get_temp_dir_path(),
                        TestConstants::LATEST
                    ),
                );

                assert!(result.is_ok());

                let expected_original = get_expected_file_paths_for_dir(
                    &temp_dir_handler,
                    TestConstants::ORIGINAL,
                    "first",
                    true,
                );
                let expected_latest = get_expected_file_paths_for_dir(
                    &temp_dir_handler,
                    TestConstants::LATEST,
                    "first",
                    true,
                );
                assert_eq!((expected_original, expected_latest), result.unwrap())
            }

            #[test]
            fn when_both_original_and_latest_directory_have_matching_file_count_even_if_paths_dont_match(
            ) {
                let temp_dir_handler = create_temp_dir_handler();
                let config = create_config_for_test(temp_dir_handler.get_temp_dir_path());
                create_dirs_and_files(&temp_dir_handler, TestConstants::ORIGINAL, "first", true);
                create_dirs_and_files(&temp_dir_handler, TestConstants::LATEST, "second", true);

                let result = get_files_if_directories_match_in_file_count(
                    &config,
                    format!(
                        "{}/{}",
                        temp_dir_handler.get_temp_dir_path(),
                        TestConstants::ORIGINAL
                    ),
                    format!(
                        "{}/{}",
                        temp_dir_handler.get_temp_dir_path(),
                        TestConstants::LATEST
                    ),
                );

                assert!(result.is_ok());

                let expected_original = get_expected_file_paths_for_dir(
                    &temp_dir_handler,
                    TestConstants::ORIGINAL,
                    "first",
                    true,
                );
                let expected_latest = get_expected_file_paths_for_dir(
                    &temp_dir_handler,
                    TestConstants::LATEST,
                    "second",
                    true,
                );
                assert_eq!((expected_original, expected_latest), result.unwrap())
            }
        }
    }

    mod test_helpers {
        use std::{
            fs::{create_dir_all, File},
            path::Path,
        };

        use crate::test_utils::files::TempDirHandler;

        pub fn create_dirs_and_files(
            temp_dir_handler: &TempDirHandler,
            base_dir_name: &str,
            unique_dir_name: &str,
            include_additional_image: bool,
        ) {
            let images_dir = Path::new(temp_dir_handler.get_temp_dir_path());

            let base_dir = images_dir.join(base_dir_name);
            let sub_dir = base_dir.join("common");
            let unique_sub_dir = base_dir.join(unique_dir_name);

            create_dir_all(base_dir.clone()).unwrap();
            create_dir_all(sub_dir.clone()).unwrap();
            create_dir_all(unique_sub_dir.clone()).unwrap();

            File::create(base_dir.join("find_one.png")).unwrap();
            File::create(base_dir.join("ignore_one.js")).unwrap();

            File::create(sub_dir.join("find_two.png")).unwrap();
            File::create(sub_dir.join("find_three.png")).unwrap();
            File::create(sub_dir.join("find_four.png")).unwrap();
            File::create(sub_dir.join("ignore_one.pdf")).unwrap();

            File::create(unique_sub_dir.join("find_five.png")).unwrap();
            File::create(unique_sub_dir.join("ignore_one.js")).unwrap();

            if include_additional_image {
                File::create(sub_dir.join("find_six.png")).unwrap();
            }
        }

        pub fn get_expected_file_paths_for_dir(
            temp_dir_handler: &TempDirHandler,
            base_dir_name: &str,
            unique_dir_name: &str,
            include_additional_image: bool,
        ) -> Vec<String> {
            let images_dir = Path::new(temp_dir_handler.get_temp_dir_path());

            let base_dir = images_dir.join(base_dir_name);
            let sub_dir = base_dir.join("common");
            let unique_sub_dir = base_dir.join(unique_dir_name);

            let mut expected = vec![
                base_dir.join("find_one.png"),
                sub_dir.join("find_two.png"),
                sub_dir.join("find_three.png"),
                sub_dir.join("find_four.png"),
                unique_sub_dir.join("find_five.png"),
            ];

            if include_additional_image {
                expected.push(sub_dir.join("find_six.png"));
            }

            expected.sort();

            expected
                .iter_mut()
                .map(|path| path.to_str().unwrap().to_string())
                .collect()
        }
    }
}

mod helpers {
    use walkdir::WalkDir;

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
}
