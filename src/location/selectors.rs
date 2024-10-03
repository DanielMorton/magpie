use lazy_static::lazy_static;
use scraper::Selector;
use std::collections::HashMap;

macro_rules! define_selectors {
    ($($name:ident => $selector:expr),+ $(,)?) => {
        lazy_static! {
            pub static ref SELECTORS: HashMap<&'static str, Selector> = {
                let mut m = HashMap::new();
                $(
                    m.insert(stringify!($name), Selector::parse($selector)
                        .unwrap_or_else(|_| panic!("Failed to parse '{}' selector", stringify!($name))));
                )+
                m
            };
        }

        pub struct Selectors;

        impl Selectors {
            $(
                pub fn $name() -> &'static Selector {
                    SELECTORS.get(stringify!($name))
                        .expect(concat!("Selector '", stringify!($name), "' not found"))
                }
            )+
        }
    };
}

define_selectors! {
    a => "a",
    leaderboard => r#"div[class="LeaderBoardSection"]"#
}