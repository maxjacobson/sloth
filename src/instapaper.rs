use config::SlothConfig;
extern crate toml;
use std::collections::BTreeMap;
extern crate hyper;

pub struct InstapaperApp {
    consumer_key: String,
    consumer_secret: String,
    username: String,
    password: String,
}

pub enum InstapaperConfigError {
    MissingValue, // TODO: include a value in this variant which tells you which value is missing
    NoInstapaperConfig,
    TypeError,
}

impl InstapaperApp {
    pub fn new(config: SlothConfig) -> Result<InstapaperApp, InstapaperConfigError> {
        let instapaper_section =
            try!(config.value.get("instapaper").ok_or(InstapaperConfigError::NoInstapaperConfig));
        let instapaper_config = try!(instapaper_section.as_table()
            .ok_or(InstapaperConfigError::TypeError));
        Ok(InstapaperApp {
            consumer_key: try!(InstapaperApp::extract_string_from_table(instapaper_config,
                                                                        "consumer_key")),
            consumer_secret: try!(InstapaperApp::extract_string_from_table(instapaper_config,
                                                                           "consumer_secret")),
            username: try!(InstapaperApp::extract_string_from_table(instapaper_config, "username")),
            password: try!(InstapaperApp::extract_string_from_table(instapaper_config, "password")),
        })
    }

    fn extract_string_from_table(table: &BTreeMap<String, toml::Value>,
                                 key: &str)
                                 -> Result<String, InstapaperConfigError> {
        Ok(try!(try!(table.get(key).ok_or(InstapaperConfigError::MissingValue))
                .as_str()
                .ok_or(InstapaperConfigError::TypeError))
            .to_owned())
    }

    pub fn start(&self) {
        println!("Starting Instapaper...");
        let client = hyper::client::Client::new();
        // FIXME: use the form url encoding feature of hyper...
        let body = format!("x_auth_username={}&x_auth_password={}&x_auth_mode=client_auth",
                           self.username,
                           self.password);
        let mut headers = hyper::header::Headers::new();

        let post_request = client.post("https://instapaper.com/api/1/oauth/access_token");
        // Currently getting a forbidden response!
        match post_request.headers(headers).body(&body).send() {
            Ok(response) => println!("{:?}", response),
            Err(err) => println!("{:?}", err),
        }
    }
}
