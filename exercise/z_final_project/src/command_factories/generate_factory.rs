use std::collections::VecDeque;
use std::num::ParseIntError;
use rand::Rng;

use crate::processing_commands::generators::Generate;
use super::overlay_fetcher::fetch_overlay;
use super::parameter_fetcher::fetch_parameter;
use super::parameter_fetcher::fetch_and_parse;
use super::parameter_fetcher::fetch_option;
use crate::size::Size;

const COLOR_FORMAT_ERROR: &str = "The given color doesn't have the format: r,g,b (all integers betweem 0 and 255)";

pub struct GenerateFactory {
    parameters: VecDeque<String>
}

impl GenerateFactory {
    pub fn new(parameters: VecDeque<String>) -> Self {
        Self { parameters }
    }

    pub fn create(&mut self) -> Result<Generate, String> {
        let command = "generate";

        let parameter = "width";
        let width = fetch_and_parse(&mut self.parameters, parameter, command)?;

        let parameter = "height";
        let height = fetch_and_parse(&mut self.parameters, parameter, command)?;

        let size = Size::new(width, height);

        let color_string = fetch_parameter(&mut self.parameters);
        let color = self.parse_color(color_string)?;

        let noise = fetch_option(&mut self.parameters, "noise");

        let overlay = fetch_overlay(&mut self.parameters, command)?;

        Ok(Generate::new(size, color, noise, overlay))
    }

    fn parse_color(&self, string: Option<String>) -> Result<[u8; 3], String> {
        match string {
            Some(color) => {
                if color == "random" {
                    let mut rng = rand::thread_rng();

                    Ok([
                        rng.gen(),
                        rng.gen(),
                        rng.gen()
                    ])
                }
                else {
                    let parts: Vec<&str> = color.split(",").map( |part| { part.trim() }).collect();

                    if parts.len() != 3 {
                        return Err(COLOR_FORMAT_ERROR.to_string());
                    }

                    let rgb: Result<Vec<u8>, ParseIntError> = parts.iter().map( |part| {
                        part.parse::<u8>()
                    }).collect();

                    match rgb {
                        Ok(values) => {
                            Ok([
                                values[0],
                                values[1],
                                values[2]
                            ])
                        },
                        Err(e) => { Err(COLOR_FORMAT_ERROR.to_string()) }
                    }
                }
            }
            None => {
                Ok([0, 0, 0])
            }
        }
    }
}