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
    // the global VOCAB variable stores all the unique words encountered in the text
    // needs to be stored in a Mutex for thread safety
    pub static ref VOCAB: Mutex<Vec<std::string::String>> = Mutex::new(vec![]);
}

// Function to identify type of the file; currently only text and HTML files are supported.
fn type_of_file(filename: &str) -> &str {
    let split_filename: Vec<&str> = filename.split(".").collect();

    // match against substring after last dot in filename (should indicate file type)
    match split_filename[split_filename.len() - 1] {
        "txt" => "txt",
        "html" => "html",
        _ => "unknown",
    }
}

// Takes HTML file (extracted from zip archive) as parameter, extracts text from p tags and returns its contents as a vector of tokens
// The "select" crate was consulted for reference, specifically:
// https://github.com/utkarshkukreti/select.rs/blob/master/tests/node_tests.rs
fn read_html_file(
    file: zip::read::ZipFile,
    filter_stopwords: bool,
    training_mode: bool,
) -> Result<Vec<(std::string::String, usize)>, std::io::Error> {
    let document = Document::from_read(file);

    let mut text = String::from(" ");

    // all HTML files were taken from the same source (archiveofourown.org),
    // where the relevant content is found in the element with the "chapters" identifier
    for main_div in document.unwrap().find(Attr("id", "chapters")) {
        for p in main_div.find(Name("p")) {
            // extract all elements with <p> tag
            match &p.children().next() {
                Some(child) => text = format!("{}", text.to_owned() + &child.text()),
                None => continue,
            }
        }
    }
    Ok(get_word_counts_for_file(
        &text,
        filter_stopwords,
        training_mode,
    ))
}

// Takes text file (extracted from zip archive) as parameter, returns its contents as a vector of tokens
fn read_txt_file(
    mut file: zip::read::ZipFile,
    filter_stopwords: bool,
    training_mode: bool,
) -> Result<Vec<(std::string::String, usize)>, std::io::Error> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error encountered while processing file");

    Ok(get_word_counts_for_file(
        &contents,
        filter_stopwords,
        training_mode,
    ))
}

// Function to extract word tokens and counts from file, optionally building up the vocabulary along the way (training_mode parameter)
fn get_word_counts_for_file<'a>(
    text: &'a str,
    filter_stopwords: bool,
    training_mode: bool,
) -> Vec<(std::string::String, usize)> {
    let text = &str::replace(text, "'", " "); // scanlex crashes and burns upon encountering apostrophes

    let mut scanner = scanlex::Scanner::new(&text);
    let mut words_and_counts: Vec<(std::string::String, usize)> = vec![];
    let mut vocab = VOCAB.lock().unwrap();

    let stopwords: Vec<_> = Spark::stopwords(Language::English)
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect();

    loop {
        let current_token = scanner.get();

        if current_token.is_iden() {
            let current_word = &current_token.to_iden().unwrap().to_lowercase();

            if filter_stopwords {
                if stopwords.contains(&current_word.to_string()) {
                    continue;
                }
            }

            if training_mode {
                if !vocab.contains(&current_word) {
                    vocab.push(current_word.to_string());
                }
            } else {
                if !vocab.contains(&current_word) {
                    continue;
                }
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
    vocab.sort_unstable();
    words_and_counts.sort_unstable();
    words_and_counts
}

// Main preprocessing function, where text is identified and boolean parameters are passed on to other functions
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

#[cfg(test)]
mod tests {
    use super::{get_word_counts_for_train_file, type_of_file};

    #[test]
    fn type_of_file_test() {
        assert_eq!(type_of_file("john.txt"), "txt");
        assert_eq!(type_of_file("mary.html"), "html");
        assert_eq!(type_of_file("hope is lost"), "unknown");
    }
    #[test]
    fn get_word_counts_for_test_file_test() {
        let vec1 = vec![
            (String::from("a"), 2 as usize),
            (String::from("bird"), 1 as usize),
            (String::from("man"), 1 as usize),
        ];
        let vec2 = vec![
            (String::from("dog"), 1 as usize),
            (String::from("m"), 1 as usize),
        ];
        assert_eq!(get_word_counts_for_train_file("A bird, a man", false), vec1);
        assert_eq!(
            get_word_counts_for_train_file("A a a a a a is dog I'm", true),
            vec2
        );
    }

}
