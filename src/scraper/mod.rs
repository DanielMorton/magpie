pub mod row;
mod scrape_page;
pub mod scrape_params;
mod scrape_table;
mod scraper;
mod selectors;
mod table;
mod utils;

pub use scraper::Scraper;

static BASE_URL: &str = "https://ebird.org/targets";
static CHECKLISTS: &str = "checklists";
static COMMON_NAME: &str = "common name";
static COUNTRY: &str = "country";
static END_MONTH: &str = "end month";
static HOME_URL: &str = "https://ebird.org/home";
static HOTSPOT: &str = "hotspot";
static HOTSPOT_COLUMNS: &[&str] = &["country", "region", "sub_region", "hotspot"];
static LOGIN_URL: &str = "https://secure.birds.cornell.edu/cassso/login";
static MAX_BACKOFF: u64 = 200;
static MIN_BACKOFF: u64 = 5;
static PERCENT: &str = "percent";
static REGION: &str = "region";
static REGION_COLUMNS: &[&str] = &["country", "region", "sub_region"];
static START_MONTH: &str = "start month";
static SUB_REGION: &str = "sub_region";
static SCIENTIFIC_NAME: &str = "scientific name";
