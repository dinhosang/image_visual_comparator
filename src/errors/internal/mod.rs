use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Could not find directories: '{missing_directories:#?}'.")]
pub struct MissingDirectoriesError {
    missing_directories: HashMap<String, String>,
}

impl MissingDirectoriesError {
    pub fn new(missing_directories: HashMap<String, String>) -> Self {
        MissingDirectoriesError {
            missing_directories,
        }
    }
}

#[derive(Error, Debug)]
#[error("Image dimensions do not match: '{location_one}' and '{location_two}'.")]
pub struct ImagePairDimensionMismatchError {
    location_one: String,
    location_two: String,
}

impl ImagePairDimensionMismatchError {
    pub fn new(location_one: String, location_two: String) -> Self {
        ImagePairDimensionMismatchError {
            location_one,
            location_two,
        }
    }
}

#[derive(Error, Debug)]
#[error("Number of images in original and latest directories do not match. Original: '{original_count}', Latest: '{latest_count}'.")]
pub struct ImageCountMismatchError {
    original_count: usize,
    latest_count: usize,
}

impl ImageCountMismatchError {
    pub fn new(original_count: usize, latest_count: usize) -> Self {
        ImageCountMismatchError {
            original_count,
            latest_count,
        }
    }
}

#[derive(Error, Debug)]
#[error("Not all images are paired up between original and latest. Please confirm image names are the same within the original and latest directories.")]
pub struct ImageNotPairedError {}

impl Default for ImageNotPairedError {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageNotPairedError {
    pub fn new() -> Self {
        ImageNotPairedError {}
    }
}
