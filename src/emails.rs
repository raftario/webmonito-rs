extern crate lettre;

use lettre::sendmail::SendmailTransport;
use lettre::{EmailAddress, Envelope, SendableEmail, Transport};
use std::error::Error;

pub fn send(
    sender: &mut SendmailTransport,
    from: &str,
    to: &str,
    message: &str,
    url: &str,
) -> Result<(), Box<dyn Error>> {
    let from = Some(EmailAddress::new(from.to_string())?);
    let to = vec![EmailAddress::new(to.to_string())?];

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
                "example@example.com",
                "example@example.com",
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
                "qwerty",
                "qwerty",
                "test",
                "https://www.google.com/",
            )
            .unwrap();
        }
    }
}
