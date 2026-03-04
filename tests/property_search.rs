use scraper::{Html, Selector};
use core::panic;
use std::collections::HashSet;

const URL: &str = "https://www.rightmove.co.uk/property-for-sale/find.html?";
const POSTCODE: &str = "searchLocation=SW1A+2AA";
const USE_LOCATION_IDENTIFIER: &str = "&useLocationIdentifier=true";
const LOCATION_IDENTIFIER: &str = "&locationIdentifier=POSTCODE%5E1246000";
const RADIUS: &str = "&radius=0.25";
const MIN_PRICE: &str = "&minPrice=100000";
const MAX_PRICE: &str = "&maxPrice=10000000";
const MIN_BEDROOMS: &str = "&minBedrooms=0";
const MAX_BEDROOMS: &str = "&maxBedrooms=6";
const INCLUDESSTC: &str = "&_includeSSTC=on";

fn get_number_of_properties() -> Result<usize, Box<dyn std::error::Error>> {
    let search_url = format!(
        "{URL}{POSTCODE}{USE_LOCATION_IDENTIFIER}{LOCATION_IDENTIFIER}{RADIUS}{MIN_PRICE}{MAX_PRICE}{MIN_BEDROOMS}{MAX_BEDROOMS}{INCLUDESSTC}" 
    );

    // Fetch HTML
    let body: String = ureq::get(search_url.clone()).call()?.body_mut().read_to_string()?;

    // Parse HTML
    let document = Html::parse_document(&body);
    
    // CSS selector for the anchor elements
    let selector =
        Selector::parse(r#"div[class^="ResultsCount_resultsCount"] span"#).expect("Invalid selector");
    
    let element = document
        .select(&selector)
        .next()
        .ok_or("results count element not found")?;

    let text = element
        .text()
        .next();
        // .ok_or("results count text missing")?;

    
    let count = match text {
        Some(text) => text.trim().parse::<usize>()?,
        None => 0usize,
    };

    Ok(count)

    // Ok(0)

}

fn get_rightmove_properties() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let search_url = format!(
        "{URL}{POSTCODE}{USE_LOCATION_IDENTIFIER}{LOCATION_IDENTIFIER}{RADIUS}{MIN_PRICE}{MAX_PRICE}{MIN_BEDROOMS}{MAX_BEDROOMS}{INCLUDESSTC}" 
    );

    // Fetch HTML
    let body: String = ureq::get(search_url.clone()).call()?.body_mut().read_to_string()?;

    // Extract page indices
    let indices = extract_page_indices(&body);

    // Use HashSet to keep hrefs unique
    let mut hrefs: HashSet<String> = HashSet::new();

    // const INDEX: &str = "&index=0";
    for index in indices {
        let url = format!(
            "{search_url}&index={index}"
        );
        
        // Fetch HTML
        let body: String = ureq::get(url).call()?.body_mut().read_to_string()?;

        // Parse HTML
        let document = Html::parse_document(&body);
    
        // CSS selector for the anchor elements
        let selector =
            Selector::parse(r#"a[data-testid="property-details-lozenge"]"#).expect("Invalid selector");
    
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                hrefs.insert(href.to_string());
            }
        }
    }
    
    // Convert to Vec if needed
    let unique_hrefs: Vec<String> = hrefs.into_iter().collect();

    Ok(unique_hrefs)
}

fn extract_page_indices(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);

    // select[data-testid="paginationSelect"] option[value]
    let opt_sel = Selector::parse(r#"select[data-testid="paginationSelect"] option[value]"#).unwrap();

    let mut indices = Vec::new();
    for opt in doc.select(&opt_sel) {
        if let Some(v) = opt.value().attr("value") {
            // Values in your example are "0", "24", "48", ...
            indices.push(v.to_string());
        }
    }

    // De-dupe while preserving order (simple, small)
    let mut seen = HashSet::new();
    indices.retain(|v| seen.insert(v.clone()));

    indices
}


#[test]
fn test_search_rightmove_properties() {
    
    let homes = match get_rightmove_properties() {
        Ok(homes) => homes,
        Err(e) => panic!("Failed to get properties with error: {e}"),
    };

    let results = match get_number_of_properties() {
        Ok(r) => r,
        Err(e) => panic!("Someting went wrong: {e}"),
    };

    dbg!("Results expected: {:#?}", &results);

    assert_eq!(homes.len(), results);
}
