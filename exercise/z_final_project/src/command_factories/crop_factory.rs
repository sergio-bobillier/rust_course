use std::collections::VecDeque;

use crate::processing_commands::Crop;
use crate::geometry::Geometry;
use super::parameter_fetcher::fetch_and_parse;
use super::parameter_fetcher::parameters_left;
use super::parameter_fetcher::require_and_parse;

pub struct CropFactory {
    parameters: VecDeque<String>
}

impl CropFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&mut self) -> Result<Crop, String> {
        let command = "crop";

        let parameter = "left";
        let left = require_and_parse(&mut self.parameters, parameter, command)?;

        let parameter = "top";
        let top = require_and_parse(&mut self.parameters, parameter, command)?;

        let parameter = "width";
        let width = fetch_and_parse(&mut self.parameters, parameter, command)?;

        let parameter = "height";
        let height = fetch_and_parse(&mut self.parameters, parameter, command)?;

        let geometry = Geometry::new(
            left, top, width, height
        );

        parameters_left(&self.parameters, "crop")?;
        Ok(Crop::new(geometry))
    }
}