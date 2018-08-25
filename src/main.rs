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
use ndarray_linalg::svd::SVD;

extern crate serde;
extern crate serde_pickle;

extern crate rusty_machine;
use rusty_machine::learning::naive_bayes::{NaiveBayes, Gaussian};
use rusty_machine::linalg::Matrix;
use rusty_machine::learning::SupModel;

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
                break;
            }
        }

        if !filename_found {
            println!("{:?}{}", train_file_short, " wasn't found in labels file!");
            continue;
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
    println!("{}", "Creating tf-idf vectors...");
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

fn save_matrix_to_file(matrix: Vec<Vec<f64>>, file: &str) -> () {
    let matrix_pickled = serde_pickle::to_vec(&matrix, true).unwrap();
    fs::write(file, matrix_pickled).expect("Unable to write to file");
}

fn read_matrix_from_file(file: &str) -> Vec<Vec<f64>> {
    let matrix = fs::read(file).expect("Unable to read file");
    serde_pickle::from_slice(&matrix).unwrap()
}

// fn perform_svd(tfidf_vectors: Array2<f64>) {
//     // TODO even if we can somehow make this work for Array2, we will need to convert our Vec<Vec<f64>> into one
//     tfidf_vectors.svd(false, true);
// }

fn genre_labels_to_numbers(texts: Vec<Text>) -> (usize, usize, Vec<Vec<f64>>) {
    let mut labels_as_numbers: Vec<Vec<f64>> = vec![];
    let rows = texts.len();

    for text in texts.iter().cloned() {

        match &*text.label {
            "detective" => labels_as_numbers.push(vec![1.0, 0.0, 0.0, 0.0, 0.0]),
            "erotica" => labels_as_numbers.push(vec![0.0, 1.0, 0.0, 0.0, 0.0]),
            "horror" => labels_as_numbers.push(vec![0.0, 0.0, 1.0, 0.0, 0.0]),
            "romance" => labels_as_numbers.push(vec![0.0, 0.0, 0.0, 1.0, 0.0]),
            "scifi" => labels_as_numbers.push(vec![0.0, 0.0, 0.0, 0.0, 1.0]),
            _ => continue,
        }
    }
    (rows, 5, labels_as_numbers) // 5 is the number of different labels
}

fn matrix_to_vec(matrix: Vec<Vec<f64>>) -> (usize, usize, Vec<f64>) {
    let mut result_vec = vec![];
    let rows = matrix.len();
    let cols = matrix[0].len();

    for vec in matrix {
        for item in vec {
            result_vec.push(item);
        }
    }
    (rows, cols, result_vec)
}

// this one doesn't seem to be working at all, but we can give it another chance on a larger data set
fn get_naive_bayes_predictions(file_matrix: Vec<Vec<f64>>, texts: Vec<Text>) -> () {
    let (file_rows, file_cols, file_matrix_flat) = matrix_to_vec(file_matrix);
    let (label_rows, label_cols, text_labels_as_numbers) = matrix_to_vec(genre_labels_to_numbers(texts).2);

    let inputs = Matrix::new(file_rows, file_cols, file_matrix_flat);
    let labels = Matrix::new(label_rows, label_cols, text_labels_as_numbers);

    let mut model = NaiveBayes::<Gaussian>::new();

    println!("{}", "Training Naive Bayes model...");
    model.train(&inputs, &labels).unwrap();

    let predictions = model.predict(&inputs).unwrap();

    println!("{:?}", predictions);
}

fn main() {
    let start = PreciseTime::now();

    let matches = parse_args();

    let train_dir = matches.value_of("TRAIN_DIR").unwrap_or("./train");

    let train_labels_file = matches
        .value_of("TRAIN_LABELS")
        .unwrap_or("labels_train.txt"); // TODO change this to exit instead, no default here!

    let (all_files, filenames_and_labels_ordered) = get_tokens_and_counts_from_corpus(
        train_dir,
        train_labels_file,
        matches.is_present("stopwords"),
    );

    let tfidf_matrix = get_tfdif_vectors(all_files);

    get_naive_bayes_predictions(tfidf_matrix, filenames_and_labels_ordered);

    // save_matrix_to_file(tfidf_matrix, "matrix.pickle");

    // let deserialised_matrix = read_matrix_from_file("minimatrix.pickle");

    let end = PreciseTime::now();
    println!("This program took {} seconds to run", start.to(end));
}
