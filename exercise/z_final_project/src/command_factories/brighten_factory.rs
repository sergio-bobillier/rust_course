use std::collections::VecDeque;

use crate::processing_commands::Brighten;
use super::parameter_fetcher::require_parameter;
use super::parameter_fetcher::parse_parameter;
use super::parameter_fetcher::parameters_left;

pub struct BrightenFactory {
    parameters: VecDeque<String>
}

impl BrightenFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&mut self) -> Result<Brighten, String> {
        let command = "brighten";

        let parameter = "delta";
        let delta_string = require_parameter(&mut self.parameters, parameter, command)?;
        let delta = parse_parameter(delta_string, parameter, command)?;

        parameters_left(&self.parameters, command)?;
        Ok(Brighten::new(delta))
    }
}