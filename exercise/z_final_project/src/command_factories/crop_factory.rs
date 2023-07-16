use std::collections::VecDeque;

use crate::processing_commands::Crop;
use crate::geometry::Geometry;
use super::parameter_fetcher::parameters_left;

pub struct CropFactory {
    parameters: VecDeque<String>
}

impl CropFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&mut self) -> Result<Crop, String> {
        let left: u32 = self.required("left")?;
        let top: u32 = self.required("top")?;
        let width: Option<u32> = self.consume("width")?;
        let height: Option<u32> = self.consume("height")?;

        let geometry = Geometry::new(
            left, top, width, height
        );

        parameters_left(&self.parameters, "crop")?;
        Ok(Crop::new(geometry))
    }

    fn required(&mut self, name: &str) -> Result<u32, String> {
        let value = self.consume(name)?;

        match value {
            Some(value) => {
                Ok(value)
            },
            None => {
                Err(format!("Missing required parameter {} for crop", name))
            }
        }
    }

    fn consume(&mut self, name: &str) -> Result<Option<u32>, String> {
        let parameter = self.parameters.pop_front();

        match parameter {
            Some(string) => {
                if string == "-" { return Ok(None) }

                let result = string.parse::<u32>();

                match result {
                    Ok(value) => {
                        Ok(Some(value))
                    }
                    Err(_e) => {
                        Err(format!("Cannot parse {} as {} for --crop", string, name))
                    }
                }
            }
            None => {
                Ok(None)
            }
        }
    }
}