use scraper::Selector;
use std::sync::OnceLock;

macro_rules! define_selector {
    ($name:ident, $selector:expr) => {
        pub fn $name() -> &'static Selector {
            static SELECTOR: OnceLock<Selector> = OnceLock::new();
            SELECTOR.get_or_init(|| {
                Selector::parse($selector)
                    .unwrap_or_else(|_| panic!("Failed to parse selector: {}", $selector))
            })
        }
    };
}

pub mod location {
    use super::*;
    define_selector!(a, "a");
    define_selector!(leaderboard, r#"div[class="LeaderBoardSection"]"#);
}

pub mod target {
    use super::*;
    define_selector!(a, "a");
    define_selector!(checklists, r#"p[class="u-text-3 u-margin-none"]"#);
    define_selector!(hotspot_select, r#"a[href^="hotspot"]"#);
    define_selector!(native, r#"section[aria-labelledby="native-and-naturalized"]"#);
    define_selector!(percent, r#"div[class="ResultsStats-stats"]"#);
    define_selector!(region_select, r#"a[href^="region"]"#);
    define_selector!(rows, r#"li[class="ResultsStats ResultsStats--action ResultsStats--toEdge"]"#);
    define_selector!(sci_name, r#"em[class="sci"]"#);
    define_selector!(species, r#"div[class="SpecimenHeader"]"#);
    define_selector!(species_count, r#"strong[class="Heading Heading--h2"]"#);
}