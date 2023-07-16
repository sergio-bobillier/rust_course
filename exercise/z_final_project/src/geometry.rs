use image::DynamicImage;
use std::fmt;

pub struct Geometry{
    top: u32,
    left: u32,
    width: Option<u32>,
    height: Option<u32>
}

impl Geometry {
    pub fn new(left: u32, top: u32, width: Option<u32>, height: Option<u32>) -> Self {
        Self { left, top, width, height }
    }

    pub fn resolve(& mut self, image: &DynamicImage) {
        self.width.get_or_insert(image.width() - self.left());
        self.height.get_or_insert(image.height() - self.top());
    }

    pub fn top(&self) -> u32 {
        self.top
    }

    pub fn left(&self) -> u32 {
        self.left
    }

    pub fn width(&self) -> u32 {
        self.width.unwrap()
    }

    pub fn height(&self) -> u32 {
        self.height.unwrap()
    }
}

impl fmt::Display for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) - ({}, {})",
            self.left(),
            self.top(),
            self.width() + self.left(),
            self.height() + self.top()
        )
    }
}