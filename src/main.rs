use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

fn main() {
    let from: String = env::var("ETHMAIL_TESTER_FROM").unwrap().into();
    let to: String = env::var("ETHMAIL_TESTER_TO").unwrap().into();
    let username: String = env::var("ETHMAIL_TESTER_USERNAME").unwrap().into();
    let password: String = env::var("ETHMAIL_TESTER_PASSWORD").unwrap().into();

    let email = Message::builder()
        .from(from.parse().unwrap())
        //.reply_to("xunkulapchvatal@ethmail.cc".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("ETHMail Tester")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Hello from ETHMail Tester!"))
        .unwrap();

    let creds = Credentials::new(username.to_owned(), password.to_owned());

    // Open a remote connection
    let mailer = SmtpTransport::starttls_relay("ethmail.cc")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
