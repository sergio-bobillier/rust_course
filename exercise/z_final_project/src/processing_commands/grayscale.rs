use image::DynamicImage;

use super::ProcessingCommand;

pub struct Grayscale {}

impl Grayscale {
    pub fn new() -> Self {
        Self { }
    }
}

impl ProcessingCommand for Grayscale {
    fn name(&self) -> String { "Grayscale".to_string() }

    fn run(&self, image: DynamicImage) -> DynamicImage {
        image.grayscale()
    }

    fn description(&self) -> String {
        "turned image to grayscale".to_string()
    }
}
