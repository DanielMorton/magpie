use scraper::Selector;

static A: &str = "a";
static CHECKLISTS: &str = r#"p[class="u-text-3 u-margin-none"]"#;
static HOTSPOT_SELECT: &str = r#"a[href^="hotspot"]"#;
static NATIVE: &str = r#"section[aria-labelledby="native-and-naturalized"]"#;
static PERCENT: &str = r#"div[class="ResultsStats-stats"]"#;
static REGION_SELECT: &str = r#"a[href^="region"]"#;
static ROW: &str = r#"li[class="ResultsStats ResultsStats--action ResultsStats--toEdge"]"#;
static SCI_NAME: &str = r#"em[class="sci"]"#;
static SPECIES: &str = r#"div[class="SpecimenHeader"]"#;
static SPECIES_COUNT: &str = r#"strong[class="Heading Heading--h1"]"#;

pub(super) struct Selectors {
    a: Selector,
    checklists: Selector,
    hotspot_select: Selector,
    native: Selector,
    percent: Selector,
    region_select: Selector,
    rows: Selector,
    sci_name: Selector,
    species: Selector,
    species_count: Selector,
}

impl Selectors {
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
        let rows = match Selector::parse(ROW) {
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

    pub(super) fn a(&self) -> &Selector {
        &self.a
    }
    pub(super) fn checklists(&self) -> &Selector {
        &self.checklists
    }
    pub(super) fn hotspot_select(&self) -> &Selector {
        &self.hotspot_select
    }
    pub(super) fn native(&self) -> &Selector {
        &self.native
    }
    pub(super) fn percent(&self) -> &Selector {
        &self.percent
    }
    pub(super) fn region_select(&self) -> &Selector {
        &self.region_select
    }
    pub(super) fn rows(&self) -> &Selector {
        &self.rows
    }
    pub(super) fn sci_name(&self) -> &Selector {
        &self.sci_name
    }
    pub(super) fn species(&self) -> &Selector {
        &self.species
    }
    pub(super) fn species_count(&self) -> &Selector {
        &self.species_count
    }
}
