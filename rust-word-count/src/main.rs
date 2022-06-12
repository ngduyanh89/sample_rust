use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::stdin;

fn count_words_from_file(file_path: &str, input_word: &str) -> u32 {
    let file = File::open(file_path).expect("error opening the file");
    let reader = BufReader::new(file);
    let mut word_count: u32 = 0;
    for line in reader.lines() {
        let curr: String = line.expect("error reading content of the file");
        let words: Vec<&str> = curr.split(' ').collect();
        let filtered_words: Vec<&str> = words.into_iter().filter(|word| word.len() > 0 && word == &input_word).collect();
        word_count+= filtered_words.len() as u32
    }
    return word_count
}

fn input_line() -> String {
    let mut line: String = String::new();
    stdin().read_line(&mut line).unwrap();
    line.pop();

    line
}


fn main() {
    let file_path = "src/input.txt";
    let input_word = input_line();
    let word_count = count_words_from_file(file_path, &input_word);
    println!("{}", word_count);
}