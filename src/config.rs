extern crate toml;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

pub enum ConfigError {
    CantOpenFile,
    CantParseConfig,
    CantReadFile,
    NoConfigFile,
    NoHomeDirectory,
}

#[derive(Debug)]
pub struct SlothConfig {
    value: BTreeMap<String, toml::Value>,
}

impl SlothConfig {
    pub fn new() -> Result<SlothConfig, ConfigError> {
        let mut config_file_path = match env::home_dir() {
            Some(path) => path,
            None => return Err(ConfigError::NoHomeDirectory),
        };

        config_file_path.push(".sloth.toml");

        if !config_file_path.is_file() {
            return Err(ConfigError::NoConfigFile);
        }

        let mut f = match File::open(config_file_path) {
            Ok(f) => f,
            Err(_) => return Err(ConfigError::CantOpenFile),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => SlothConfig::from_raw_input(s),
            Err(_) => Err(ConfigError::CantReadFile),
        }
    }

    fn from_raw_input(raw_input: String) -> Result<SlothConfig, ConfigError> {
        match toml::Parser::new(&raw_input).parse() {
            Some(value) => Ok(SlothConfig { value: value }),
            _ => Err(ConfigError::CantParseConfig),
        }
    }
}
