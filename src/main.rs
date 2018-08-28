extern crate clap;
extern crate scanlex;
extern crate select;
extern crate stopwords;
extern crate time;
extern crate zip;
use zip::ZipArchive;

use time::PreciseTime;

#[macro_use]
extern crate lazy_static;

extern crate ndarray;
use ndarray::Array2;

extern crate ndarray_linalg;
use ndarray_linalg::svd::SVD;

extern crate serde;
extern crate serde_pickle;

extern crate rusty_machine;
use rusty_machine::learning::SupModel;
use rusty_machine::learning::naive_bayes::{Multinomial, NaiveBayes};
use rusty_machine::linalg::Matrix;

use std::fs;
use std::fs::File;
use std::io::Read;

mod args;
use args::parse_args;

mod preprocessing;
use preprocessing::VOCAB;
use preprocessing::preprocess_file;

mod text;
use text::read_filenames_and_labels;

// function to get tokens from the whole training corpus
fn get_tokens_and_counts_from_corpus(
    directory: &str,
    labels_file: &str,
    filter_stopwords: bool,
    training_mode: bool,
) -> (
    Vec<Vec<(std::string::String, usize)>>,
    Vec<std::string::String>,
) {
    let dir = fs::read_dir(directory).unwrap();

    let mut files: Vec<Vec<(std::string::String, usize)>> = vec![];

    let filenames_and_labels: Vec<(std::string::String, std::string::String)> =
        read_filenames_and_labels(labels_file);
    let mut labels_ordered: Vec<std::string::String> = vec![];

    for (count, file) in dir.enumerate() {
        let file_path = file.unwrap().path();
        let file_full: &str = file_path.to_str().unwrap();
        let file_as_vec: Vec<&str> = file_full.split("/").collect();

        let file_short = file_as_vec[file_as_vec.len() - 1]; // gets filename after the last slash

        let mut filename_found = false;

        for (filename, label) in filenames_and_labels.iter() {
            if filename == file_short {
                labels_ordered.push(label.to_string());
                filename_found = true;
                break;
            }
        }

        if !filename_found {
            println!("{:?}{}", file_short, " wasn't found in labels file!");
            continue;
        }

        println!("{:?}", file_short);
        println!("{:?}", count + 1);

        let file_vector = preprocess_file(file_full, filter_stopwords, training_mode);

        match file_vector {
            Some(v) => files.push(v),
            None => continue,
        }
    }
    (files, labels_ordered)
}

fn get_tfdif_vectors(files: Vec<Vec<(std::string::String, usize)>>) -> Vec<Vec<f64>> {
    println!("{}", "Creating tf-idf vectors...");
    let mut tfidf_vectors: Vec<Vec<f64>> = vec![];
    let mut word_idf_scores: Vec<f64> = vec![];

    let vocab = VOCAB.lock().unwrap().to_vec();

    for (i, word) in vocab.iter().enumerate() {
        println!("{}{:?}{}{}", "Idf section: ", i + 1, " / ", vocab.len());
        let mut word_in_document_count = 0;
        for doc in files.iter() {
            for (w, _c) in doc.iter() {
                if w == word {
                    word_in_document_count += 1;
                    break;
                }
            }
        }

        if word_in_document_count > 0 {
            word_idf_scores.push((files.len() as f64 / word_in_document_count as f64).log2());
        } else {
            word_idf_scores.push(0f64); // word_in_document_count can be 0 when analysing test set
        }
    }

    for (i, doc) in files.iter().enumerate() {
        println!("{}{:?}{}{}", "Tf section: ", i + 1, " / ", files.len());
        let mut tfidf_vector: Vec<f64> = vec![0f64; vocab.len()];

        let mut total_words = 0;
        for (_word, count) in doc.iter() {
            total_words += count;
        }

        for (word, count) in doc.iter() {
            let position = vocab.iter().position(|t| t == word).unwrap();
            tfidf_vector[position] = *count as f64 / total_words as f64 * word_idf_scores[position];
        }
        tfidf_vectors.push(tfidf_vector);
    }
    tfidf_vectors
}

fn save_matrix_to_file(matrix: Vec<Vec<f64>>, file: &str) -> () {
    let matrix_pickled = serde_pickle::to_vec(&matrix, true).unwrap();
    fs::write(file, matrix_pickled).expect("Unable to write to file");
}

