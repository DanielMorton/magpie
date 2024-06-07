use scraper::Selector;

static A: &str = "a";
static CHECKLISTS: &str = r#"p[class="u-text-3 u-margin-none"]"#;
static HOTSPOT_SELECT: &str = r#"a[href^="hotspot"]"#;
static NATIVE: &str = r#"section[aria-labelledby="native-and-naturalized"]"#;
static PERCENT: &str = r#"div[class="ResultsStats-stats"]"#;
static REGION_SELECT: &str = r#"a[href^="region"]"#;
static ROWS: &str = r#"li[class="ResultsStats ResultsStats--action ResultsStats--toEdge"]"#;
static SCI_NAME: &str = r#"em[class="sci"]"#;
static SPECIES: &str = r#"div[class="SpecimenHeader"]"#;
static SPECIES_COUNT: &str = r#"strong[class="Heading Heading--h2"]"#;

/**
Struct containing all the HTML selectors used when parsing the HTML of the scraped web pages.
*/
pub(super) struct Selectors {
    /// Selects HTML tag "a".
    pub(crate) a: Selector,

    /// Selects the number of checklists for location and time period.
    pub(crate) checklists: Selector,

    /// Selects for the URL of the scraped page if location is a hotspot.
    pub(crate) hotspot_select: Selector,

    /// Selects for all species not exotic of escapees.
    pub(crate) native: Selector,

    /// Selects the frequency of the species observations in percentage form.
    pub(crate) percent: Selector,

    /// Selects for the URL of the scraped page if location is a region.
    pub(crate) region_select: Selector,

    /// Selects individual rows of the table.
    pub(crate) rows: Selector,

    /// Selects the scientific name.
    pub(crate) sci_name: Selector,

    /// Selects the species names, both common and scientific.
    pub(crate) species: Selector,

    /// Selects number of species in extracted table.
    pub(crate) species_count: Selector,
}

/**
 Implementation of the Selectors struct
*/
impl Selectors {
    /**
    Constructs all selectors used for scraping.
    */
    pub(super) fn new() -> Self {
        let a = match Selector::parse(A) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };

        let checklists = match Selector::parse(CHECKLISTS) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        let hotspot_select = match Selector::parse(HOTSPOT_SELECT) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        let native = match Selector::parse(NATIVE) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        let percent = match Selector::parse(PERCENT) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        let region_select = match Selector::parse(REGION_SELECT) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        let rows = match Selector::parse(ROWS) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        let sci_name = match Selector::parse(SCI_NAME) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        let species = match Selector::parse(SPECIES) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        let species_count = match Selector::parse(SPECIES_COUNT) {
            Ok(selector) => selector,
            Err(e) => panic!("{}", e),
        };
        Selectors {
            a,
            checklists,
            hotspot_select,
            native,
            percent,
            region_select,
            rows,
            sci_name,
            species,
            species_count,
        }
    }
}
