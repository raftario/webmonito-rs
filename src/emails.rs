extern crate lettre;

use lettre::sendmail::SendmailTransport;
use lettre::{EmailAddress, Envelope, SendableEmail, Transport};
use std::error::Error;

pub fn send(
    sender: &mut SendmailTransport,
    from: String,
    to: Vec<String>,
    message: &str,
    url: &str,
) -> Result<(), Box<dyn Error>> {
    let from = Some(EmailAddress::new(from)?);
    let to = to
        .iter()
        .map(|f| EmailAddress::new(f.clone()).unwrap_or(from.clone().unwrap()))
        .collect();

    let email = SendableEmail::new(
        Envelope::new(from, to).unwrap(),
        format!("update-{}", url),
        format!("Webpage at {} has been updated.\n{}", url, message).into_bytes(),
    );

    Ok(sender.send(email)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod send {
        use super::*;

        #[test]
        #[ignore]
        fn valid() {
            let mut sender = SendmailTransport::new();
            send(
                &mut sender,
                "example@example.com".to_string(),
                vec!["example@example.com".to_string()],
                "test",
                "https://www.google.com/",
            )
            .unwrap();
        }

        #[test]
        #[ignore]
        #[should_panic]
        fn invalid() {
            let mut sender = SendmailTransport::new();
            send(
                &mut sender,
                "qwerty".to_string(),
                vec!["qwerty".to_string()],
                "test",
                "https://www.google.com/",
            )
            .unwrap();
        }
    }
}
