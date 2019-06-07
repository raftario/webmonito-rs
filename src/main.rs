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
            .conflicts_with_all(&["url", "email", "ping"]))
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .help("Indicates an url to monitor")
            .takes_value(true)
            .value_name("URL")
            .multiple(true))
        .arg(Arg::with_name("email")
            .short("e")
            .long("email")
            .help("Indicates an adress to email on changes")
            .takes_value(true)
            .value_name("EMAIL")
            .multiple(true))
        .arg(Arg::with_name("ping")
            .short("p")
            .long("ping")
            .help("Indicates an url to ping on changes")
            .takes_value(true)
            .value_name("URL")
            .multiple(true))
        .get_matches();

    let verbose = matches.is_present("verbose");
}