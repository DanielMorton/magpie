use lazy_static::lazy_static;
use scraper::Selector;

macro_rules! define_selector {
    ($fn_name:ident, $static_name:ident, $selector:expr) => {
        lazy_static! {
            static ref $static_name: Selector = Selector::parse($selector)
                .unwrap_or_else(|_| panic!("Failed to parse selector: {}", $selector));
        }
        pub fn $fn_name() -> &'static Selector {
            &$static_name
        }
    };
}

pub mod location {
    use super::*;
    define_selector!(a, __LOC_A, "a");
    define_selector!(leaderboard, __LOC_LEADERBOARD, r#"div[class="LeaderBoardSection"]"#);
}

pub mod target {
    use super::*;
    define_selector!(a, __TGT_A, "a");
    define_selector!(checklists, __TGT_CHECKLISTS, r#"p[class="u-text-3 u-margin-none"]"#);
    define_selector!(hotspot_select, __TGT_HOTSPOT_SELECT, r#"a[href^="hotspot"]"#);
    define_selector!(native, __TGT_NATIVE, r#"section[aria-labelledby="native-and-naturalized"]"#);
    define_selector!(percent, __TGT_PERCENT, r#"div[class="ResultsStats-stats"]"#);
    define_selector!(region_select, __TGT_REGION_SELECT, r#"a[href^="region"]"#);
    define_selector!(rows, __TGT_ROWS, r#"li[class="ResultsStats ResultsStats--action ResultsStats--toEdge"]"#);
    define_selector!(sci_name, __TGT_SCI_NAME, r#"em[class="sci"]"#);
    define_selector!(species, __TGT_SPECIES, r#"div[class="SpecimenHeader"]"#);
    define_selector!(species_count, __TGT_SPECIES_COUNT, r#"strong[class="Heading Heading--h2"]"#);
}