extern crate clap;
extern crate serde_derive;
extern crate toml;

pub mod config {
    use std::error::Error;
    use std::fmt::Debug;
    use std::fs;
    use clap::ArgMatches;
    use toml;
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Email {
        address: String,
        content: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct Ping {
        url: String,
        content: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Config {
        verbose: Option<bool>,
        timeout: u32,
        urls: Vec<String>,
        emails: Option<Vec<Email>>,
        pings: Option<Vec<Ping>>,
    }

    impl Config {
        fn from_defaults() -> Config {
            Config {
                verbose: Some(false),
                timeout: 60,
                urls: vec![],
                emails: None,
                pings: None,
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
            } else if let None = config.verbose {
                config.verbose = Some(false);
            }

            // Override timeout
            if matches.is_present("timeout") {
                config.timeout = matches.value_of("timeout")
                    .unwrap()
                    .parse()?;
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
                    config.emails = Some(vec![]);
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
                    config.pings = Some(vec![]);
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
                let config = Config::from_file("Cargo.toml")
                    .expect("Config parsing from invalid file failed");
                println!("Config: {:#?}", config);
            }

            #[test]
            fn ok() {
                let config = Config::from_file("example_config.toml")
                    .expect("Config parsing from valid file failed");
                println!("Config: {:#?}", config);
            }
        }

        mod from_defaults {
            use super::*;

            #[test]
            fn ok() {
                let config = Config::from_defaults();
                println!("Config: {:#?}", config);
                assert_eq!(config.verbose, Some(false));
                assert_eq!(config.timeout, 60);

                let urls: Vec<String> = vec![];
                assert_eq!(config.urls, urls);
            }
        }
    }
}
