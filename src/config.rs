extern crate clap;
extern crate serde_derive;
extern crate toml;

use clap::ArgMatches;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Email {
    pub address: String,
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Ping {
    pub url: String,
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub verbose: Option<bool>,
    pub timeout: u32,
    pub urls: Vec<String>,
    pub emails: Option<Vec<Email>>,
    pub pings: Option<Vec<Ping>>,
    pub sender: Option<String>,
}

impl Config {
    fn from_defaults() -> Config {
        Config {
            verbose: Some(false),
            timeout: 60,
            urls: Vec::new(),
            emails: None,
            pings: None,
            sender: None,
        }
    }

    fn from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    pub fn new(matches: ArgMatches) -> Result<Config, Box<dyn Error>> {
        let mut config = if matches.is_present("config") {
            Config::from_file(matches.value_of("config").unwrap())?
        } else {
            Config::from_defaults()
        };

        // Override verbose
        if matches.is_present("verbose") {
            config.verbose = Some(true);
        }

        // Override timeout
        if matches.is_present("timeout") {
            config.timeout = matches.value_of("timeout").unwrap().parse()?;
        }

        // Append urls
        if matches.is_present("urls") {
            let urls: Vec<&str> = matches.values_of("urls").unwrap().collect();
            for url in urls {
                config.urls.push(url.to_string())
            }
        }

        // Append emails
        if matches.is_present("emails") {
            if let None = config.emails {
                config.emails = Some(Vec::new());
            }

            let mut emails = config.emails.unwrap();
            let new_emails: Vec<&str> = matches.values_of("emails").unwrap().collect();
            for email in new_emails {
                emails.push(Email {
                    address: email.to_string(),
                    content: None,
                });
            }
            config.emails = Some(emails);
        }

        // Append pings
        if matches.is_present("pings") {
            if let None = config.pings {
                config.pings = Some(Vec::new());
            }

            let mut pings = config.pings.unwrap();
            let new_pings: Vec<&str> = matches.values_of("pings").unwrap().collect();
            for ping in new_pings {
                pings.push(Ping {
                    url: ping.to_string(),
                    content: None,
                });
            }
            config.pings = Some(pings);
        }

        // Override sender
        if matches.is_present("sender") {
            config.sender = Some(matches.value_of("timeout").unwrap().to_string());
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_file {
        use super::*;

        #[test]
        #[should_panic]
        fn missing() {
            let config = Config::from_file("missing_config.toml")
                .expect("Config parsing from missing file failed");
            println!("Config: {:#?}", config);
        }

        #[test]
        #[should_panic]
        fn invalid() {
            let config =
                Config::from_file("Cargo.toml").expect("Config parsing from invalid file failed");
            println!("Config: {:#?}", config);
        }

        #[test]
        fn valid() {
            let config = Config::from_file("example_config.toml")
                .expect("Config parsing from valid file failed");
            println!("Config: {:#?}", config);
        }
    }

    mod from_defaults {
        use super::*;

        #[test]
        fn valid() {
            let config = Config::from_defaults();
            println!("Config: {:#?}", config);
            assert_eq!(config.verbose, Some(false));
            assert_eq!(config.timeout, 60);

            let urls: Vec<String> = Vec::new();
            assert_eq!(config.urls, urls);

            if let None = config.emails {
            } else {
                panic!("Emails should be empty")
            }

            if let None = config.pings {
            } else {
                panic!("Pings should be empty")
            }

            if let None = config.sender {
            } else {
                panic!("Sender should be empty")
            }
        }
    }
}
