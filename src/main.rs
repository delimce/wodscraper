mod database;
mod handler;
mod libs;
mod orm;

use libs::scraper::*;
use libs::text_processor::*;

use itertools::izip;
use std::time::{SystemTime, UNIX_EPOCH};

struct WodItem {
    pub name: String,
    pub group: i32,
    pub rounds: i32,
    pub time: String,
    pub detail: Vec<String>,
}

fn get_wod_info() {
    let html_content = get_html_wod_content();
    let name_selector = get_selector(SELECTOR_WOD_NAMES);
    let type_selector = get_selector(SELECTOR_WOD_TYPE);
    let detail_selector = get_selector(SELECTOR_WOD_DETAILS);

    let wod_names = html_content.select(&name_selector).map(|x| x.inner_html());
    let wod_types = html_content.select(&type_selector).map(|x| x.inner_html());
    let wod_details = html_content
        .select(&detail_selector)
        .map(|x| x.inner_html());

    let mut wod_items = Vec::new();

    for (wod_name, wod_type, wod_detail) in izip!(wod_names, wod_types, wod_details) {
        let name = handler::format_wod_name(wod_name.to_string());
        let details = handler::get_wod_details(&wod_detail);
        let time = handler::get_wod_time(wod_type.to_string(), &details);
        let group = handler::get_wod_category_id(wod_type.to_string());
        let rounds = handler::get_wod_rounds(group, &details);

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
    let item_type = handler::get_wod_type_by_criteria(wod.group, &wod.detail);
    println!(
        "title:{} - type:{} - movements:{} rounds:{} time {}",
        wod.name,
        item_type,
        &wod.detail.len(),
        wod.rounds,
        wod.time
    );
}

fn capture_time_ms() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

fn get_movements_info() -> Vec<String> {
    let html_content = get_html_movements_content();
    let mov_selector = get_selector(SELECTOR_MOVEMENTS);
    let mov_names = html_content.select(&mov_selector).map(|x| x.inner_html());

    let mut movements = Vec::new();
    let mut index = 1;
    for mov_name in mov_names {
        if index >= 27 && index <= 126 {
            let fixed_name = remove_issues_from_text(mov_name.to_string());
            movements.push(fixed_name);
        }
        index += 1;
    }
    return movements;
}

fn main() {
    let in_ms = capture_time_ms();
   /*  get_wod_info();
    get_movements_info(); */

    let pool = database::create_pool();

    database::insert_movements(&pool, get_movements_info());

    /* let measures = database::get_register_pool(&pool); */
    /*  let measures = database::get_registers(); */

   /*  for measure in measures {
        println!("{}", measure.name);
    } */

    let out_ms = capture_time_ms();
    println!("execution time:{} ms", out_ms - in_ms);
}
