use std;

use rusty_machine::learning::SupModel;
use rusty_machine::learning::naive_bayes::{Multinomial, NaiveBayes};
use rusty_machine::linalg::Matrix;

use model::matrix_to_vec;

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

pub fn get_naive_bayes_predictions(
    train_matrix: Vec<Vec<f64>>,
    test_matrix: Vec<Vec<f64>>,
    label_vec: &Vec<std::string::String>,
) -> Vec<std::string::String> {
    let (train_rows, train_cols, train_matrix_flat) = matrix_to_vec(train_matrix);
    let (test_rows, test_cols, test_matrix_flat) = matrix_to_vec(test_matrix);
    let (label_rows, label_cols, text_labels_as_numbers) =
        matrix_to_vec(genre_labels_to_numbers(label_vec.to_vec()));

    let train = Matrix::new(train_rows, train_cols, train_matrix_flat);
    let test = Matrix::new(test_rows, test_cols, test_matrix_flat);
    let labels = Matrix::new(label_rows, label_cols, text_labels_as_numbers);

    let mut model = NaiveBayes::<Multinomial>::new();

    model.train(&train, &labels).unwrap();

    save_pred_labels_to_vec(model.predict(&test).unwrap())
}
