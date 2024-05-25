use super::{ImageHolder, PixelCoord};

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    images: (ImageHolder, ImageHolder),
    mismatched_pixels: Vec<PixelCoord>,
}

impl ComparisonResult {
    pub fn new(images: (ImageHolder, ImageHolder), mismatched_pixels: Vec<PixelCoord>) -> Self {
        ComparisonResult {
            images,
            mismatched_pixels,
        }
    }
}
