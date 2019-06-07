extern crate toml;
extern crate serde_derive;

pub mod config {
    use std::fs;
    use std::error::Error;
    use toml;
    use serde_derive::Deserialize;
    use std::fmt::Debug;

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
        urls: Vec<String>,
        emails: Option<Vec<Email>>,
        pings: Option<Vec<Ping>>,
    }

    impl Config {
        pub fn from_file(path: &str, verbose: bool) -> Result<Config, Box<dyn Error>> {
            let contents = fs::read_to_string(path)?;
            let mut config: Config = toml::from_str(&contents)?;
            // If verbose is not specified in config file, use command line argument value.
            config.verbose = Some(config.verbose.unwrap_or(verbose));

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
                let config = Config::from_file("missing.toml", false)
                    .expect("Config parsing from missing file failed");
                println!("Config: {:#?}", config);
            }

            #[test]
            #[should_panic]
            fn invalid() {
                let config = Config::from_file("Cargo.toml", false)
                    .expect("Config parsing from invalid file failed");
                println!("Config: {:#?}", config);
            }

            #[test]
            fn ok() {
                let config = Config::from_file("config.toml", false)
                    .expect("Config parsing from valid file failed");
                println!("Config: {:#?}", config);
            }
        }
    }
}
