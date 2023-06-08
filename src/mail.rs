use smtp::client::net::NetworkStream;
use smtp::client::Client;
use smtp::client::ClientSecurity;
use smtp::commands::{DATA, MAIL, RCPT};
use smtp::error::Error;
use smtp::extension::ClientId;
use smtp::extension::ClientId;
use smtp::extension::Extension;
use smtp::response::Response;
use std::io::Write;

fn send_mail(to: &str, subject: &str, body: &str) -> Result<(), Error> {
    let mut client = Client::new_plain("smtp.gmail.com", ClientSecurity::None)?;
    client.set_hello(ClientId::Domain("example.com".to_string()));
    client.authenticate_plain("username", "password")?;

    client.command(MAIL.format(&["from@example.com"]))?.check();
    client.command(RCPT.format(&[to]))?.check();
    client.command(DATA)?.check();
    client
        .message()
        .unwrap()
        .write_all(format!("Subject: {}\r\n\r\n{}", subject, body).as_bytes())?;
    client.message_end()?.check();
    client.quit()?;

    Ok(())
}
