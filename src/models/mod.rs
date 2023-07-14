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
