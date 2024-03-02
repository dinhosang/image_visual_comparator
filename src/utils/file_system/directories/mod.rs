use crate::{config::AppConfig, errors::ivc::IVCError};

use self::helpers::validate_directories_exist;

pub fn get_directories_if_exist(config: &AppConfig) -> Result<(String, String), IVCError> {
    let original_dir = config.get_original_images_dir();
    let latest_dir = config.get_latest_images_dir();

    validate_directories_exist(&original_dir, &latest_dir)?;

    Ok((original_dir, latest_dir))
}

#[cfg(test)]
mod tests {
    mod get_directories_if_exist {
        use crate::{
            config::AppConfig,
            test_utils::{
                config::create_config_for_test,
                constants::TestConstants,
                files::{create_temp_dir_handler, TempDirHandler},
            },
        };

        fn setup(
            has_original: bool,
            has_latest: bool,
        ) -> (TempDirHandler, String, String, AppConfig) {
            let temp_dir_handler = create_temp_dir_handler();
            let config = create_config_for_test(temp_dir_handler.get_temp_dir_path());

            if has_original {
                temp_dir_handler.create_dir_in_temp_dir(TestConstants::ORIGINAL)
            }

            if has_latest {
                temp_dir_handler.create_dir_in_temp_dir(TestConstants::LATEST)
            }

            let original_dir = format!(
                "{}/{}",
                temp_dir_handler.get_temp_dir_path(),
                TestConstants::ORIGINAL
            );
            let latest_dir = format!(
                "{}/{}",
                temp_dir_handler.get_temp_dir_path(),
                TestConstants::LATEST
            );

            (temp_dir_handler, original_dir, latest_dir, config)
        }
        mod returns_error {
            use crate::{
                test_utils::config::create_config_for_test,
                utils::file_system::directories::{
                    get_directories_if_exist, tests::get_directories_if_exist::setup,
                },
            };

            #[test]
            fn when_neither_directory_exists() {
                let expected_error = get_directories_if_exist(&create_config_for_test("something"));
                assert!(expected_error.is_err());

                let error_text = expected_error.unwrap_err().to_string();
                assert!(error_text.contains("Could not find directories: '"));
                assert!(error_text.contains("\"original\": \"something/original\""));
                assert!(error_text.contains("\"latest\": \"something/latest\""));
            }

            #[test]
            fn when_only_original_directory_exists() {
                let (_temp_dir_handler, original_dir, latest_dir, config) = setup(true, false);

                let expected_error = get_directories_if_exist(&config);
                assert!(expected_error.is_err());

                let error_text = expected_error.unwrap_err().to_string();
                assert!(error_text.contains("Could not find directories: '"));
                assert!(error_text.contains(&format!("\"latest\": \"{latest_dir}\"")));
                assert!(!error_text.contains(&format!("\"original\": \"{original_dir}\"")));
            }

            #[test]
            fn when_only_latest_directory_exists() {
                let (_temp_dir_handler, original_dir, latest_dir, config) = setup(false, true);

                let expected_error = get_directories_if_exist(&config);
                assert!(expected_error.is_err());

                let error_text = expected_error.unwrap_err().to_string();
                assert!(error_text.contains("Could not find directories: '"));
                assert!(error_text.contains(&format!("\"original\": \"{original_dir}\"")));
                assert!(!error_text.contains(&format!("\"latest\": \"{latest_dir}\"")));
            }
        }

        mod returns_directories {
            use crate::utils::file_system::directories::{
                get_directories_if_exist, tests::get_directories_if_exist::setup,
            };

            #[test]
            fn when_both_original_and_latest_exist() {
                let (_temp_dir_handler, original_dir, latest_dir, config) = setup(true, true);
                let (actual_original, actual_latest) = get_directories_if_exist(&config).unwrap();

                assert_eq!(original_dir, actual_original);
                assert_eq!(latest_dir, actual_latest);
            }
        }
    }
}

mod helpers {
    use std::path::Path;

    use crate::errors::{handling::create_missing_directories_error, ivc::IVCError};

    fn directory_exists(dir: &str) -> bool {
        if Path::new(&dir).is_dir() {
            return true;
        }

        // TODO: should log if false
        // TODO: test that log happens
        false
    }

    pub fn validate_directories_exist(
        original_dir: &String,
        latest_dir: &String,
    ) -> Result<(), IVCError> {
        let (orig_dir_exists, latest_dir_exists) =
            (directory_exists(original_dir), directory_exists(latest_dir));

        if orig_dir_exists && latest_dir_exists {
            return Ok(());
        }

        Err(create_missing_directories_error(
            original_dir.to_string(),
            orig_dir_exists,
            latest_dir.to_string(),
            latest_dir_exists,
        ))
    }
}
