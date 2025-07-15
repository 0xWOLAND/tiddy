use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct WordList {
    words: Vec<String>,
}

fn load_words_from_file(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = format!("words/{}", filename);
    let content = fs::read_to_string(&path)?;
    let word_list: WordList = serde_json::from_str(&content)?;
    Ok(word_list.words)
}

pub fn generate_words(count: usize, word_list: Option<&str>) -> Vec<String> {
    let filename = word_list.unwrap_or("english.json");
    let words = load_words_from_file(filename)
        .unwrap_or_else(|_| vec!["the".to_string(), "quick".to_string(), "brown".to_string()]);
    
    let mut rng = rand::thread_rng();
    words
        .choose_multiple(&mut rng, count)
        .map(|word| word.clone())
        .collect()
}