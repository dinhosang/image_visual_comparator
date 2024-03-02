use std::collections::HashMap;

use image::ImageError;

use crate::models::ImageHolder;

use super::{
    external::IOReadError,
    internal::{
        ImageCountMismatchError, ImageNotPairedError, ImagePairDimensionMismatchError,
        MissingDirectoriesError,
    },
    ivc::IVCError,
};

pub fn create_missing_directories_error(
    original_dir: String,
    does_orig_exist: bool,
    latest_dir: String,
    does_latest_exist: bool,
) -> IVCError {
    let mut missing_directories = HashMap::new();

    if !does_orig_exist {
        missing_directories.insert("original".to_string(), original_dir);
    }

    if !does_latest_exist {
        missing_directories.insert("latest".to_string(), latest_dir);
    }

    IVCError::MissingDirectory(MissingDirectoriesError::new(missing_directories))
}

pub fn create_image_count_mismatch_error(original_count: usize, latest_count: usize) -> IVCError {
    IVCError::ImageCountMismatch(ImageCountMismatchError::new(original_count, latest_count))
}

pub fn create_image_not_paired_error() -> IVCError {
    IVCError::ImageNotPaired(ImageNotPairedError::new())
}

pub fn create_io_read_error(location: String, source: ImageError) -> IVCError {
    IVCError::IORead(IOReadError::new(location, source))
}

pub fn create_dimension_mismatch_error(images: (ImageHolder, ImageHolder)) -> IVCError {
    IVCError::ImagePairDimensionMismatch(ImagePairDimensionMismatchError::new(
        images.0.location.to_string(),
        images.1.location.to_string(),
    ))
}
