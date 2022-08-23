mod lib_handler;
mod lib_scraper;
mod lib_text_processor;
use itertools::izip;

struct WodItem {
    pub name: String,
    pub group: i32,
    pub rounds: i32,
    pub time: String,
    pub detail: Vec<String>,
}

fn get_wod_info() {
    let html_content = lib_scraper::get_html_content();
    let name_selector = lib_scraper::get_selector(lib_scraper::WOD_NAMES);
    let type_selector = lib_scraper::get_selector(lib_scraper::WOD_TYPE);
    let detail_selector = lib_scraper::get_selector(lib_scraper::WOD_DETAILS);

    let wod_names = html_content.select(&name_selector).map(|x| x.inner_html());
    let wod_types = html_content.select(&type_selector).map(|x| x.inner_html());
    let wod_details = html_content
        .select(&detail_selector)
        .map(|x| x.inner_html());

    let mut wod_items = Vec::new();

    for (wod_name, wod_type, wod_detail) in izip!(wod_names, wod_types, wod_details) {
        let name = lib_handler::format_wod_name(wod_name.to_string());
        let details = lib_handler::get_wod_details(&wod_detail);
        let time = lib_handler::get_wod_time(wod_type.to_string(), &details);
        let group = lib_handler::get_wod_category_id(wod_type.to_string());
        let rounds = lib_handler::get_wod_rounds(group, &details);

        let wod_item = WodItem {
            name: name,
            group,
            detail: details,
            rounds: rounds,
            time: time,
        };
        wod_items.push(wod_item);
    }

    for wod_item in wod_items {
        store_information(wod_item);
    }
}

fn store_information(wod: WodItem) {
    let item_type = lib_handler::get_wod_type_by_criteria(wod.group, &wod.detail);
    println!(
        "title:{} - type:{} - movements:{} rounds:{} time {}",
        wod.name,
        item_type,
        &wod.detail.len(),
        wod.rounds,
        wod.time
    );
}

fn main() {
    get_wod_info();
}
