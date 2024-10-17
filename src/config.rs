// Configuration Module
// This module ensures the configuration file is loaded in for the rest of the peripherals.
use serde::Deserialize;
use std::{error::Error, fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub model_name: String,
    pub do_streaming: bool,
    pub system_prompt: String,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
