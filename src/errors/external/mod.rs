use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Issue parsing file at location: '{location}'. Message: '{source_message}'")]
pub struct IOReadError {
    location: String,
    source_message: String,
}

impl IOReadError {
    pub fn new(location: String, source: ImageError) -> Self {
        IOReadError {
            location,
            source_message: source.to_string(),
        }
    }
}
