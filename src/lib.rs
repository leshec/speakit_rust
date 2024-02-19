use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn clean_word(word: &str) -> Option<String> {
    let cleaned_word = word.trim().to_lowercase();
    let x: &[_] = &[',', ':', ';', ')', '(', '[', ']', '.', '!', '?'];
    let cleaned_word = cleaned_word.trim_end_matches(x).to_string();
    if cleaned_word.len() > 2 && !cleaned_word.chars().any(|c| !c.is_alphabetic()) {
        Some(cleaned_word)
    } else {
        None
    }
}

pub fn count_frequency(lines: &[String]) -> HashMap<String, usize> {
    let mut freq: HashMap<String, usize> = HashMap::new();
    for line in lines {
        for word in line.split_whitespace().flat_map(|w| clean_word(w)) {
            *freq.entry(word).or_insert(0) += 1;
        }
    }
    freq
}

pub fn get_matching_words<'a>(freq: &'a HashMap<String, usize>, substring: &str) -> Vec<&'a String> {
    let mut sorted_freq: Vec<_> = freq.iter().collect();
    sorted_freq.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_freq.into_iter().filter(|(word, _)| word.contains(substring)).map(|(word, _)| word).collect()
}


