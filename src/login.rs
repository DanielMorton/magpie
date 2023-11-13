use std::io;

use reqwest::blocking::Client;
use rpassword::prompt_password;
use scraper::{Html, Selector};

static LOGIN_URL: &str = "https://secure.birds.cornell.edu/cassso/login";

fn get_token(client: &Client) -> String {
    let response = client.get(LOGIN_URL).send().unwrap().text().unwrap();
    let doc = Html::parse_document(&response);
    let selector = Selector::parse(r#"input[name="lt"]"#).unwrap();
    let token = match doc
        .select(&selector)
        .next()
        .map(|t| t.value())
        .map(|t| t.attr("value"))
        .flatten()
    {
        Some(t) => t.to_string(),
        None => panic!("No Login Token Provided."),
    };
    token
}

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
    return client;
}
