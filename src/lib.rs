mod compare;
mod errors;
mod models;
mod utils;

pub mod config;
pub use crate::utils::logger;

mod test_utils;

use tokio::runtime::Runtime;
use tokio::task::JoinSet;

use compare::compare_pair_of_images;
use config::AppConfig;
use errors::{
    handling::{create_dimension_mismatch_error, create_tokio_join_error},
    ivc::IVCError,
};
use utils::{
    file_paths::get_file_path_pairs_if_valid,
    file_system::{
        directories::get_directories_if_exist, files::get_files_if_directories_match_in_file_count,
        images::get_pair_of_images_from_file_locations,
    },
    image::are_dimensions_matching_for_images,
};

pub fn run(config: AppConfig) -> Result<(), IVCError> {
    let pixel_tolerance = config.get_tolerance();

    // TODO: should be able to return Err, and main invoking run should exit code 1
    // TODO: test
    let (original_dir, latest_dir) = get_directories_if_exist(&config)?;

    // TODO: should be able to return Err, and main invoking run should exit code 1
    // TODO: test
    let (orig_image_file_paths, latest_images_file_paths) =
        get_files_if_directories_match_in_file_count(&config, original_dir, latest_dir)?;

    // TODO: should be able to return Err, and main invoking run should exit code 1
    // TODO: test
    let image_pairs =
        get_file_path_pairs_if_valid(&config, orig_image_file_paths, latest_images_file_paths)?;

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let mut set = JoinSet::new();

        // TODO FUTURE: perhaps create a tokio message channel to send result of a spawned task
        //          can then go through and check for errors etc.
        // TODO FUTURE: also currently no return from this part so mismatch error goes nowhere

        for (orig_image_location, lat_image_location) in image_pairs.into_iter() {
            set.spawn_blocking(move || {
                get_pair_of_images_from_file_locations(&orig_image_location, &lat_image_location)
            });
        }

        while let Some(tokio_join_result) = set.join_next().await {
            let task_result = match tokio_join_result {
                Ok(result) => result,
                Err(err) => {
                    return Err(create_tokio_join_error(
                        "retrieving images from file system",
                        err,
                    ));
                }
            };

            let image_pair = match task_result {
                Ok(value) => value,
                Err(err) => return Err(err),
            };

            if !are_dimensions_matching_for_images(&image_pair) {
                return Err(create_dimension_mismatch_error(image_pair));
            }

            let _mismatched_pixels = compare_pair_of_images(&image_pair, pixel_tolerance);
            // TODO: create model to hold image location and mismatched pixels, maybe ImageHolders and mismatched
            //          then need to create image with mismatched pixels business
        }

        Ok(())
    })
}
