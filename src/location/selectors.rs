use scraper::Selector;

static A: &str = "a";

static LEADERBOARD: &str = r#"div[class="LeaderBoardSection"]"#;

pub struct Selectors {
    /// Selects HTML tag "a".
    pub(crate) a: Selector,

    /// Selects the table to scrape.
    pub(crate) leaderboard: Selector,
}

impl Selectors {
    pub fn new() -> Self {
        let a = match Selector::parse(A) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };

        let leaderboard = match Selector::parse(LEADERBOARD) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        Selectors { a, leaderboard }
    }
}
