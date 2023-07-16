use super::ProcessingCommand;
use image::DynamicImage;

pub struct Rotate {
    angle: u16
}

impl Rotate {
    pub fn new(angle: u16) -> Self {
        Self { angle }
    }
}

impl ProcessingCommand for Rotate {
    fn name(&self) -> String { "Rotate".to_string() }

    fn run(&self, image: DynamicImage) -> DynamicImage {
        match self.angle {
            90 => image.rotate90(),
            180 => image.rotate180(),
            270 => image.rotate270(),
            _ => panic!("Not a valid angle: {}", self.angle)
        }
    }

    fn description(&self) -> String {
        format!("rotated the image {} degrees", self.angle)
    }
}