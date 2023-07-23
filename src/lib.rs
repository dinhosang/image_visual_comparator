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
use utils::files::get_pair_of_images_from_file_locations;

pub fn run(config: AppConfig) {
    let pixel_tolerance = config.get_tolerance();

    /*
        TODO:
        - check if directory for current exists
        - check if directory for original exists
        - check that there are no solo images
            - should error if there are
        - should grab pairs of images from current/original dirs
            - need new struct ?
        - should then use to spawn the comparison process
    */

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut set = JoinSet::new();

        for index in 0..10 {
            set.spawn(async move {
                let _ = handle_pair_of_images(
                    format!("./images/original/image_{index}.png").as_str(),
                    format!("./images/current/image_{index}.png").as_str(),
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
