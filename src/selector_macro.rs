/// Defines a `Selectors` struct with one static, lazily-parsed CSS `Selector` per entry,
/// exposed as a zero-argument associated function (e.g. `Selectors::a()`).
///
/// This macro is shared by `location::selectors` and `target::selectors` so the two
/// modules don't each maintain their own copy of the same boilerplate.
macro_rules! define_selectors {
    ($($name:ident => $selector:expr),+ $(,)?) => {
        lazy_static::lazy_static! {
            static ref SELECTORS: std::collections::HashMap<&'static str, scraper::Selector> = {
                let mut m = std::collections::HashMap::new();
                $(
                    m.insert(stringify!($name), scraper::Selector::parse($selector)
                        .unwrap_or_else(|_| panic!("Failed to parse '{}' selector", stringify!($name))));
                )+
                m
            };
        }

        pub struct Selectors;

        impl Selectors {
            $(
                pub fn $name() -> &'static scraper::Selector {
                    SELECTORS.get(stringify!($name))
                        .expect(concat!("Selector '", stringify!($name), "' not found"))
                }
            )+
        }
    };
}

pub(crate) use define_selectors;