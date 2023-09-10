use image::{DynamicImage, Pixel};
use image::imageops::overlay;

pub struct Overlay {
    alpha: u8
}

impl Overlay {
    pub fn new(alpha: u8) -> Self {
        Self { alpha }
    }

    pub fn run(&self, background: DynamicImage, foreground: DynamicImage) -> DynamicImage {
        let mut bottom = background.to_rgba8();
        let mut top = foreground.to_rgba8();

        for (_x, _y, pixel) in top.enumerate_pixels_mut() {
            pixel.channels_mut()[3] = self.alpha;
        }

        overlay( &mut bottom, &top, 0, 0);
        DynamicImage::from(bottom)
    }
}