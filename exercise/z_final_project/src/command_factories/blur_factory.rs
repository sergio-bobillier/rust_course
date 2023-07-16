use std::collections::VecDeque;

use crate::processing_commands::Blur;
use super::parameter_fetcher::require_parameter;
use super::parameter_fetcher::parse_parameter;
use super::parameter_fetcher::parameters_left;

pub struct BlurFactory {
    parameters: VecDeque<String>
}

impl BlurFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&mut self) -> Result<Blur, String> {
        let command = "blur";

        let parameter = "amount";
        let amount_string = require_parameter(&mut self.parameters, parameter, command)?;
        let amount = parse_parameter(amount_string, parameter, command)?;

        parameters_left(&self.parameters, command)?;
        Ok(Blur::new(amount))
    }
}
