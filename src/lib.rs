extern crate clap;

pub mod config;
pub mod emails;
pub mod pings;
pub mod urls;

use config::Config;
use lettre::sendmail::SendmailTransport;
use std::{process, thread, time};

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

    let mut list = urls::hash_list(&config.urls);

    {
        let client = reqwest::Client::new();

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

    loop {
        {
            let client = reqwest::Client::new();
            let mut transport = SendmailTransport::new();

            println!("- Checking for updates...");
            for (k, v) in list.clone().iter() {
                let hash = urls::compare(&client, k, v).unwrap_or_else(|e| {
                    eprintln!("Request error for {}: {}", &k, e.to_string());
                    None
                });

                if let Some(r) = hash {
                    println!("  Page at {} has changed", &k);
                    list.insert(k.clone(), r);

                    if let Some(p) = &config.pings {
                        for ping in p {
                            let response = pings::send(
                                &client,
                                &ping.url,
                                match &ping.content {
                                    Some(c) => c,
                                    None => "",
                                },
                                k,
                            )
                            .unwrap_or_else(|e| {
                                eprintln!("Ping error for {}: {}", &ping.url, e.to_string());
                                String::new()
                            });

                            verbose_println(
                                verbose,
                                &format!("  Ping sent to {}. Response is {}", &ping.url, response),
                            );
                        }
                    }

                    if let Some(e) = &config.emails {
                        for email in e {
                            emails::send(
                                &mut transport,
                                match &config.sender {
                                    Some(s) => s,
                                    None => "wbmrs@localhost",
                                },
                                &email.address,
                                match &email.content {
                                    Some(c) => c,
                                    None => "",
                                },
                                k,
                            )
                            .unwrap_or_else(|e| {
                                eprintln!("Email error for {}: {}", &email.address, e.to_string());
                            });

                            verbose_println(
                                verbose,
                                &format!("  Email sent to {}.", &email.address),
                            );
                        }
                    }
                } else {
                    verbose_println(verbose, &format!("  Page at {} has not changed", &k));
                }
            }
        }

        thread::sleep(time::Duration::from_secs((config.timeout * 60) as u64));
    }
}
