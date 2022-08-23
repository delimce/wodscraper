use crate::lib_scraper;
use crate::lib_text_processor;

const WOD_TYPE_UNDEFINED_ID: i32 = 4;

pub fn format_wod_name(wod_name: String) -> String {
    return lib_text_processor::remove_pref(wod_name, "Hero WOD: ");
}

pub fn format_wod_type(wod_type: String) -> String {
    return lib_text_processor::remove_pref(wod_type, "WOD Type: ");
}

/**
 * category_id: will be the position of the wod type in the wod_types array
 * begin from 1
 */
pub fn get_wod_category_id(wod_category: String) -> i32 {
    let sanitized = format_wod_type(wod_category.to_string());
    let wod_types: Vec<&str> = vec!["EMOM", "AMRAP", "For Time"];
    let position = lib_text_processor::get_word_position(&sanitized, &wod_types.join(" "));
    if position == -1 {
        return WOD_TYPE_UNDEFINED_ID;
    }
    return position + 1 as i32;
}

pub fn get_wod_type_by_criteria(wod_type: i32, details: &Vec<String>) -> i32 {
    if wod_type == WOD_TYPE_UNDEFINED_ID {
        if details.len() <= 3 {
            return 2; // AMRAP
        }
    }
    return wod_type;
}

pub fn get_wod_time(wod_type: String, details: &Vec<String>) -> String {
    let mut time = "";
    let mut title_with_time = String::new();
    let minutes = "minutes";
    let first_detail = details[0].to_string().to_lowercase();

    // try with wod type if exists
    let mut position = lib_text_processor::get_word_position(&wod_type, &minutes);
    if position == -1 {
        // try with first detail if exists
        position = lib_text_processor::get_word_position(&first_detail, &minutes);
        if position > -1 {
            title_with_time = first_detail.to_string();
        }
    } else {
        title_with_time = wod_type;
    }

    if position > -1 {
        let words = lib_text_processor::get_words_from_sentence(&title_with_time);
        time = words[position as usize - 1].trim();
    }
    return time.to_string();
}

pub fn get_wod_rounds(wod_type: i32, details: &Vec<String>) -> i32 {
    let mut rounds = 0;
    if wod_type != 2 {
        let first_detail = details[0].to_string().to_lowercase();
        if first_detail.find("rounds") != None {
            let words = lib_text_processor::get_words_from_sentence(&first_detail);
            let position = lib_text_processor::get_word_position(&first_detail, "rounds");
            // pre position of the word "rounds"
            rounds = words[position as usize - 1].trim().parse::<i32>().unwrap();
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
