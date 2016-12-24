use config::SlothConfig;
extern crate hyper;

#[derive(Debug)]
pub struct InstapaperApp {
    consumer_key: String,
    consumer_secret: String,
    username: String,
    password: String,
}

pub enum InstapaperConfigError {
    NoConsumerKey,
    NoConsumerSecret,
    NoInstapaperConfig,
    TypeError,
}

impl InstapaperApp {
    // FIXME: figure out idiomatic way to write something like this
    // I wrote it out handling all possible paths to get a feel for
    // handling all paths the painful way, to motivate me to learn
    // the rigth way...
    pub fn new(config: SlothConfig) -> Result<InstapaperApp, InstapaperConfigError> {
        match config.value.get("instapaper") {
            Some(instapaper) => {
                match instapaper.as_table() {
                    Some(table) => {
                        match table.get("consumer_key") {
                            Some(consumer_key) => {
                                match table.get("consumer_secret") {
                                    Some(consumer_secret) => {
                                        match consumer_key.as_str() {
                                            Some(consumer_key) => {
                                                match consumer_secret.as_str() {
                                                    Some(consumer_secret) => {
                                                        Ok(InstapaperApp {
                                                            consumer_key: consumer_key.to_owned(),
                                                            consumer_secret: consumer_secret.to_owned(),
                                                            username: String::from("TKTK"),
                                                            password: String::from("TKTK"),
                                                        })
                                                    },
                                                    None => Err(InstapaperConfigError::TypeError),
                                                }
                                            },
                                            None => Err(InstapaperConfigError::TypeError),
                                        }
                                    },
                                    None => Err(InstapaperConfigError::NoConsumerSecret),
                                }
                            },
                            None => Err(InstapaperConfigError::NoConsumerKey),
                        }
                    },
                    None => Err(InstapaperConfigError::TypeError),
                }
            },
            None => Err(InstapaperConfigError::NoInstapaperConfig),
        }
    }

    pub fn start(&self) {
        println!("Starting Instapaper... {:?}", self);
        let client = hyper::client::Client::new();
        let body = format!("x_auth_username={}&x_auth_password={}&x_auth_mode=client_auth", self.username, self.password);
        let post_request = client.post("https://instapaper.com/api/1/oauth/access_token");
        // Currently getting a forbidden response!
        match post_request.body(&body).send() {
            Ok(response) => println!("{:?}", response),
            Err(err) => println!("{:?}", err),
        }
    }
}
