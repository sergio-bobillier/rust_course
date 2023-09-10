use image::DynamicImage;
use std::fmt;

pub struct Size {
    width: Option<u32>,
    height: Option<u32>
}

impl Size {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self {width, height}
    }

    pub fn resolve(& mut self, image: &DynamicImage) {
        self.width.get_or_insert(image.width());
        self.height.get_or_insert(image.height());
    }

    pub fn width(&self) -> u32 {
        self.width.unwrap()
    }

    pub fn height(&self) -> u32 {
        self.height.unwrap()
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width(), self.height())
    }
}