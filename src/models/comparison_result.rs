use super::{ImageHolder, PixelCoord};

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    original_image: ImageHolder,
    latest_image: ImageHolder,
    // TODO: make non optional after creating image
    comparison_image: Option<ImageHolder>,
    mismatched_pixels: Vec<PixelCoord>,
}

impl ComparisonResult {
    pub fn new(
        original_latest_image_pair: (ImageHolder, ImageHolder),
        mismatched_pixels: Vec<PixelCoord>,
        comparison_image: Option<ImageHolder>,
    ) -> Self {
        ComparisonResult {
            original_image: original_latest_image_pair.0,
            latest_image: original_latest_image_pair.1,
            mismatched_pixels,
            comparison_image,
        }
    }
}
