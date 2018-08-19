//so far i ordered all imports according to functions, so it would be easier for you
//to understand what comes from what
//TODO: order them afterwards according common sense
use std::fs::File;
use std::io::{Read}; // BufRead, BufReader, Write 
//imports for stopwords function
extern crate stopwords;
use std::collections::HashSet;
use stopwords::{Language, Spark, Stopwords};

extern crate scanlex;
//use scanlex::{Scanner, Token};

extern crate ngrams;
//use ngrams::Ngram;

extern crate select;
use select::document::Document;
use select::predicate::Name;
//use std::io::Cursor;

// preprocessing - probs lets put it in separate file

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
fn read_html_file(filename: &str) -> Result<Vec<std::string::String>, std::io::Error> {
    let file = File::open(filename).expect("File not found");
    let document = Document::from_read(file);

    // type_of_file could have returned ".html" if the file were anything but ".txt"
    assert!(filename.contains(".html"), "File not HTML!");
    let mut text = String::from(" ");
    for p in document.unwrap().find(Name("p")) {
        text = format!("{}", text.to_owned() + &p.text());
    }

    return Ok(tokenize_text(&text));
}

// Takes text file as argument, returns its contents as a vector of tokens
fn read_txt_file(filename: &str) -> Result<Vec<std::string::String>, std::io::Error> {
    let mut file = File::open(filename).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error encountered while processing file");
    
    return Ok(tokenize_text(&contents));
}

//function to tokenize text, using word tokenize
fn tokenize_text<'a>(text: &'a str) -> Vec<std::string::String> {
    let tokenized_text: Vec<_> = scanlex::Scanner::new(text)
        .filter_map(|t| t.to_iden())
        .collect();
    return tokenized_text;
}

//function to apply stopwords
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
fn preprocess_file(with_stopwords: bool, filename: &str) -> Vec<std::string::String> {
    match type_of_file(filename) {
        "txt" => read_txt_file(filename).unwrap(),
        "html" => read_html_file(filename).unwrap(),
        _ => panic!("Unknown file type encountered!"),
    }
}

fn main() {

    // Tests here for now TODO delete them
    // type_of_the_function
    assert_eq!(type_of_file("james.txt"), "txt");
    assert_eq!(type_of_file("marko.html"), "html");
    // assert!(type_of_file("anna is more than a file").is_err());
    // remove_stopwords
    assert_eq!(remove_stopwords("a house tired is"), &["house", "tired"]);
    // tokenize text
    assert_eq!(
        tokenize_text("??? who are! (CAT)))"),
        &["who", "are", "CAT"]
    );

    let lion = &preprocess_file(false, "train/a_lion.txt");
    println!("{:?}", lion);
    
    let baby = &preprocess_file(false, "train/baby.html");
    println!("{:?}", baby);
}
