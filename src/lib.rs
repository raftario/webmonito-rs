extern crate clap;

pub mod config;
pub mod emails;
pub mod pings;
pub mod urls;

use config::Config;
use std::process;

fn verbose_println(log: bool, message: &str) {
    if log {
        println!("{}", message);
    }
}

pub fn run(matches: clap::ArgMatches) {
    println!("- Initial setup...");

    let config = Config::new(matches).unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e.to_string());
        process::exit(1);
    });
    let verbose = config.verbose.unwrap_or(false);
    verbose_println(verbose, "  Config parsed.");

    let mut client = reqwest::Client::new();

    let mut list = urls::hash_list(&config.urls);
    for (k, v) in list.clone().iter() {
        let hash = urls::compare(&client, k, v).unwrap_or_else(|e| {
            eprintln!("Request error for {}: {}", &k, e.to_string());
            None
        });
        if let Some(r) = hash {
            verbose_println(verbose, &format!("  Initial hash for {} obtained.", &k));
            list.insert(k.clone(), r);
        }
    }
}
