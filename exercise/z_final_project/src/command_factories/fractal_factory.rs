use std::collections::VecDeque;

use crate::processing_commands::generators::Fractal;
use super::parameter_fetcher::parameters_left;

pub struct FractalFactory {
    parameters: VecDeque<String>
}

impl FractalFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&self) -> Result<Fractal, String> {
        parameters_left(&self.parameters, "fractal")?;
        Ok(Fractal::new())
    }
}