fn read_matrix_from_compressed_file(zip_file: &str) -> Vec<Vec<f64>> {
    let zip_file = File::open(zip_file).expect("Unable to read archive");
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    let mut f = zip_archive.by_index(0).unwrap(); // zip file cannot be accessed directly, has to be read from a zip archive
    let mut contents = Vec::new();
    f.read_to_end(&mut contents).unwrap();
    serde_pickle::from_slice(&contents).unwrap()
}

fn save_vector_to_file(vector: Vec<std::string::String>, file: &str) -> () {
    let vector_pickled = serde_pickle::to_vec(&vector, true).unwrap();
    fs::write(file, vector_pickled).expect("Unable to write to file");
}

fn read_vector_from_compressed_file(zip_file: &str) -> Vec<std::string::String> {
    let zip_file = File::open(zip_file).expect("Unable to read archive");
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    let mut f = zip_archive.by_index(0).unwrap();
    let mut contents = Vec::new();
    f.read_to_end(&mut contents).unwrap();
    serde_pickle::from_slice(&contents).unwrap()
}

// fn perform_svd(tfidf_vectors: Array2<f64>) {
//     // TODO even if we can somehow make this work for Array2, we will need to convert our Vec<Vec<f64>> into one
//     tfidf_vectors.svd(false, true);
// }

fn genre_labels_to_numbers(labels: Vec<std::string::String>) -> Vec<Vec<f64>> {
    let mut labels_as_numbers: Vec<Vec<f64>> = vec![];

    for label in labels {
        let label_as_string: std::string::String = label.to_owned();
        let label_as_slice: &str = &label_as_string[..];

        match label_as_slice {
            "detective" => labels_as_numbers.push(vec![1.0, 0.0, 0.0, 0.0, 0.0]),
            "erotica" => labels_as_numbers.push(vec![0.0, 1.0, 0.0, 0.0, 0.0]),
            "horror" => labels_as_numbers.push(vec![0.0, 0.0, 1.0, 0.0, 0.0]),
            "romance" => labels_as_numbers.push(vec![0.0, 0.0, 0.0, 1.0, 0.0]),
            "scifi" => labels_as_numbers.push(vec![0.0, 0.0, 0.0, 0.0, 1.0]),
            _ => continue,
        }
    }
    labels_as_numbers
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
fn get_naive_bayes_predictions(file_matrix: Vec<Vec<f64>>, texts: Vec<std::string::String>) -> () {
    let (file_rows, file_cols, file_matrix_flat) = matrix_to_vec(file_matrix);
    let (label_rows, label_cols, text_labels_as_numbers) =
        matrix_to_vec(genre_labels_to_numbers(texts));

    let inputs = Matrix::new(file_rows, file_cols, file_matrix_flat);
    let labels = Matrix::new(label_rows, label_cols, text_labels_as_numbers);

    let mut model = NaiveBayes::<Multinomial>::new();

    println!("{}", "Training Naive Bayes model...");
    model.train(&inputs, &labels).unwrap();

    let predictions = model.predict(&inputs).unwrap();

    println!("{:?}", predictions);
}

fn main() {
    let start = PreciseTime::now();

    // let matches = parse_args();

    // let train_dir = matches.value_of("TRAIN_DIR").unwrap_or("./train");

    // let train_labels_file = matches
    //     .value_of("TRAIN_LABELS")
    //     .unwrap_or("labels_train.txt"); // TODO change this to exit instead, no default here!

    // let test_dir = matches.value_of("TEST_DIR").unwrap_or("./test");

    // let test_labels_file = matches.value_of("TEST_LABELS").unwrap_or("labels_test.txt"); // TODO change this to exit instead, no default here!

    // let (train_files_and_counts, labels_train) = get_tokens_and_counts_from_corpus(
    //     train_dir,
    //     train_labels_file,
    //     matches.is_present("stopwords"),
    //     true,
    // );

    // let (test_files_and_counts, labels_test) = get_tokens_and_counts_from_corpus(
    //     test_dir,
    //     test_labels_file,
    //     matches.is_present("stopwords"),
    //     false,
    // );

    let tfidf_matrix_train = read_matrix_from_compressed_file("models/matrix_train.pickle.zip");
    let labels_train = read_vector_from_compressed_file("models/labels_train.pickle.zip");

    let tfidf_matrix_test = read_matrix_from_compressed_file("models/matrix_test.pickle.zip");
    let labels_test = read_vector_from_compressed_file("models/labels_test.pickle.zip");

    // get_naive_bayes_predictions(tfidf_matrix_train, labels_train);

    let end = PreciseTime::now();
    println!("This program took {} seconds to run", start.to(end));
}
