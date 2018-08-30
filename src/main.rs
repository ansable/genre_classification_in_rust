extern crate clap;
extern crate la;
#[macro_use]
extern crate lazy_static;
extern crate rusty_machine;
extern crate scanlex;
extern crate select;
extern crate serde;
extern crate serde_pickle;
extern crate stopwords;
extern crate zip;

use std::time::SystemTime;

use rusty_machine::learning::SupModel;
use rusty_machine::learning::naive_bayes::{Multinomial, NaiveBayes};
use rusty_machine::linalg::Matrix;

mod args;
use args::parse_args;

mod eval;
use eval::macro_averaged_evaluation;

mod model;
use model::{get_tfdif_vectors, get_word_counts_from_corpus, matrix_to_vec};

mod preprocessing;

mod save;
use save::{read_matrix_from_compressed_file, read_vector_from_compressed_file};

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

fn main() {
    let start_time = SystemTime::now();

    let args = parse_args();

    if args.is_present("autopilot") {
        // has to be called as "cargo run -- -a" (so as not to confuse with cargo arguments)
        println!("{}", "Loading training document matrix...");
        let tfidf_matrix_train = read_matrix_from_compressed_file("models/matrix_train.pickle.zip");
        let labels_train = read_vector_from_compressed_file("models/labels_train.pickle.zip");

        println!("{}", "Loading test document matrix...");
        let tfidf_matrix_test = read_matrix_from_compressed_file("models/matrix_test.pickle.zip");
        let labels_test = read_vector_from_compressed_file("models/labels_test.pickle.zip");

        let pred = save_pred_labels_to_vec(get_naive_bayes_predictions(
            tfidf_matrix_train,
            tfidf_matrix_test,
            &labels_train,
        ));

        let (precision, recall, f1) = macro_averaged_evaluation(labels_test, pred);

        println!("{}{:?}", "Precision: ", precision);
        println!("{}{:?}", "Recall: ", recall);
        println!("{}{:?}", "F1 score: ", f1);
    } else {
        let train_dir = args.value_of("TRAIN_CORPUS").unwrap_or("./train.zip");
        let test_dir = args.value_of("TEST_CORPUS").unwrap_or("./test.zip");

        let train_labels_file = args.value_of("TRAIN_LABELS").unwrap_or("labels_train.txt");
        let test_labels_file = args.value_of("TEST_LABELS").unwrap_or("labels_test.txt");

        println!("{}", "Extracting word counts for training data...");
        let (train_files_and_counts, labels_train) = get_word_counts_from_corpus(
            train_dir,
            train_labels_file,
            args.is_present("stopwords"),
            true,
        );

        println!("{}", "Creating training term-document matrix...");
        let tfidf_matrix_train = get_tfdif_vectors(train_files_and_counts);

        println!("{}", "Extracting word counts for test data...");
        let (test_files_and_counts, labels_test) = get_word_counts_from_corpus(
            test_dir,
            test_labels_file,
            args.is_present("stopwords"),
            false,
        );

        println!("{}", "Creating test term-document matrix...");
        let tfidf_matrix_test = get_tfdif_vectors(test_files_and_counts);

        println!("{}", "Training Naive Bayes model...");
        let pred = save_pred_labels_to_vec(get_naive_bayes_predictions(
            tfidf_matrix_train,
            tfidf_matrix_test,
            &labels_train,
        ));

        let (precision, recall, f1) = macro_averaged_evaluation(labels_test, pred);

        println!("{}{:?}", "Precision: ", precision);
        println!("{}{:?}", "Recall: ", recall);
        println!("{}{:?}", "F1 score: ", f1);
    }

    let duration = SystemTime::now()
        .duration_since(start_time)
        .expect("Time measurement failed");
    println!("This program took {:?} seconds to run", duration.as_secs());
}
