use image::DynamicImage;

use super::ProcessingCommand;

pub struct Invert { }

impl Invert {
    pub fn new() -> Self {
        Self { }
    }
}

impl ProcessingCommand for Invert {
    fn name(&self) -> String { "Invert".to_string() }

    fn run(&self, image: DynamicImage) -> DynamicImage {
        let mut inverted = image;
        inverted.invert();
        inverted
    }

    fn description(&self) -> String {
        "inverted the colors of the image".to_string()
    }
}