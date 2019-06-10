extern crate crypto;
extern crate reqwest;

use std::error::Error;
use crypto::digest::Digest;
use crypto::sha1::Sha1;

fn contents(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(
        reqwest::get(url)?
        .text()?
    )
}

pub fn compare(url: &str, hash: &str) -> Result<Option<String>, Box<dyn Error>> {
    let new_contents = contents(url)?;
    let mut hasher = Sha1::new();

    hasher.input_str(&new_contents);

    let new_hash = hasher.result_str();

    if new_hash == hash {
        Ok(None)
    } else {
        Ok(Some(new_hash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod contents {
        use super::*;

        #[test]
        #[ignore]
        fn valid() {
            let result = contents("https://www.rust-lang.org/").unwrap();
            println!("Contents: {}", result);
        }

        #[test]
        #[ignore]
        #[should_panic]
        fn invalid() {
            let result = contents("qwerty").unwrap();
            println!("Contents: {}", result);
        }
    }

    mod compare {
        use super::*;

        #[test]
        #[ignore]
        fn same() {
            let rust_hash = compare("https://www.rust-lang.org/", "")
                .unwrap()
                .unwrap();

            println!("Hash: {}", rust_hash);

            let comparison = compare("https://www.rust-lang.org/", &rust_hash)
                .unwrap();

            match comparison {
                Some(h) => {
                    println!("New hash: {}", h);
                    panic!("Hash should be the same");
                },
                None => {
                    println!("New hash: {}", rust_hash);
                },
            }
        }

        #[test]
        #[ignore]
        fn different() {
            let rust_hash = compare("https://www.rust-lang.org/", "")
                .unwrap()
                .unwrap();

            println!("Hash: {}", rust_hash);

            let comparison = compare("https://docs.rs/", &rust_hash)
                .unwrap();

            match comparison {
                Some(h) => {
                    println!("New hash: {}", h);
                },
                None => {
                    println!("New hash: {}", rust_hash);
                    panic!("Hash should be different")
                },
            }
        }
    }
}