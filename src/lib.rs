mod compare;
pub mod config;
mod errors;
mod models;
mod utils;

mod test_utils;

use compare::compare_pair_of_images;
use config::AppConfig;
use errors::IVCError;
use models::{ImageHolder, PixelCoord};
use tokio::runtime::Runtime;
use tokio::task::JoinSet;
use utils::files::{
    directory_exists, find_files, get_pair_of_images_from_file_locations,
    get_pairs_of_file_paths_for_images,
};

use crate::utils::files::are_file_paths_valid;

pub fn run(config: AppConfig) {
    let pixel_tolerance = config.get_tolerance();

    let original_dir = config.get_original_images_dir();
    let latest_dir = config.get_latest_images_dir();

    if !directory_exists(&original_dir) || !directory_exists(&latest_dir) {
        // TODO FUTURE: add error handling here
        // TODO FUTURE: should return Err, and main invoking run should exit code 1
        // TODO FUTURE: should log reason here though
        println!("TEMP: orig and/or latest directory is missing!");
        println!("expected .... put orig and latest dirs in here");
        return;
    }

    let orig_png_file_paths = find_files(&original_dir, &config.image_extension);
    let latest_png_file_paths = find_files(&latest_dir, &config.image_extension);

    if !are_file_paths_valid(&config, &orig_png_file_paths, &latest_png_file_paths) {
        return;
    }

    let image_pairs =
        get_pairs_of_file_paths_for_images(orig_png_file_paths, latest_png_file_paths);

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut set = JoinSet::new();

        // TODO FUTURE: perhaps create a tokio message channel to send result of a spawned task
        //          can then go through and check for errors etc.

        for (orig_image_location, lat_image_location) in image_pairs.into_iter() {
            set.spawn(async move {
                let _ = handle_pair_of_images(
                    orig_image_location.as_str(),
                    lat_image_location.as_str(),
                    pixel_tolerance,
                );
            });
        }

        while let Some(task_result) = set.join_next().await {
            task_result.unwrap();
        }
    });
}

fn handle_pair_of_images(
    image_location_one: &str,
    image_location_two: &str,
    pixel_tolerance: f32,
) -> Result<Vec<PixelCoord>, IVCError> {
    let images: (ImageHolder, ImageHolder) =
        get_pair_of_images_from_file_locations(image_location_one, image_location_two)?;

    Ok(compare_pair_of_images(&images, pixel_tolerance)?)
}
