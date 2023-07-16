mod blur_factory;
mod brighten_factory;
mod crop_factory;
mod grayscale_factory;
mod invert_factory;
mod parameter_fetcher;
mod rotate_factory;

use std::collections::VecDeque;

use super::ProcessingCommand;
use blur_factory::BlurFactory;
use brighten_factory::BrightenFactory;
use crop_factory::CropFactory;
use grayscale_factory::GrayscaleFactory;
use invert_factory::InvertFactory;
use rotate_factory::RotateFactory;

struct Command {
    name: String,
    parameters: VecDeque<String>
}

impl Command {
    fn new(name: String) -> Self {
        Self { name, parameters: VecDeque::new() }
    }
}

pub struct CommandsFactory {
    args: VecDeque<String>
}

impl CommandsFactory {
    pub fn new(args: VecDeque<String>) -> Self {
        Self { args }
    }

    pub fn create(&mut self) -> Result<Vec<Box<dyn ProcessingCommand>>, String> {
        let mut processing_commands: Vec<Box<dyn ProcessingCommand>> = Vec::with_capacity(self.args.len());
        let mut commands: Vec<Command> = Vec::new();

        loop {
            let arg = self.args.pop_front();

            match arg {
                Some(arg) => {
                    if let Some(name) = arg.strip_prefix("--") {
                        commands.push(Command::new(name.to_string()))
                    } else {
                        let last_command = commands.last_mut();

                        match last_command {
                            Some(command) => {
                                command.parameters.push_back(arg);
                            },
                            None => {
                                return Err(format!("Command parameter without a command: '{}'", arg));
                            }
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }

        for command in commands {
            if command.name == "blur" {
                let mut factory = BlurFactory::new(command.parameters);
                let blur = factory.create()?;
                processing_commands.push(Box::new(blur));
            } else if command.name == "crop" {
                let mut factory = CropFactory::new(command.parameters);
                let crop = factory.create()?;
                processing_commands.push(Box::new(crop));
            } else if command.name == "brighten" {
                let mut factory = BrightenFactory::new(command.parameters);
                let brighten = factory.create()?;
                processing_commands.push(Box::new(brighten));
            } else if command.name == "rotate" {
                let mut factory = RotateFactory::new(command.parameters);
                let rotate = factory.create()?;
                processing_commands.push(Box::new(rotate));
            } else if command.name == "invert" {
                let factory = InvertFactory::new(command.parameters);
                let invert = factory.create()?;
                processing_commands.push(Box::new(invert));
            } else if command.name == "grayscale" {
                let factory = GrayscaleFactory::new(command.parameters);
                let grayscale = factory.create()?;
                processing_commands.push(Box::new(grayscale));
            } else {
                return Err(format!("Unrecognized command: '{}'", command.name));
            }
        }

        Ok(processing_commands)
    }
}