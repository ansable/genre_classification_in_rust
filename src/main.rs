extern crate scanlex;
extern crate select;
extern crate stopwords;

extern crate time;
use time::PreciseTime;

use std::collections::HashMap;
use std::fs;

mod preprocessing;
use preprocessing::preprocess_file;
use preprocessing::tokenize_text;

// function to build empty vec with all words that appear in vocabulary
fn build_vocab(train_directory: &str) -> (Vec<Vec<std::string::String>>, Vec<std::string::String>) {
    let train_dir = fs::read_dir("./train").unwrap();

    let mut vocab: Vec<std::string::String> = vec![];
    let mut all_files: Vec<Vec<std::string::String>> = vec![];

    let mut count = 0usize;

    for train_file in train_dir {
        count += 1;
        println!("{:?}", count);
        let tokens_for_file = &preprocess_file(false, train_file.unwrap().path().to_str().unwrap());
        all_files.push(tokens_for_file.to_vec());

        for token in tokens_for_file {
            if !vocab.contains(&token) {
                vocab.push(token.to_string());
            }
        }
    }
    (all_files, vocab)
}

fn main() {
    // type_of_the_function
    // assert_eq!(type_of_file("james.txt"), "txt");
    // assert_eq!(type_of_file("marko.html"), "html");
    // assert!(type_of_file("anna is more than a file").is_err());
    // remove_stopwords
    // assert_eq!(remove_stopwords("a house tired is"), &["house", "tired"]);
    // tokenize text
    // assert_eq!(tokenize_text("??? who are! (CAT)))"),&["who", "are", "CAT"]);

    // let lion = &preprocess_file(false, "train/ghost_of_alive.txt");
    // println!("{:?}", lion);

    let baby = &preprocess_file(false, "train/disappearence.txt");
    println!("{:?}", baby);
    // let start = PreciseTime::now();

    // let vocab = build_vocab("./train").1;

    // let end = PreciseTime::now();
    // println!("This program took {} seconds to run", start.to(end)); // current benchmark: 148s
    // println!("{:?}", vocab.len());
}
