use crate::lib_scraper;

const WOD_TYPE_UNDEFINED_ID: i32 = 4;

pub fn format_wod_name(wod_name: String) -> String {
    let name = wod_name.split("Hero WOD: ").collect::<Vec<&str>>()[1].trim();
    return name.to_string();
}

pub fn format_wod_type(wod_type: String) -> String {
    let name = wod_type.split("WOD Type: ").collect::<Vec<&str>>()[1].trim();
    return name.to_string();
}

/**
 * category_id: will be the position of the wod type in the wod_types array
 * begin from 1
 */
pub fn get_wod_category_id(wod_category: String) -> i32 {
    let sanitized = format_wod_type(wod_category.to_string());
    let wod_types: Vec<&str> = vec!["EMOM", "AMRAP", "For Time"];
    let position = wod_types
        .iter()
        .position(|&x| x.trim() == sanitized.as_str());
    return match position {
        Some(x) => x as i32 + 1,
        None => WOD_TYPE_UNDEFINED_ID,
    };
}

pub fn get_wod_type_by_criteria(wod_type: i32, details: &Vec<String>) -> i32 {
    if wod_type == WOD_TYPE_UNDEFINED_ID {
        if details.len() <= 3 {
            return 2; // AMRAP
        }
    }
    return wod_type;
}

pub fn get_wod_rounds(wod_type: i32, details: &Vec<String>) -> i32 {
    let mut rounds = 0;
    if wod_type != 2 {
        let first_detail = details[0].to_string().to_lowercase();
        if first_detail.find("rounds") != None {
            let words = first_detail.split(" ").collect::<Vec<&str>>();
            let position = words.iter().position(|&x| x.contains("rounds"));
            rounds = words[position.unwrap() - 1].trim().parse::<i32>().unwrap();
        }
    }
    return rounds;
}

pub fn get_wod_details(content: &str) -> Vec<String> {
    let html_content = lib_scraper::get_html_by_content(content);
    let detail_selector = lib_scraper::get_selector("li");
    let wod_details = html_content
        .select(&detail_selector)
        .map(|x| x.inner_html());
    let mut wod_items = Vec::new();
    for wod_detail in wod_details {
        wod_items.push(wod_detail.to_string());
    }
    return wod_items;
}
