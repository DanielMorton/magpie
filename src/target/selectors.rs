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
    checklists => r#"p[class="u-text-3 u-margin-none"]"#,
    hotspot_select => r#"a[href^="hotspot"]"#,
    native => r#"section[aria-labelledby="native-and-naturalized"]"#,
    percent => r#"div[class="ResultsStats-stats"]"#,
    region_select => r#"a[href^="region"]"#,
    rows => r#"li[class="ResultsStats ResultsStats--action ResultsStats--toEdge"]"#,
    sci_name => r#"em[class="sci"]"#,
    species => r#"div[class="SpecimenHeader"]"#,
    species_count => r#"strong[class="Heading Heading--h2"]"#,
}
