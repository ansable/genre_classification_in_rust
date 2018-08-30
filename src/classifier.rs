// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Final Project
// Authors:     Anna Soboleva, Marko Ložajić
//
// Honor Code:  We pledge that this program represents our own work.

/// Module containing a simple Naïve Bayes classifier (as found in the rusty-machine crate) and associated helper methods.
use std;

use rusty_machine::learning::SupModel;
use rusty_machine::learning::naive_bayes::{Multinomial, NaiveBayes};
use rusty_machine::linalg::Matrix;

// Function to encode labels in a format understood by the rusty-machine Naive Bayes classifier.
fn genre_labels_to_onehot(labels: Vec<std::string::String>) -> Vec<Vec<f64>> {
    let mut labels_as_numbers: Vec<Vec<f64>> = vec![];

    for label in labels {
        let label_as_string: std::string::String = label.to_owned();
        let label_as_slice: &str = &label_as_string[..]; // std::string::String must be converted to &str for the match expression to work

        // a list containing all label types could be used to unambiguously encode an arbitrarily large number of different labels,
        // but since we were working with a fixed label set we decided to sacrifice this flexibility for speed and simplicity
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

// Function to decode matrix of one-hot label representations as their actual (string) values
fn genre_labels_from_onehot(matrix: Matrix<f64>) -> Vec<std::string::String> {
    let mut preds = vec![];
    for i in 0..matrix.data().len() {
        if matrix.data()[i] == 1.0 {
            match i % 5 {
                // the vector is underlyingly divided into one-hot blocks of length five
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

// Converts a two-dimensional vector to the matrix representation familiar to several crates in the project, namely rusty-machine and la.
pub fn matrix_to_vec(matrix: Vec<Vec<f64>>) -> (usize, usize, Vec<f64>) {
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

// Function that computes the model predictions, using rusty-machine's Naive Bayes classifier and assuming a Multinomial distribution.
pub fn get_naive_bayes_predictions(
    train_matrix: Vec<Vec<f64>>,
    test_matrix: Vec<Vec<f64>>,
    label_vec: &Vec<std::string::String>,
) -> Vec<std::string::String> {
    let (train_rows, train_cols, train_matrix_flat) = matrix_to_vec(train_matrix);
    let (test_rows, test_cols, test_matrix_flat) = matrix_to_vec(test_matrix);
    let (label_rows, label_cols, text_labels_as_numbers) =
        matrix_to_vec(genre_labels_to_onehot(label_vec.to_vec()));

    let train = Matrix::new(train_rows, train_cols, train_matrix_flat);
    let test = Matrix::new(test_rows, test_cols, test_matrix_flat);
    let labels = Matrix::new(label_rows, label_cols, text_labels_as_numbers);

    let mut model = NaiveBayes::<Multinomial>::new();

    model.train(&train, &labels).unwrap();

    genre_labels_from_onehot(model.predict(&test).unwrap())
}
