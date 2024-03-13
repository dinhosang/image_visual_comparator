use std::fmt;

use super::external::IOReadError;
use super::external::TokioJoinError;
use super::internal::ImageCountMismatchError;
use super::internal::ImageNotPairedError;
use super::internal::ImagePairDimensionMismatchError;
use super::internal::MissingDirectoriesError;

#[derive(Debug)]
pub enum IVCError {
    ImagePairDimensionMismatch(ImagePairDimensionMismatchError),
    IORead(IOReadError),
    MissingDirectory(MissingDirectoriesError),
    ImageCountMismatch(ImageCountMismatchError),
    ImageNotPaired(ImageNotPairedError),
    TokioJoin(TokioJoinError),
}

impl fmt::Display for IVCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IVCError::ImagePairDimensionMismatch(err) => err.fmt(f),
            IVCError::IORead(err) => err.fmt(f),
            IVCError::MissingDirectory(err) => err.fmt(f),
            IVCError::ImageCountMismatch(err) => err.fmt(f),
            IVCError::ImageNotPaired(err) => err.fmt(f),
            IVCError::TokioJoin(err) => err.fmt(f),
        }
    }
}
