mod models;
use models::product::ProductData;
mod scrap;
use scrap::scrapping::{get_product_link, get_information_product};

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;


const BASE_URL: &str = "https://castel.it/catalog/liste-de-produits/flltres-deshydrateurs-00657/filtres-deshydrateurs-hermetiques-ligne-go-green-00815";
const FILE_NAME: &str = "go-green-deshy-co2.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client.get(BASE_URL).send()?.text()?;
    let main_document = Html::parse_document(&response);

    let list_pagination = Selector::parse("ul.pagination>li>a.page-link").unwrap();

    println!("Recherche des pages disponnibles");

    let mut paginations: Vec<String> = main_document
        .select(&list_pagination)
        .filter_map(|el| el.value().attr("href").map(String::from))
        .collect();

    paginations.push(BASE_URL.to_string());

    println!("{} pages trouvées", paginations.len());

    let mut products_link: HashSet<String> = HashSet::new();

    for path_page in &paginations {
        let links = get_product_link(&client, path_page)?;
        for link in links {
            products_link.insert(link);
        }
        
    }

    let mut data_product: Vec<ProductData> = Vec::new();

    for path_product in &products_link {
        data_product.push(get_information_product(&client, path_product)?);
    }
    println!("Scrapping terminé");
    println!("Génération du fichier");
    generate_file(data_product)?;

    Ok(())
}



fn generate_file(products_data: Vec<ProductData>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(FILE_NAME)?;
    let data_json = serde_json::to_vec(&products_data)?;
    file.write_all(&data_json)?;
    Ok(())
}