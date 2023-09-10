use std::collections::VecDeque;

pub fn fetch_option(parameters: &mut VecDeque<String>, name: &str) -> bool {
    let string = fetch_parameter(parameters);

    match string {
        Some(string) => {
            string == format!("-{}", name)
        },
        None => { false }
    }
}

pub fn fetch_parameter(parameters: &mut VecDeque<String>) -> Option<String> {
    let string = parameters.pop_front();

    match string {
        Some(string) => {
            if string == "-" {
                None
            } else {
                Some(string)
            }
        },
        None => {
            None
        }
    }
}

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

pub fn fetch_and_parse<T:std::str::FromStr>(parameters: &mut VecDeque<String>, parameter: &str, command: &str) -> Result<Option<T>, String> {
    let parameter_string = fetch_parameter(parameters);

    match parameter_string {
        Some(string) => {
            let parsed = parse_parameter::<T>(string, parameter, command)?;
            Ok(Some(parsed))
        },
        None => {
            Ok(None)
        }
    }
}

pub fn require_and_parse<T:std::str::FromStr>(parameters: &mut VecDeque<String>, parameter: &str, command: &str) -> Result<T, String> {
    let parameter_string = require_parameter(parameters, parameter, command)?;
    let parsed = parse_parameter::<T>(parameter_string, parameter, command)?;
    Ok(parsed)
}

pub fn parameters_left(parameters: &VecDeque<String>, command: &str) -> Result<(), String> {
    if !parameters.is_empty() {
        Err(format!("Stray parameter for {}: {}", command, parameters.front().unwrap()))
    }
    else {
        Ok(())
    }
}
