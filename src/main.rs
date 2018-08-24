extern crate clap;
extern crate scanlex;
extern crate select;
extern crate stopwords;
extern crate time;

use time::PreciseTime;

#[macro_use]
extern crate lazy_static;

extern crate tfidf;
use tfidf::{TfIdf, TfIdfDefault};

use std::fs;

mod args;
use args::parse_args;

mod preprocessing;
use preprocessing::VOCAB;
use preprocessing::preprocess_file;

// function to get tokens from the whole training corpus
fn get_tokens_and_counts_from_corpus(
    train_directory: &str,
    train_labels_file: &str,
    filter_stopwords: bool,
) -> Vec<Vec<(std::string::String, usize)>> {
    let train_dir = fs::read_dir(train_directory).unwrap();

    let mut all_files: Vec<Vec<(std::string::String, usize)>> = vec![];

    for (count, train_file) in train_dir.enumerate() {
        println!("{:?}", count + 1);

        let file_vector = preprocess_file(
            train_file.unwrap().path().to_str().unwrap(),
            filter_stopwords,
        );

        match file_vector {
            Some(v) => all_files.push(v),
            None => continue,
        }
    }
    all_files
}

fn get_tfdif_vectors(all_files: Vec<Vec<(std::string::String, usize)>>) -> Vec<Vec<f64>> {
    let mut tfidf_vectors: Vec<Vec<f64>> = vec![];
    for doc in &all_files {
        let mut tfidf_vector: Vec<f64> = vec![0f64; VOCAB.lock().unwrap().len()];

        for word in doc.iter() {
            let position = VOCAB
                .lock()
                .unwrap()
                .iter()
                .position(|t| t == &word.0)
                .unwrap();
            tfidf_vector[position] = TfIdfDefault::tfidf(&word.0, doc, all_files.iter());
        }
        tfidf_vectors.push(tfidf_vector);
    }
    tfidf_vectors
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

    let start = PreciseTime::now();

    let matches = parse_args();

    let train_dir = matches.value_of("TRAIN_DIR").unwrap_or("./train");
    let train_labels_file = matches
        .value_of("TRAIN_LABELS")
        .unwrap_or("labels_train.txt");

    let all_files = get_tokens_and_counts_from_corpus(
        train_dir,
        train_labels_file,
        matches.is_present("stopwords"),
    );
    let tfidf_vectors = get_tfdif_vectors(all_files);
    println!("{:?}", tfidf_vectors);

    let end = PreciseTime::now();
    println!("This program took {} seconds to run", start.to(end)); // current benchmark: 149s
    println!("{:?}", VOCAB.lock().unwrap().len());
}
