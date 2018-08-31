// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Final Project
// Authors:     Anna Soboleva, Marko Ložajić
//
// Honor Code:  We pledge that this program represents our own work.

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

mod args;
use args::parse_args;

mod classifier;
use classifier::get_naive_bayes_predictions;

mod eval;
use eval::macro_averaged_evaluation;

mod model;
use model::{get_tfdif_vectors, get_word_counts_for_corpus};

mod preprocessing;

mod save;
use save::{read_matrix_from_compressed_file, read_vector_from_compressed_file};

fn main() {
    let start_time = SystemTime::now();

    let args = parse_args();

    if args.is_present("autopilot") {
        println!("{}", "Loading training document matrix and labels...");
        let tfidf_matrix_train = read_matrix_from_compressed_file("models/matrix_train.pickle.zip");
        let labels_train = read_vector_from_compressed_file("models/labels_train.pickle.zip");

        println!("{}", "Loading test document matrix and labels...");
        let tfidf_matrix_test = read_matrix_from_compressed_file("models/matrix_test.pickle.zip");
        let labels_test = read_vector_from_compressed_file("models/labels_test.pickle.zip");

        let pred =
            get_naive_bayes_predictions(tfidf_matrix_train, tfidf_matrix_test, &labels_train);

        let (precision, recall, f1) = macro_averaged_evaluation(&labels_test, &pred);

        println!("{}{:?}", "Precision: ", precision);
        println!("{}{:?}", "Recall: ", recall);
        println!("{}{:?}", "F1 score: ", f1);
    } else {
        let train_dir = args.value_of("TRAIN_CORPUS").unwrap_or("./train.zip");
        let test_dir = args.value_of("TEST_CORPUS").unwrap_or("./test.zip");

        let train_labels_file = args.value_of("TRAIN_LABELS").unwrap_or("labels_train.txt");
        let test_labels_file = args.value_of("TEST_LABELS").unwrap_or("labels_test.txt");

        println!("{}", "Extracting word counts for training data...");
        let (train_files_and_counts, labels_train) = get_word_counts_for_corpus(
            train_dir,
            train_labels_file,
            args.is_present("stopwords"),
            true,
        );

        println!("{}", "Creating training term-document matrix...");
        let tfidf_matrix_train = get_tfdif_vectors(train_files_and_counts);

        println!("{}", "Extracting word counts for test data...");
        let (test_files_and_counts, labels_test) = get_word_counts_for_corpus(
            test_dir,
            test_labels_file,
            args.is_present("stopwords"),
            false,
        );

        println!("{}", "Creating test term-document matrix...");
        let tfidf_matrix_test = get_tfdif_vectors(test_files_and_counts);

        println!("{}", "Training Naive Bayes model...");
        let pred =
            get_naive_bayes_predictions(tfidf_matrix_train, tfidf_matrix_test, &labels_train);

        let (precision, recall, f1) = macro_averaged_evaluation(&labels_test, &pred);

        println!("{}{:?}", "Precision: ", precision);
        println!("{}{:?}", "Recall: ", recall);
        println!("{}{:?}", "F1 score: ", f1);
    }

    let duration = SystemTime::now()
        .duration_since(start_time)
        .expect("Time measurement failed");
    println!("This program took {:?} seconds to run", duration.as_secs());
}
