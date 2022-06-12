use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::stdin;

fn count_words_from_file(file_path: &str, input_word: &str) -> u32 {
    let file = File::open(file_path).expect("error opening the file");
    let reader = BufReader::new(file);
    let mut _word_count: u32 = 0;
    for line in reader.lines() {
        let curr: String = line.expect("error reading content of the file");
        let words: Vec<&str> = curr.split(input_word).collect();
        _word_count+= words.len() as u32 - 1;
    }
    return _word_count
}


fn count_words_from_string(input_string: &str, input_word: &str) -> u32 {
    let mut _word_count: u32 = 0;
    let words: Vec<&str> = input_string.split(input_word).collect();
    _word_count = words.len() as u32 - 1;
    
    return _word_count;
}

fn input_line() -> String {
    let mut line: String = String::new();
    stdin().read_line(&mut line).unwrap();
    line.pop();

    line
}

fn main() {
    // input from terminal
    let input_word = input_line();
    // string to check
    let input_string = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    
    let word_count_from_string = count_words_from_string(input_string, &input_word);
    println!("Number of word {} in the string is {}", input_word, word_count_from_string);

    let file_path = "src/input.txt";
    let word_count_from_file = count_words_from_file(file_path, &input_word);
    println!("Number of word {} in the file's text is {}", input_word, word_count_from_file);
}