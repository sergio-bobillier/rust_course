use image::DynamicImage;
use crate::geometry::Geometry;
use super::ProcessingCommand;

pub struct Crop {
    geometry: Geometry
}

impl Crop {
    pub fn new(geometry: Geometry) -> Self {
        Self { geometry }
    }
}

impl ProcessingCommand for Crop {
    fn name(&self) -> String { "Crop".to_string() }

    fn pre_process(&mut self, image: &DynamicImage) {
        self.geometry.resolve(image);
    }

    fn run(&self, image: DynamicImage) -> DynamicImage {
        image.crop_imm(
            self.geometry.left(),
            self.geometry.top(),
            self.geometry.width(),
            self.geometry.height()
        )
    }

    fn description(&self) -> String {
        format!("cropped image with geometry {}", self.geometry)
    }
}
