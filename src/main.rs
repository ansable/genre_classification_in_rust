//so far i ordered all imports according to functions, so it would be easier for you
//to understand what comes from what
//TODO: order them afterwards according common sense
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
//imports for stopwords function
extern crate stopwords;
use std::collections::HashSet;
use stopwords::{Language, Spark, Stopwords};

extern crate scanlex;
use scanlex::{Scanner, Token};

extern crate ngrams;
use ngrams::Ngram;

extern crate select;
use select::document::Document;
use select::predicate::Name;
use std::io::Cursor;

//preprocessing - probs lets put it in separate file

//function to identify type of the file. Returns String with: "html", "txt" or "other"
fn type_of_file(filename: &str) -> &str {
    if filename.contains(".txt") {
        return "txt";
    } else if filename.contains(".html") {
        return "html";
    } else {
        return "other";
    }
}

//function to extract p tags from html and make a text out of it
//tests and stuff to work easier with a crate
//https://github.com/utkarshkukreti/select.rs/blob/master/tests/node_tests.rs
fn preprocess_html(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let document = Document::from_read(f);
    assert!(document.is_ok());
    let mut text = String::from(" ");
    for p in document.unwrap().find(Name("p")) {
        text = format!("{}", text.to_owned() + &p.text());
        //println!("{}", text);
    }
    return text;
}

//function to remove stuff from txt
//not finished: it's just reading text so far
//reason: seems like very unproductive
//idea: use grep? don't do preprocessing for txt at all?
fn preprocess_txt(filename: &str) -> Result<(), std::io::Error> {
    let input = File::open(filename).expect("Could not open settings file");
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
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

//function to make ngrams out of everything
fn make_ngrams<'a>(n: usize, text: &'a str) -> Vec<std::vec::Vec<&str>> {
    let mut ngrams: Vec<_> = text.split(' ').ngrams(n).collect();
    return ngrams;
}

//main preprocessing function, where text is identified and True/False (stopwords) is in parameters
fn preprocess(with_stopwords: bool, filename: &str) -> Vec<std::string::String> {
    let mut result = match type_of_file(filename) {
        "txt" => panic!("shoudnt be here"),
        "html" => tokenize_text(&preprocess_html(filename)),
        "other" => panic!("Smth wrong"),
        _ => panic!("eh"),
    };
    return result;
}

fn main() {
    println!("Rust project created!");
    //tests for everything
    //TODO: obv delete in the end
    //stage1
    //type_of_the_function
    assert_eq!(type_of_file("james.txt"), "txt");
    assert_eq!(type_of_file("marko.html"), "html");
    assert_eq!(type_of_file("anna is more than a file"), "other");
    //remove_stopwords
    assert_eq!(remove_stopwords("a house tired is"), &["house", "tired"]);
    //tokenize text
    assert_eq!(
        tokenize_text("??? who are! (CAT)))"),
        &["who", "are", "CAT"]
    );
    //    let s = preprocess_html("Arrangement.html");
    //    println!("{}", s);
    let s = preprocess(false, "Arrangement.html");
    println!("{:?}", s);
}
