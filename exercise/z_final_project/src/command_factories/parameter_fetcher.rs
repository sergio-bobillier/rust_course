use std::collections::VecDeque;

pub fn require_parameter(parameters: &mut VecDeque<String>, parameter: &str, command: &str) -> Result<String, String> {
    let string = parameters.pop_front();

    match string {
        Some(string) => {
            Ok(string)
        },
        None => {
            Err(format!("Missing required parameter {} for {}", parameter, command))
        }
    }
}

pub fn parse_parameter<T:std::str::FromStr>(parameter_string: String, parameter: &str, command: &str) -> Result<T, String> {
    let value = parameter_string.parse::<T>();

    match value {
        Ok(value) => { Ok(value) },
        Err(_e) => {
            Err(format!("Cannot parse {} as {} for {}. Parse error", parameter_string, parameter, command))
        }
    }
}

pub fn parameters_left(parameters: &VecDeque<String>, command: &str) -> Result<(), String> {
    if !parameters.is_empty() {
        Err(format!("Stray parameter for {}: {}", command, parameters.front().unwrap()))
    }
    else {
        Ok(())
    }
}
