use std::collections::VecDeque;

use crate::processing_commands::Rotate;
use super::parameter_fetcher::require_parameter;
use super::parameter_fetcher::parse_parameter;
use super::parameter_fetcher::parameters_left;

const VALID_ANGLES:[u16; 3] = [90, 180, 270];

pub struct RotateFactory {
    parameters: VecDeque<String>
}

impl RotateFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&mut self) -> Result<Rotate, String> {
        let command = "rotate";

        let parameter = "angle";
        let angle_string = require_parameter(&mut self.parameters, parameter, command)?;
        let angle = parse_parameter(angle_string, parameter, command)?;
        let valid_angle = self.valid_angle(angle)?;

        parameters_left(&self.parameters, command)?;
        Ok(Rotate::new(valid_angle))
    }

    fn valid_angle(&self, angle: u16) -> Result<u16, String> {
        if VALID_ANGLES.contains(&angle) {
            Ok(angle)
        } else {
            Err(format!("Invalid angle: {} for rotate. Valid angles are {:?}", angle, VALID_ANGLES))
        }
    }
}