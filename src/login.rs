use crate::error::{AppError, Result};
use regex::Regex;
use reqwest::blocking::Client;
use rpassword::prompt_password;
use std::io::{self, Write};
use std::sync::OnceLock;

const LOGIN_URL: &str = "https://secure.birds.cornell.edu/cassso/login";

fn get_token(client: &Client) -> Result<String> {
    let text = client.get(LOGIN_URL).send()?.text()?;

    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r#"name="execution"\s+value="([^"]+)""#).unwrap());

    re.captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_owned())
        .ok_or(AppError::MissingLoginToken)
}

pub fn login() -> Result<Client> {
    let client = Client::builder().cookie_store(true).build()?;
    let token = get_token(&client)?;

    print!("Username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    let password = prompt_password("Password: ")?;

    client
        .post(LOGIN_URL)
        .form(&[
            ("username", username.trim()),
            ("password", &password),
            ("execution", &token),
            ("_eventId", "submit"),
        ])
        .send()?;

    Ok(client)
}
