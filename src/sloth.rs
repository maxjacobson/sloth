use instapaper::{InstapaperApp, InstapaperConfigError};
use config::SlothConfig;
use config::ConfigError;

#[derive(Debug)]
pub struct SlothApp {
    config: SlothConfig,
}

impl SlothApp {
    pub fn new() -> Result<SlothApp, String> {
        match SlothConfig::new() {
            Ok(config) => Ok(SlothApp { config: config }),
            Err(ConfigError::CantOpenFile) => Err(String::from("Couldn't open ~/.sloth.toml")),
            Err(ConfigError::CantParseConfig) => {
                Err(String::from("Couldn't parse ~/.sloth.toml as toml"))
            }
            Err(ConfigError::CantReadFile) => Err(String::from("Couldn't read ~/.sloth.toml")),
            Err(ConfigError::NoConfigFile) => Err(String::from("Must create ~/.sloth.toml")),
            Err(ConfigError::NoHomeDirectory) => Err(String::from("Can't identify home directory")),
        }
    }

    pub fn print_status(&self) {
        println!("Not much going on yet!");
    }

    pub fn receive_input(&self, input: String) -> bool {
        if input == "help" {
            self.print_help();
        } else if input == "instapaper" {
            match InstapaperApp::new(self.config.clone()) {
                Ok(instapaper_app) => instapaper_app.start(),
                Err(InstapaperConfigError::MissingValue) => {
                    println!("Make sure your [instapaper] section has all the values it needs.");
                    return true;
                },
                Err(InstapaperConfigError::NoInstapaperConfig) => {
                    println!("Make sure to add an [instapaper] section to your ~/.sloth.toml");
                    return true;
                },
                Err(InstapaperConfigError::TypeError) => {
                    println!("Make sure your instapaper config has correct types");
                    return true;
                },
            }
        } else if input == "exit" {
            return true;
        } else {
            println!("Unrecognized input");
        }

        false
    }

    fn print_help(&self) {
        println!("Help text will go here...");
    }
}
