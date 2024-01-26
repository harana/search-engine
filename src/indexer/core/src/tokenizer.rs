use harana_common::itertools::Itertools;
use harana_common::lazy_static::lazy_static;

lazy_static! {
    pub static ref STOP_WORDS: Vec<String> = stop_words::get(stop_words::LANGUAGE::English);
}

pub fn tokenize(str: &str) -> Vec<String> {
    str.to_ascii_lowercase()
        .replace("http://", "")
        .replace("https://", "")
        .replace(&['(', ')', ',', '.',';', ':', '/', '\n', '\r', '\t', '@', '|', '_', '-'][..], " ")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split(" ")
        .filter(|&s| s.len() > 2)
        .filter(|&s| !STOP_WORDS.contains(&s.to_string()))
        .filter(|&s| !s.parse::<u32>().is_ok())
        .map(|s| s.trim().to_string())
        .collect_vec()
}