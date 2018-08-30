// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Final Project
// Authors:     Anna Soboleva, Marko Ložajić
//
// Honor Code:  We pledge that this program represents our own work.

/// Module containing functions for cleaning input and passing raw word counts downstream.

use std;
use std::io::Read;
use std::sync::Mutex;

use scanlex;

use select::document::Document;
use select::predicate::Attr;
use select::predicate::Name;

use stopwords::{Language, Spark, Stopwords};

use zip;

lazy_static! {
    pub static ref VOCAB: Mutex<Vec<std::string::String>> = Mutex::new(vec![]);
}

// function to identify type of the file. Returns String with: "html", "txt" or "other"
fn type_of_file(filename: &str) -> &str {
    let split_filename: Vec<&str> = filename.split(".").collect();

    // match against string following last dot in filename (should indicate file type)
    match split_filename[split_filename.len() - 1] {
        "txt" => "txt",
        "html" => "html",
        _ => "unknown",
    }
}

// Takes HTML file as parameter, extracts text from p tags and returns its contents as a vector of tokens
// following crate was consulted for reference:
// https://github.com/utkarshkukreti/select.rs/blob/master/tests/node_tests.rs
fn read_html_file(
    file: zip::read::ZipFile,
    filter_stopwords: bool,
    training_mode: bool,
) -> Result<Vec<(std::string::String, usize)>, std::io::Error> {
    let document = Document::from_read(file);

    let mut text = String::from(" ");

    for main_div in document.unwrap().find(Attr("id", "chapters")) {
        for p in main_div.find(Name("p")) {
            match &p.children().next() {
                Some(child) => text = format!("{}", text.to_owned() + &child.text()),
                None => continue,
            }
        }
    }
    match training_mode {
        true => Ok(get_word_counts_for_train_file(&text, filter_stopwords)),
        false => Ok(get_word_counts_for_test_file(&text, filter_stopwords)),
    }
}

// Takes text file as argument, returns its contents as a vector of tokens
fn read_txt_file(
    mut file: zip::read::ZipFile,
    filter_stopwords: bool,
    training_mode: bool,
) -> Result<Vec<(std::string::String, usize)>, std::io::Error> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error encountered while processing file");

    match training_mode {
        true => Ok(get_word_counts_for_train_file(&contents, filter_stopwords)),
        false => Ok(get_word_counts_for_test_file(&contents, filter_stopwords)),
    }
}

// function to tokenize text, extract/count word tokens and build vocabulary
fn get_word_counts_for_train_file<'a>(
    text: &'a str,
    filter_stopwords: bool,
) -> Vec<(std::string::String, usize)> {
    let text = &str::replace(text, "'", " "); // scanlex crashes and burns upon encountering apostrophes

    let mut scanner = scanlex::Scanner::new(&text);
    let mut words_and_counts: Vec<(std::string::String, usize)> = vec![];
    let mut vocab = VOCAB.lock().unwrap();

    if filter_stopwords {
        let stopwords: Vec<_> = Spark::stopwords(Language::English)
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect();

        loop {
            let current_token = scanner.get();

            if current_token.is_iden() {
                let current_word = &current_token.to_iden().unwrap().to_lowercase();

                if stopwords.contains(&current_word.to_string()) {
                    continue;
                }

                if !vocab.contains(&current_word) {
                    vocab.push(current_word.to_string());
                }

                match words_and_counts
                    .iter()
                    .position(|(x, _y)| x == &current_word.to_string())
                {
                    Some(p) => words_and_counts[p].1 += 1,
                    None => words_and_counts.push((current_word.to_string(), 1)),
                }
            } else if current_token.finished() {
                break;
            }
        }
    } else {
        loop {
            let current_token = scanner.get();

            if current_token.is_iden() {
                let current_word = &current_token.to_iden().unwrap().to_lowercase();

                if !vocab.contains(&current_word) {
                    vocab.push(current_word.to_string());
                }

                match words_and_counts
                    .iter()
                    .position(|(x, _y)| x == &current_word.to_string())
                {
                    Some(p) => words_and_counts[p].1 += 1,
                    None => words_and_counts.push((current_word.to_string(), 1)),
                }
            } else if current_token.finished() {
                break;
            }
        }
    }
    // sort vectors so that we can look for words in O(log n) time using binary search - important for calculating TF-IDF scores
    vocab.sort_unstable();
    words_and_counts.sort_unstable();
    words_and_counts
}

// function to tokenize text, extract/count word tokens and build vocabulary
fn get_word_counts_for_test_file<'a>(
    text: &'a str,
    filter_stopwords: bool,
) -> Vec<(std::string::String, usize)> {
    let text = &str::replace(text, "'", " "); // scanlex crashes and burns upon encountering apostrophes

    let mut scanner = scanlex::Scanner::new(&text);
    let mut words_and_counts: Vec<(std::string::String, usize)> = vec![];
    let mut vocab = VOCAB.lock().unwrap();

    if filter_stopwords {
        let stopwords: Vec<_> = Spark::stopwords(Language::English)
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect();

        loop {
            let current_token = scanner.get();

            if current_token.is_iden() {
                let current_word = &current_token.to_iden().unwrap().to_lowercase();

                if stopwords.contains(&current_word.to_string()) {
                    continue;
                }

                if !vocab.contains(&current_word) {
                    continue;
                }

                match words_and_counts
                    .iter()
                    .position(|(x, _y)| x == &current_word.to_string())
                {
                    Some(p) => words_and_counts[p].1 += 1,
                    None => words_and_counts.push((current_word.to_string(), 1)),
                }
            } else if current_token.finished() {
                break;
            }
        }
    } else {
        loop {
            let current_token = scanner.get();

            if current_token.is_iden() {
                let current_word = &current_token.to_iden().unwrap().to_lowercase();

                if !vocab.contains(&current_word) {
                    continue;
                }

                match words_and_counts
                    .iter()
                    .position(|(x, _y)| x == &current_word.to_string())
                {
                    Some(p) => words_and_counts[p].1 += 1,
                    None => words_and_counts.push((current_word.to_string(), 1)),
                }
            } else if current_token.finished() {
                break;
            }
        }
    }
    // sort vectors so that we can look for words in O(log n) time using binary search - important for calculating TF-IDF scores
    vocab.sort_unstable();
    words_and_counts.sort_unstable();
    words_and_counts
}

// main preprocessing function, where text is identified and True/False (stopwords) is in parameters
pub fn preprocess_file(
    file: zip::read::ZipFile,
    filter_stopwords: bool,
    training_mode: bool,
) -> Option<Vec<(std::string::String, usize)>> {
    let sanitized_name = file.sanitized_name();
    let filename = sanitized_name.to_str().unwrap();
    match type_of_file(filename) {
        "txt" => Some(read_txt_file(file, filter_stopwords, training_mode).unwrap()),
        "html" => Some(read_html_file(file, filter_stopwords, training_mode).unwrap()),
        _ => None,
    }
}
