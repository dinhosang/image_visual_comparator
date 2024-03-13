use thiserror::Error;

use image::ImageError;
use tokio::task::JoinError;

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

#[derive(Error, Debug)]
#[error("Issue with tokio join set when performing '{action}'")]
pub struct TokioJoinError {
    action: String,
    source_message: String,
}

impl TokioJoinError {
    pub fn new(action: String, source: JoinError) -> Self {
        TokioJoinError {
            action,
            source_message: source.to_string(),
        }
    }
}
