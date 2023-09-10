use std::collections::VecDeque;

use crate::overlay::Overlay;
use super::parameter_fetcher::fetch_option;
use super::parameter_fetcher::require_and_parse;

pub fn fetch_overlay(parameters: &mut VecDeque<String>, command: &str) -> Result<Option<Overlay>, String> {
    let overlay_on = fetch_option(parameters, "overlay");
    if overlay_on {
        let alpha = require_and_parse(parameters, "alpha", command)?;
        let overlay = Overlay::new(alpha);
        Ok(Some(overlay))
    } else {
        Ok(None)
    }
}
