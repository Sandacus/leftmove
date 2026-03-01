use scraper::{Html, Selector};
use std::collections::HashSet;

const URL: &str = "https://www.rightmove.co.uk";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch HTML
    let body: String = ureq::get(URL).call()?.body_mut().read_to_string()?;

    // Parse HTML
    let document = Html::parse_document(&body);

    // CSS selector for the anchor elements
    let selector =
        Selector::parse(r#"a[data-testid="property-details-lozenge"]"#).expect("Invalid selector");

    // Use HashSet to keep hrefs unique
    let mut hrefs: HashSet<String> = HashSet::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            hrefs.insert(href.to_string());
        }
    }

    // Convert to Vec if needed
    let unique_hrefs: Vec<String> = hrefs.into_iter().collect();

    println!("{:#?}", unique_hrefs);

    Ok(())
}
