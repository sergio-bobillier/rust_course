use image::DynamicImage;

pub trait ProcessingCommand {
    fn name(&self) -> String;

    fn pre_process(&mut self, _image: &DynamicImage) { }

    fn run(&self, image: DynamicImage) -> DynamicImage;

    fn description(&self) -> String;
}
