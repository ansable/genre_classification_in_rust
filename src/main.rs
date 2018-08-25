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

extern crate ndarray;
use ndarray::Array2;

extern crate ndarray_linalg;
use ndarray_linalg::svd::*;

use std::fs;

mod args;
use args::parse_args;

mod preprocessing;
use preprocessing::VOCAB;
use preprocessing::preprocess_file;

mod text;
use text::Text;
use text::read_filenames_and_labels;

// function to get tokens from the whole training corpus
fn get_tokens_and_counts_from_corpus(
    train_directory: &str,
    train_labels_file: &str,
    filter_stopwords: bool,
) -> (Vec<Vec<(std::string::String, usize)>>, Vec<Text>) {
    let train_dir = fs::read_dir(train_directory).unwrap();

    let mut all_files: Vec<Vec<(std::string::String, usize)>> = vec![];

    let filenames_and_labels: Vec<Text> = read_filenames_and_labels(train_labels_file);
    let mut filenames_and_labels_ordered: Vec<Text> = vec![];

    for (count, train_file) in train_dir.enumerate() {
        let train_file_path = train_file.unwrap().path();
        let train_file_full: &str = train_file_path.to_str().unwrap();
        let train_file_as_vec: Vec<&str> = train_file_full.split("/").collect();

        let train_file_short = train_file_as_vec[train_file_as_vec.len() - 1]; // gets filename after the last slash

        let mut filename_found = false;

        for text in filenames_and_labels.iter().cloned() {
            if text.filename == train_file_short {
                filenames_and_labels_ordered.push(text);
                filename_found = true;
                break
            }
        }

        if !filename_found {
            println!("{:?}{}", train_file_short, " wasn't found in labels file!");
            continue
        }

        println!("{:?}", train_file_short);
        println!("{:?}", count + 1);

        let file_vector = preprocess_file(train_file_full, filter_stopwords);

        match file_vector {
            Some(v) => all_files.push(v),
            None => continue,
        }
    }
    (all_files, filenames_and_labels_ordered)
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

    let start = PreciseTime::now();

    let matches = parse_args();

    let train_dir = matches.value_of("TRAIN_DIR").unwrap_or("./train");

    let train_labels_file = matches
        .value_of("TRAIN_LABELS")
        .unwrap_or("labels_train.txt"); // TODO  change this to exit instead, no default here!

    let (all_files, filenames_and_labels_ordered) = get_tokens_and_counts_from_corpus(
        train_dir,
        train_labels_file,
        matches.is_present("stopwords"),
    );

    println!("{:?}", all_files);
    let tfidf_vectors = get_tfdif_vectors(all_files);
    println!("{:?}", filenames_and_labels_ordered);
    println!("{:?}", tfidf_vectors);
    println!("{}{:?}", "The vocabulary size is: ", VOCAB.lock().unwrap().len());

    let end = PreciseTime::now();
    println!("This program took {} seconds to run", start.to(end));
}
