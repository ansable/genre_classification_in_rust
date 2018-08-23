//so far i ordered all imports according to functions, so it would be easier for you
//to understand what comes from what
//TODO: order them afterwards according common sense
use std;
use std::fs::File;
use std::io::Read;

extern crate scanlex;
//use scanlex::{Scanner, Token};

use std::collections::HashSet;
use stopwords::{Language, Spark, Stopwords};

extern crate ngrams;
//use ngrams::Ngram;

use select::document::Document;
use select::predicate::Attr;
use select::predicate::Name;

use std::sync::Mutex;

lazy_static! {
    pub static ref VOCAB: Mutex<Vec<std::string::String>> = Mutex::new(vec![]);
}

// function to identify type of the file. Returns String with: "html", "txt" or "other"
fn type_of_file(filename: &str) -> &str {
    match filename.contains(".txt") {
        true => "txt",
        false => "html",
    }
}

// Takes HTML file as parameter, extracts text from p tags and returns its contents as a vector of tokens
// following crate was consulted for reference:
// https://github.com/utkarshkukreti/select.rs/blob/master/tests/node_tests.rs
fn read_html_file(filename: &str) -> Result<Vec<(std::string::String, usize)>, std::io::Error> {
    let file = File::open(filename).expect("File not found");
    let document = Document::from_read(file);

    // type_of_file could have returned ".html" if the file were anything but ".txt"
    assert!(filename.contains(".html"), "File not HTML!");
    let mut text = String::from(" ");

    for main_div in document.unwrap().find(Attr("id", "chapters")) {
        for p in main_div.find(Name("p")) {
            text = format!("{}", text.to_owned() + &p.first_child().unwrap().text());
        }
    }

    return Ok(tokenize_and_count(&text));
}

// Takes text file as argument, returns its contents as a vector of tokens
fn read_txt_file(filename: &str) -> Result<Vec<(std::string::String, usize)>, std::io::Error> {
    let mut file = File::open(filename).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error encountered while processing file");

    return Ok(tokenize_and_count(&contents));
}

// function to tokenize text, count tokens and build vocabulary
pub fn tokenize_and_count<'a>(text: &'a str) -> Vec<(std::string::String, usize)> {
    let text = str::replace(text, "'", " "); // scanlex crashes and burns upon encountering apostrophes

    let mut scanner = scanlex::Scanner::new(&text);

    let mut tokens_and_counts: Vec<(std::string::String, usize)> = vec![];

    loop {
        let current_token = scanner.get();

        if current_token.is_iden() {
            let current_word = &current_token.to_iden().unwrap();

            if !VOCAB.lock().unwrap().contains(&current_word) {
                VOCAB.lock().unwrap().push(current_word.to_string());
            }

            let mut token_in_tokens = false;
            let mut position_counter = 0;

            for &(ref token, _count) in tokens_and_counts.iter() {
                if token == &current_word.to_string() {
                    token_in_tokens = true;
                    break;
                }
                position_counter += 1;
            }
            if token_in_tokens {
                tokens_and_counts[position_counter].1 += 1;
            } else {
                tokens_and_counts.push((current_word.to_string(), 1));
            }
        } else if current_token.finished() {
            break;
        }
    }
    tokens_and_counts
}

// function to apply stopwords
fn remove_stopwords(line: &str) -> std::vec::Vec<&str> {
    let stopwords: HashSet<_> = Spark::stopwords(Language::English)
        .unwrap()
        .iter()
        .collect();
    let mut txt: Vec<&str> = line.split(" ").collect();
    txt.retain(|s| !stopwords.contains(s));
    return txt;
}

// // function to make ngrams out of everything
// fn make_ngrams<'a>(n: usize, text: &'a str) -> Vec<std::vec::Vec<&str>> {
//     let mut ngrams: Vec<_> = text.split(' ').ngrams(n).collect();
//     return ngrams;
// }

// main preprocessing function, where text is identified and True/False (stopwords) is in parameters
pub fn preprocess_file(filename: &str, with_stopwords: bool) -> Vec<(std::string::String, usize)> {
    match type_of_file(filename) {
        "txt" => read_txt_file(filename).unwrap(),
        "html" => read_html_file(filename).unwrap(),
        _ => panic!("Unknown file type encountered!"),
    }
}
