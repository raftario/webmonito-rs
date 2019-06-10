extern crate crypto;
extern crate reqwest;

use std::collections::HashMap;
use std::error::Error;
use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn hash_list(urls: &Vec<String>) -> HashMap<String, String> {
    let mut list = HashMap::new();

    for url in urls {
        list.insert(
            url.clone(),
            String::new(),
        );
    }

    list
}

fn contents(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url)?.text()?)
}

fn compare(url: &str, hash: &str) -> Result<Option<String>, Box<dyn Error>> {
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

pub fn compare_all(list: &mut HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    for (k, v) in list.clone().iter() {
        if let Some(r) = compare(k, v)? {
            list.insert(k.clone(), r);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod hash_list {
        use super::*;

        #[test]
        fn valid() {
            let urls = vec![
                "https://www.rust-lang.org/".to_string(),
                "https://docs.rs/".to_string(),
            ];
            let mut expected = HashMap::new();
            expected.insert(
                "https://www.rust-lang.org/".to_string(),
                String::new(),
            );
            expected.insert(
                "https://docs.rs/".to_string(),
                String::new(),
            );

            assert_eq!(hash_list(&urls), expected)
        }
    }

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

    mod compare_all {
        use super::*;

        #[test]
        #[ignore]
        fn different() {
            let mut list = HashMap::new();
            list.insert(
                "https://www.rust-lang.org/".to_string(),
                String::new(),
            );
            list.insert(
                "https://docs.rs/".to_string(),
                String::new(),
            );
            let old_list = list.clone();

            compare_all(&mut list).unwrap();
            println!("New list: {:#?}", list);

            assert_ne!(list, old_list);
        }
    }
}