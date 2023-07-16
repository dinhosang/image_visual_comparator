use image::DynamicImage;

#[derive(Debug, PartialEq)]
pub struct PixelCoord {
    pub x: u32,
    pub y: u32,
}

impl PixelCoord {
    pub fn new(x: u32, y: u32) -> Self {
        PixelCoord { x, y }
    }
}

#[derive(Debug, PartialEq)]
pub struct ImageHolder {
    pub image: DynamicImage,
    pub location: String,
}

impl ImageHolder {
    pub fn new(image: DynamicImage, location: &str) -> Self {
        ImageHolder {
            image,
            location: location.to_string(),
        }
    }
}
