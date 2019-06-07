extern crate clap;

use clap::{Arg, App};
use webmonitors::config;

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Prints more information while running"))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("Sets a custom config file")
            .takes_value(true)
            .value_name("FILE")
            .index(1)
            .conflicts_with_all(&["url", "email", "ping"]))
        .arg(Arg::with_name("urls")
            .short("u")
            .long("urls")
            .help("Indicates an url to monitor")
            .takes_value(true)
            .value_name("URL")
            .multiple(true)
            .display_order(1))
        .arg(Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .help("Indicates the delay between checks in minutes")
            .takes_value(true)
            .value_name("TIMEOUT")
            .display_order(2)
            .requires("url"))
        .arg(Arg::with_name("emails")
            .short("e")
            .long("emails")
            .help("Indicates an adress to email on changes")
            .takes_value(true)
            .value_name("EMAIL")
            .multiple(true)
            .display_order(3)
            .requires("url"))
        .arg(Arg::with_name("pings")
            .short("p")
            .long("pings")
            .help("Indicates an url to ping on changes")
            .takes_value(true)
            .value_name("URL")
            .multiple(true)
            .display_order(4)
            .requires("url"))
        .after_help(
            "Either pass a config file or pass desired command line options.\n\
            If both are provided, command line options will override or add up to the ones specified in the config file.\n\
            If none are provided, the program will look for a 'example_config.toml' file in the current directory."
        )
        .get_matches();

    let verbose = matches.is_present("verbose");
}