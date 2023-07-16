use core::fmt;

use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IVCError {
    Comparison(#[from] DimensionMismatchError),
    IO(#[from] IOError),
}

impl fmt::Display for IVCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IVCError::Comparison(error) => write!(f, "{}", error),
            IVCError::IO(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Error, Debug)]
#[error("dimensions do not match: {location_one} and {location_two}")]
pub struct DimensionMismatchError {
    pub location_one: String,
    pub location_two: String,
}

impl DimensionMismatchError {
    pub fn new(location_one: String, location_two: String) -> Self {
        DimensionMismatchError {
            location_one,
            location_two,
        }
    }
}

#[derive(Error, Debug)]
pub enum IOError {
    #[error("when reading location: '{location}'. Message: '{0}'", source.to_string())]
    Read {
        location: String,
        source: ImageError,
    },
}
