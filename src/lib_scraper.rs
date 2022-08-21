use reqwest;
use scraper::{Html, Selector};
use std::process::exit;

const BASE_URL: &str = "https://garagegymbuilder.com/crossfit-hero-wod-masterlist/";

pub(crate) const WOD_NAMES: &str = "h3#tab-con-3";
pub(crate) const WOD_TYPE: &str = "p span.bold_text";
pub(crate) const WOD_DETAILS: &str = "ul.tve_ul.tve_ul1.tve_green";

pub fn get_web_content() -> String {
    let response = reqwest::blocking::get(BASE_URL).unwrap();
    if !response.status().is_success() {
        println!("Request error: {}", response.status());
        exit(0);
    }
    return response.text().unwrap();
}

pub fn get_html_content() -> Html {
    let html_content = get_web_content();
    return Html::parse_document(&html_content);
}

pub fn get_html_by_content(content: &str) -> Html {
    return Html::parse_document(content);
}

pub fn get_selector(selector: &str) -> Selector {
    return Selector::parse(selector).unwrap();
}
