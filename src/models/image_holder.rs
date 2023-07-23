use image::DynamicImage;

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
