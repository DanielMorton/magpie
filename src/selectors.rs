use scraper::Selector;

static A: &str = "a";
static NATIVE: &str = r#"section[aria-labelledby="native-and-naturalized"]"#;
static PERCENT: &str = r#"div[class="ResultsStats-stats"]"#;
static ROW: &str = r#"li[class="ResultsStats ResultsStats--action ResultsStats--toEdge"]"#;
static SCI_NAME: &str = r#"em[class="sci"]"#;
static SPECIES: &str = r#"div[class="SpecimenHeader"]"#;
static SPECIES_COUNT: &str = r#"strong[class="Heading Heading--h1"]"#;

pub(crate) struct Selectors {
    a: Selector,
    native: Selector,
    percent: Selector,
    rows: Selector,
    sci_name: Selector,
    species: Selector,
    species_count: Selector,
}

impl Selectors {
    pub(crate) fn new() -> Self {
        let a = match Selector::parse(A) {
            Ok(seletor) => seletor,
            Err(e) => panic!("{}", e),
        };
        let native = match Selector::parse(NATIVE) {
            Ok(seletor) => seletor,
            Err(e) => panic!("{}", e),
        };
        let percent = match Selector::parse(PERCENT) {
            Ok(seletor) => seletor,
            Err(e) => panic!("{}", e),
        };
        let rows = match Selector::parse(ROW) {
            Ok(seletor) => seletor,
            Err(e) => panic!("{}", e),
        };
        let sci_name = match Selector::parse(SCI_NAME) {
            Ok(seletor) => seletor,
            Err(e) => panic!("{}", e),
        };
        let species = match Selector::parse(SPECIES) {
            Ok(seletor) => seletor,
            Err(e) => panic!("{}", e),
        };
        let species_count = match Selector::parse(SPECIES_COUNT) {
            Ok(seletor) => seletor,
            Err(e) => panic!("{}", e),
        };
        Selectors {
            a,
            native,
            percent,
            rows,
            sci_name,
            species,
            species_count,
        }
    }

    pub(crate) fn a(&self) -> &Selector {
        &self.a
    }
    pub(crate) fn native(&self) -> &Selector {
        &self.native
    }
    pub(crate) fn percent(&self) -> &Selector {
        &self.percent
    }
    pub(crate) fn rows(&self) -> &Selector {
        &self.rows
    }
    pub(crate) fn sci_name(&self) -> &Selector {
        &self.sci_name
    }
    pub(crate) fn species(&self) -> &Selector {
        &self.species
    }
    pub(crate) fn species_count(&self) -> &Selector {
        &self.species_count
    }
}
