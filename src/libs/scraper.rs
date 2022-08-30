use reqwest;
use scraper::{Html, Selector};
use std::process::exit;

const BASE_URL_HEROES: &str = "https://garagegymbuilder.com/crossfit-hero-wod-masterlist/";
const BASE_URL_MOVEMENTS: &str = "https://wodwell.com/browse/?ref=footer#movement";

pub(crate) const SELECTOR_WOD_NAMES: &str = "h3#tab-con-3";
pub(crate) const SELECTOR_WOD_TYPE: &str = "p span.bold_text";
pub(crate) const SELECTOR_WOD_DETAILS: &str = "ul.tve_ul.tve_ul1.tve_green";
pub(crate) const SELECTOR_MOVEMENTS: &str = "a.wod-collections-list__item";

pub fn get_web_content(url: &str) -> String {
    let response = reqwest::blocking::get(url).unwrap();
    if !response.status().is_success() {
        println!("Request error: {}", response.status());
        exit(0);
    }
    return response.text().unwrap();
}

pub fn get_html_wod_content() -> Html {
    let html_content = get_web_content(BASE_URL_HEROES);
    return Html::parse_document(&html_content);
}

pub fn get_html_movements_content() -> Html {
    let html_content = get_web_content(BASE_URL_MOVEMENTS);
    return Html::parse_document(&html_content);
}

pub fn get_html_by_content(content: &str) -> Html {
    return Html::parse_document(content);
}

pub fn get_selector(selector: &str) -> Selector {
    return Selector::parse(selector).unwrap();
}
