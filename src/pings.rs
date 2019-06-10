extern crate reqwest;

use std::collections::HashMap;
use std::error::Error;

pub fn send(
    client: &reqwest::Client,
    ping_url: &str,
    message: &str,
    url: &str,
) -> Result<String, Box<dyn Error>> {
    let mut json = HashMap::new();
    json.insert("url", url);
    json.insert("message", message);

    Ok(client.post(ping_url).json(&json).send()?.text()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod send {
        use super::*;

        #[test]
        #[ignore]
        fn valid() {
            let client = reqwest::Client::new();
            let result = send(
                &client,
                "https://api.github.com/",
                "Ping!",
                "https://www.google.com/",
            )
            .unwrap();
            println!("Response: {}", result);
        }

        #[test]
        #[ignore]
        #[should_panic]
        fn invalid() {
            let client = reqwest::Client::new();
            let result = send(&client, "qwerty", "Ping!", "https://www.google.com/").unwrap();
            println!("Response: {}", result);
        }
    }
}
