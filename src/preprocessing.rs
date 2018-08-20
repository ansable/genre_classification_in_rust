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
use select::predicate::Name;

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
fn read_html_file(filename: &str, mut vocab: Vec<std::string::String>) -> Result<(Vec<&str>,Vec<std::string::String>), std::io::Error> {
    let file = File::open(filename).expect("File not found");
    let document = Document::from_read(file);

    // type_of_file could have returned ".html" if the file were anything but ".txt"
    assert!(filename.contains(".html"), "File not HTML!");
    let mut contents = String::from(" ");
    for p in document.unwrap().find(Name("p")) {
        contents = format!("{}", contents.to_owned() + &p.text());
    }

    return Ok(tokenize_text(&contents, vocab));
}

// Takes text file as argument, returns its contents as a vector of tokens
fn read_txt_file(filename: &str, mut vocab: Vec<std::string::String>) -> Result<(Vec<&str>,Vec<std::string::String>), std::io::Error> {
    let mut file = File::open(filename).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error encountered while processing file");

    return Ok(tokenize_text(&contents, vocab));
}

// function to tokenize text
pub fn tokenize_text<'a>(text: &'a str, mut vocab: Vec<std::string::String>) -> (Vec<&str>, Vec<std::string::String>) {
    let text = str::replace(text, "'", " "); // scanlex crashes and burns upon encountering apostrophes

    let mut scanner = scanlex::Scanner::new(&text);

    let mut tokens: Vec<&str> = vec![];

    loop {
        let mut current_token = scanner.get();

        if current_token.finished() {
            break;
        }

        else if current_token.is_iden() {
            let current_word = current_token.to_iden().unwrap();

            tokens.push(&current_word);

            if !vocab.contains(&current_word) {
                vocab.push(current_word);
            }
        }
    }
    (tokens, vocab)
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
pub fn preprocess_file(with_stopwords: bool, filename: &str, mut vocab: Vec<std::string::String>) -> (Vec<&str>, Vec<std::string::String>) {
    match type_of_file(filename) {
        "txt" => read_txt_file(filename, vocab).unwrap(),
        "html" => read_html_file(filename, vocab).unwrap(),
        _ => panic!("Unknown file type encountered!"),
    }
}
