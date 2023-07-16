use super::processing_command::ProcessingCommand;
use image::DynamicImage;

pub struct Blur {
    amount: f32
}

impl Blur {
    pub fn new(amount: f32) -> Blur {
        Self { amount }
    }
}

impl ProcessingCommand for Blur {
    fn name(&self) -> String { "Blur".to_string() }

    fn run(&self, image: DynamicImage) -> DynamicImage {
        image.blur(self.amount)
    }

    fn description(&self) -> String {
        format!("blurred image by {}", self.amount)
    }
}
