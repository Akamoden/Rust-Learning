//todo : create a pattern scanner that scans a string for a pattern and returns whole words then words with number of occurrences   
use std::collections::HashMap;

//todo : create a function that takes a string and a pattern and returns a vector of words that match the pattern
fn pattern_scanner(input: &str, pattern: &str) -> Vec<String> {
    let mut result = Vec::new();
    let words: Vec<&str> = input.split_whitespace().collect();
    for word in words {
        if word.contains(pattern) {
            result.push(word.to_string());
        }
    }
    result
}   

//todo : tokenize the input string and count the number of occurrences of each word
fn word_count(input: &str) -> HashMap<String, usize> {
    let mut count_map = HashMap::new();
    let words: Vec<&str> = input.split_whitespace().collect();
    for word in words {
        let counter = count_map.entry(word.to_string()).or_insert(0);
        *counter += 1;
    }
    count_map
}           

//todo : create a main function that takes a string and a pattern and prints the words that match the pattern and the number of occurrences of each word
fn main() {
    let input = "hello world hello rust pattern scanner pattern scanner";
    let pattern = "pattern";
    let matched_words = pattern_scanner(input, pattern);
    println!("Words that match the pattern '{}': {:?}", pattern, matched_words);
    let word_counts = word_count(input);
    println!("Word counts: {:?}", word_counts);
}    