use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

const WORD_LIST_FILE_PATH: &str = "./words_alpha.txt";
const WORD_LENGTH: usize = 5;

fn main() -> Result<()> {
    let lines = read_lines(WORD_LIST_FILE_PATH)?;
    let words_vector = read_words(lines, WORD_LENGTH);

    for word in find_words_matching_regex(words_vector, "br.n.")? {
        println!("{}", word);
    }
    println!();

    return Ok(());
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return Ok(BufReader::new(file).lines());
}

fn read_words(lines: Lines<BufReader<File>>, filer_length: usize) -> Vec<String> {
    return lines
        .flatten()
        .into_iter()
        .filter(|line| line.len() == filer_length)
        .collect();
}

fn find_words_matching_regex(words: Vec<String>, pattern: &str) -> Result<Vec<String>> {
    let regex = Regex::new(pattern)?;

    let filtered_words = words
        .into_iter()
        .filter(|word| regex.is_match(word))
        .collect();

    return Ok(filtered_words);
}
