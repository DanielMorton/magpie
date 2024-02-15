use std::io;

use reqwest::blocking::Client;
use rpassword::prompt_password;
use scraper::{Html, Selector};

static LOGIN_URL: &str = "https://secure.birds.cornell.edu/cassso/login";
static TOKEN: &str = r#"input[name="lt"]"#;

/**
Extracts the random token needed to log in to eBird.
*/
fn get_token(client: &Client) -> String {
    let response = client.get(LOGIN_URL).send().unwrap().text().unwrap();
    let doc = Html::parse_document(&response);
    let selector = Selector::parse(TOKEN).unwrap();
    let token = match doc
        .select(&selector)
        .next()
        .map(|t| t.value())
        .and_then(|t| t.attr("value"))
    {
        Some(t) => t.to_string(),
        None => panic!("No Login Token Provided."),
    };
    token
}

/**
Creates an eBird login session.
*/
pub(crate) fn login() -> Client {
    let client = Client::builder().cookie_store(true).build().unwrap();
    let token = get_token(&client);
    println!("Username:");
    let mut username = String::new();
    let _ = io::stdin().read_line(&mut username).unwrap();
    let password = prompt_password("Password:\n ").unwrap();
    let login_data = vec![
        ("username", username.trim().to_string()),
        ("password", password),
        ("lt", token),
        ("execution", "e1s1".to_string()),
        ("_eventId", "submit".to_string()),
    ];

    match client.post(LOGIN_URL).form(&login_data).send() {
        Ok(r) => r,
        Err(e) => panic!("Login Failed.\n {}", e),
    };
    client
}
