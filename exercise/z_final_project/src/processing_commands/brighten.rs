use super::ProcessingCommand;
use image::DynamicImage;

pub struct Brighten {
    delta: i32
}

impl Brighten {
    pub fn new(delta: i32) -> Self {
        Self { delta }
    }
}

impl ProcessingCommand for Brighten {
    fn name(&self) -> String { "Brighten".to_string() }

    fn run(&self, image: DynamicImage) -> DynamicImage {
        image.brighten(self.delta)
    }

    fn description(&self) -> String {
        let action = if self.delta < 0 {
            "dimmed"
        } else {
            "brightened"
        };

        format!("{} image by {}", action, self.delta)
    }
}