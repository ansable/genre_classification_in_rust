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

extern crate la;
use la::SVD;
//TODO: rename it to some laMatrix
use la::Matrix as Matrix1;

extern crate serde;
extern crate serde_pickle;

extern crate rusty_machine;
use rusty_machine::learning::SupModel;
use rusty_machine::learning::naive_bayes::{Multinomial, NaiveBayes};
use rusty_machine::linalg::Matrix;

use std::fs;
use std::fs::File;
use std::io::Read;

use std::collections::HashSet;
use std::iter::FromIterator;

mod args;
use args::parse_args;

mod preprocessing;
use preprocessing::VOCAB;
use preprocessing::preprocess_file;

mod text;
use text::read_filenames_and_labels;

// function to get tokens from the whole training corpus
fn get_tokens_and_counts_from_corpus(
    corpus_path: &str,
    labels_file: &str,
    filter_stopwords: bool,
    training_mode: bool,
) -> (
    Vec<Vec<(std::string::String, usize)>>,
    Vec<std::string::String>,
) {
    let zip_file = File::open(corpus_path).expect("Unable to read archive");
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();

    let mut files_and_counts: Vec<Vec<(std::string::String, usize)>> = vec![];

    let filenames_and_labels: Vec<(std::string::String, std::string::String)> =
        read_filenames_and_labels(labels_file);
    let mut labels_ordered: Vec<std::string::String> = vec![];

    for i in 1..zip_archive.len() {
        let file = zip_archive.by_index(i).unwrap();
        let sanitized_name = file.sanitized_name();
        let filename = sanitized_name.to_str().unwrap();

        let filename_as_vec: Vec<&str> = filename.split("/").collect();

        let filename_short = filename_as_vec[filename_as_vec.len() - 1]; // gets filename after the last slash

        let mut filename_found = false;

        for (name, label) in filenames_and_labels.iter() {
            if name == filename_short {
                labels_ordered.push(label.to_string());
                filename_found = true;
                break;
            }
        }

        if !filename_found {
            println!("{:?}{}", filename_short, " wasn't found in labels file!");
            continue;
        }

        println!("{:?}", filename_short);
        println!("{:?}", i);

        let file_vector = preprocess_file(file, filter_stopwords, training_mode);

        match file_vector {
            Some(v) => files_and_counts.push(v),
            None => continue,
        }
    }
    (files_and_counts, labels_ordered)
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
            let position = vocab.binary_search(&word).unwrap();
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

//SVD (using la crate)
fn perform_svd(matrix: Vec<Vec<f64>>) -> (SVD<f64>, Matrix1<f64>){
    let (n_rows,n_cols,data) = matrix_to_vec(matrix);
    let mut la_matrix = Matrix1::new(n_rows,n_cols,data);
    let svd = SVD::new(&la_matrix);
    return (svd,la_matrix);
}

//fixed
fn svd_to_matrix(double: (SVD<f64>,Matrix1<f64>)) -> Matrix1<f64> {
    let (svd, matrix) = double;

    let s = svd.get_s();

    let mut diagonal = vec![];
    for i in 0..s.rows() {
        for x in 0..s.cols() {
            let item = s.get(i, x);
            if item != 0.0 {
                diagonal.push(item);
            }
        }
    }
    let s = &Matrix1::diag(diagonal);
    let mut zeros = Matrix1::zero(s.rows(), s.cols());
    let zeros = zeros.mt();
    let mut u = svd.get_u();

    let result = s.mmul(u,zeros);
    result.clone()
}
// this one doesn't seem to be working at all, but we can give it another chance on a larger data set
fn get_naive_bayes_predictions(
    train_matrix: Vec<Vec<f64>>,
    test_matrix: Vec<Vec<f64>>,
    label_vec: &Vec<std::string::String>,
) -> Matrix<f64> {
    let (train_rows, train_cols, train_matrix_flat) = matrix_to_vec(train_matrix);
    let (test_rows, test_cols, test_matrix_flat) = matrix_to_vec(test_matrix);
    let (label_rows, label_cols, text_labels_as_numbers) =
        matrix_to_vec(genre_labels_to_numbers(label_vec.to_vec()));

    let train = Matrix::new(train_rows, train_cols, train_matrix_flat);
    let test = Matrix::new(test_rows, test_cols, test_matrix_flat);
    let labels = Matrix::new(label_rows, label_cols, text_labels_as_numbers);

    let mut model = NaiveBayes::<Multinomial>::new();

    println!("{}", "Training Naive Bayes model...");
    model.train(&train, &labels).unwrap();

    model.predict(&test).unwrap()
}

fn save_pred_labels_to_vec(matrix: Matrix<f64>) -> Vec<std::string::String> {
    let mut preds = vec![];
    for i in 0..matrix.data().len() {
        if matrix.data()[i] == 1.0 {
            match i % 5 {
                0 => preds.push(String::from("detective")),
                1 => preds.push(String::from("erotica")),
                2 => preds.push(String::from("horror")),
                3 => preds.push(String::from("romance")),
                4 => preds.push(String::from("scifi")),
                _ => continue,
            }
        }
    }
    preds
}

// Precision: out of the times a label was predicted, {return values} of the time the system was in fact correct
fn precision(
    gold: &Vec<std::string::String>,
    pred: &Vec<std::string::String>,
    label: std::string::String,
) -> f64 {
    assert_eq!(
        gold.len(),
        pred.len(),
        "Label vectors must have the same length"
    );

    let mut total_predicted_for_label = 0;
    let mut true_positive_for_label = 0;

    if !pred.contains(&label) {
        return 0.0;
    }

    for i in 0..pred.len() {
        if pred[i] == label {
            total_predicted_for_label += 1;
            if gold[i] == label {
                true_positive_for_label += 1;
            }
        }
    }
    true_positive_for_label as f64 / total_predicted_for_label as f64
}

// Recall: out of the times a label should have been predicted, {return value} of the labels were correctly predicted
fn recall(
    gold: &Vec<std::string::String>,
    pred: &Vec<std::string::String>,
    label: std::string::String,
) -> f64 {
    let mut total_gold_for_label = 0;
    let mut true_positive_for_label = 0;

    if !gold.contains(&label) {
        return 0.0;
    }

    for i in 0..gold.len() {
        if gold[i] == label {
            total_gold_for_label += 1;
            if pred[i] == label {
                true_positive_for_label += 1;
            }
        }
    }
    true_positive_for_label as f64 / total_gold_for_label as f64
}

// F1: harmonic average of precision and recall
fn f1_score(precision: f64, recall: f64) -> f64 {
    if precision + recall == 0.0 {
        return 0.0;
    }

    2.0 * precision * recall / (precision + recall)
}

fn macro_averaged_evaluation(gold: Vec<std::string::String>, pred: Vec<std::string::String>) -> (f64, f64, f64) {
    let mut macro_averaged_precision = 0.0;
    let mut macro_averaged_recall = 0.0;
    let mut macro_averaged_f1_score = 0.0;

    // the following line assumes every label appears at least once in the training data
    let labels_set: HashSet<std::string::String> = HashSet::from_iter(gold.iter().cloned());

    for label in labels_set.iter() {
        macro_averaged_precision += precision(&gold, &pred, label.to_string());
        macro_averaged_recall += recall(&gold, &pred, label.to_string());
    }

    macro_averaged_precision = macro_averaged_precision / labels_set.len() as f64;
    macro_averaged_recall = macro_averaged_recall / labels_set.len() as f64;
    macro_averaged_f1_score = f1_score(macro_averaged_precision, macro_averaged_recall);


    (macro_averaged_precision, macro_averaged_recall, macro_averaged_f1_score)
}

fn main() {
    let start = PreciseTime::now();

    let matches = parse_args();

    let train_dir = matches.value_of("TRAIN_CORPUS").unwrap_or("./train.zip");

    let train_labels_file = matches
        .value_of("TRAIN_LABELS")
        .unwrap_or("labels_train.txt"); // TODO change this to exit instead, no default here!

    let test_dir = matches.value_of("TEST_CORPUS").unwrap_or("./test.zip");

    let test_labels_file = matches.value_of("TEST_LABELS").unwrap_or("labels_test.txt"); // TODO change this to exit instead, no default here!

    let (train_files_and_counts, labels_train) = get_tokens_and_counts_from_corpus(
        train_dir,
        train_labels_file,
        matches.is_present("stopwords"),
        true,
    );

    let tfidf_matrix_train = get_tfdif_vectors(train_files_and_counts);
    println!("{:?}", tfidf_matrix_train[0]);

    // let (test_files_and_counts, labels_test) = get_tokens_and_counts_from_corpus(
    //     test_dir,
    //     test_labels_file,
    //     matches.is_present("stopwords"),
    //     false,
    // );

    





    // println!("{:?}", "Loading training document matrix...");
    // let tfidf_matrix_train = read_matrix_from_compressed_file("models/matrix_train.pickle.zip");
    // let labels_train = read_vector_from_compressed_file("models/labels_train.pickle.zip");

    // println!("{:?}", "Loading test document matrix...");
    // let tfidf_matrix_test = read_matrix_from_compressed_file("models/matrix_test.pickle.zip");
    // let labels_test = read_vector_from_compressed_file("models/labels_test.pickle.zip");

    // let pred =
    //     save_pred_labels_to_vec(get_naive_bayes_predictions(tfidf_matrix_train, tfidf_matrix_test, &labels_train));

    // let (precision, recall, f1) = macro_averaged_evaluation(labels_test, pred);

    // println!("{}{:?}", "Precision: ", precision);
    // println!("{}{:?}", "Recall: ", precision);
    // println!("{}{:?}", "F1 score: ", precision);

    let end = PreciseTime::now();
    println!("This program took {} seconds to run", start.to(end));
}
