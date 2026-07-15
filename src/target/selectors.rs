use crate::selector_macro::define_selectors;

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
