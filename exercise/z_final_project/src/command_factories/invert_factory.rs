use std::collections::VecDeque;

use crate::processing_commands::Invert;
use super::parameter_fetcher::parameters_left;

pub struct InvertFactory {
    parameters: VecDeque<String>
}

impl InvertFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(self) -> Result<Invert, String> {
        parameters_left(&self.parameters, "invert")?;
        Ok(Invert::new())
    }
}