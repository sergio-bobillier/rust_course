use std::collections::VecDeque;

use crate::processing_commands::Grayscale;
use super::parameter_fetcher::parameters_left;

pub struct GrayscaleFactory {
    parameters: VecDeque<String>
}

impl GrayscaleFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&self) -> Result<Grayscale, String> {
        parameters_left(&self.parameters, "grayscale")?;
        Ok(Grayscale::new())
    }
}