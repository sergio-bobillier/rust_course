use std::collections::VecDeque;

use crate::processing_commands::generators::Fractal;
use super::overlay_fetcher::fetch_overlay;
use super::parameter_fetcher::fetch_and_parse;
use super::parameter_fetcher::parameters_left;
use crate::size::Size;

pub struct FractalFactory {
    parameters: VecDeque<String>
}

impl FractalFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&mut self) -> Result<Fractal, String> {
        let command = "fractal";

        let parameter = "width";
        let width = fetch_and_parse(&mut self.parameters, parameter, command)?;

        let parameter = "height";
        let height = fetch_and_parse(&mut self.parameters, parameter, command)?;

        let size = Size::new(width, height);

        let overlay = fetch_overlay(&mut self.parameters, command)?;

        parameters_left(&self.parameters, "fractal")?;
        Ok(Fractal::new(size, overlay))
    }
}
