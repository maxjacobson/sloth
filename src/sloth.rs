use instapaper::InstapaperApp;
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
            Err(ConfigError::CantReadFile) => Err(String::from("Couldn't read ~/.sloth.toml")),
            Err(ConfigError::NoConfigFile) => Err(String::from("Must create ~/.sloth.toml")),
            Err(ConfigError::NoHomeDirectory) => Err(String::from("Can't identify home directory")),
        }
    }

    pub fn print_status(&self) {
        println!("Not much going on yet!");
    }

    pub fn receive_input(&self, input: String) -> bool {
        if input == String::from("help") {
            self.print_help();
        } else if input == String::from("instapaper") {
            InstapaperApp::new().start()
        } else if input == String::from("exit") {
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
