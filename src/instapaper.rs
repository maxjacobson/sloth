use config::SlothConfig;

#[derive(Debug)]
pub struct InstapaperApp {
    config: SlothConfig,
}

pub enum InstapaperConfigError {
}

impl InstapaperApp {
    pub fn new(config: SlothConfig) -> Result<InstapaperApp, InstapaperConfigError> {
        // FIXME: validate the config and return errs for bad config
        Ok(InstapaperApp { config: config })
    }

    pub fn start(&self) {
        println!("Starting Instapaper... {:?}", self);
    }
}
