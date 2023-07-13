use regex::Regex;

pub fn remove_pref(text: String, pref: &str) -> String {
    let sanitized = text.split(pref).collect::<Vec<&str>>()[1].trim();
    return sanitized.to_string();
}

pub fn get_words_from_sentence(sentence: &str) -> Vec<&str> {
    return sentence.split(" ").collect::<Vec<&str>>();
}

pub fn remove_issues_from_text(text: String) -> String {
    let regex = Regex::new(r"\([^)]*\)").unwrap(); // remove parenthesis
    return regex.replace_all(&text, "").trim().to_string();
}

pub fn are_words_in_sentence(words:&str,sentence:&str)-> bool {
    return sentence.to_lowercase().contains(words.to_lowercase().as_str());
}

pub fn get_word_position(text: &String, word: &str) -> i32 {
    let words = get_words_from_sentence(text);
    let position = words.iter().position(|&x| x.contains(word));
    return match position {
        Some(x) => x as i32,
        None => -1,
    };
}
