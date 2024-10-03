use std::error::Error;
use std::io::{self, Write};

use reqwest::blocking::Client;
use rpassword::prompt_password;
use scraper::{Html, Selector};

const LOGIN_URL: &str = "https://secure.birds.cornell.edu/cassso/login";
const TOKEN_SELECTOR: &str = r#"input[name="lt"]"#;

fn get_token(client: &Client) -> Result<String, Box<dyn Error>> {
    let response = client.get(LOGIN_URL).send()?.text()?;
    let doc = Html::parse_document(&response);
    let selector = Selector::parse(TOKEN_SELECTOR)?;

    doc.select(&selector)
        .next()
        .and_then(|t| t.value().attr("value"))
        .map(ToString::to_string)
        .ok_or_else(|| Box::<dyn Error>::from("No Login Token Provided"))
}

pub(crate) fn login() -> Result<Client, Box<dyn Error>> {
    let client = Client::builder().cookie_store(true).build()?;
    let token = get_token(&client)?;

    print!("Username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    let password = prompt_password("Password: ")?;

    let login_data = [
        ("username", username.trim()),
        ("password", &password),
        ("lt", &token),
        ("execution", "e1s1"),
        ("_eventId", "submit"),
    ];

    client.post(LOGIN_URL).form(&login_data).send()?;
    Ok(client)
}
