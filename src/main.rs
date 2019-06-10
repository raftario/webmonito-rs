extern crate clap;

use clap::{Arg, App};
use wbmrs::config::Config;

fn main() {
    let matches = App::new("webmonito-rs")
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
            .index(1))
        .arg(Arg::with_name("urls")
            .short("u")
            .long("urls")
            .help("Indicates an url to monitor")
            .takes_value(true)
            .value_name("URL")
            .multiple(true)
            .display_order(1)
            .required_unless("config"))
        .arg(Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .help("Indicates the delay between checks in minutes")
            .takes_value(true)
            .value_name("TIMEOUT")
            .display_order(2)
            .required_unless("config"))
        .arg(Arg::with_name("emails")
            .short("e")
            .long("emails")
            .help("Indicates an adress to email on changes")
            .takes_value(true)
            .value_name("EMAIL")
            .multiple(true)
            .display_order(3))
        .arg(Arg::with_name("pings")
            .short("p")
            .long("pings")
            .help("Indicates an url to ping on changes")
            .takes_value(true)
            .value_name("URL")
            .multiple(true)
            .display_order(4))
        .after_help(
            "Either pass a config file or pass desired command line options.\n\
            If both are provided, command line options will override or add up to the ones specified in the config file."
        )
        .get_matches();

    let config = Config::new(matches).unwrap();
    println!("Config: {:#?}", config);

    let verbose = config.verbose;
    let timeout = config.timeout;
    let urls = config.urls;
    let emails = config.emails;
    let pings = config.pings;
    println!("verbose: {:#?}", verbose);
    println!("timeout: {:#?}", timeout);
    println!("urls: {:#?}", urls);
    println!("emails: {:#?}", emails);
    println!("pings: {:#?}", pings);
    println!("urls: {:#?}", wbmrs::urls::hash_list(&urls));
}