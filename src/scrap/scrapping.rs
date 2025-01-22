use reqwest::blocking::Client;
use scraper::{Html, Selector};
use crate::models::product::{Features, ProductData};



pub fn get_product_link(client: &Client, path: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let response = client.get(path).send()?.text()?;
    let document = Html::parse_document(&response);
    let link_components = Selector::parse("div.cat-products>div>a")?;

    let links: Vec<String> = document
    .select(&link_components)
    .filter_map(|element| element.value().attr("href").map(String::from))
    .collect();

    Ok(links)
}

pub fn get_information_product(client: &Client, path: &String) -> Result<ProductData, Box<dyn std::error::Error>> {
    let response = client.get(path).send()?.text()?;
    let document = Html::parse_document(&response);
    let code_selector = Selector::parse("div.product-code")?;

    let code = document
        .select(&code_selector)
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default()
        .trim()
        .to_string();

    println!("Traitement en cours : {}", code);

    let row_selector = Selector::parse("div.righe-tabella>div.product-value-cell").unwrap();
    let key_selector = Selector::parse("div.product-values-key").unwrap();
    let value_selector = Selector::parse("div.product-values-value").unwrap();
    
    let mut features: Vec<Features> = Vec::new();

    for row in document.select(&row_selector) {
        let key = row.select(&key_selector).next().map(|el| el.text().collect::<String>()).unwrap_or_default().trim().to_string();
        let value = row.select(&value_selector).next().map(|el| el.text().collect::<String>()).unwrap_or_default().trim().to_string();
        features.push(Features {
            key, value
        });
    }

    Ok(ProductData { code, features })
}
