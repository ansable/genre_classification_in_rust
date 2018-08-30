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
extern crate time;
extern crate zip;

use la::Matrix as Matrix1; //TODO: rename it to some laMatrix
use la::SVD;

use rusty_machine::learning::naive_bayes::{Multinomial, NaiveBayes};
use rusty_machine::learning::SupModel;
use rusty_machine::linalg::Matrix;

use time::PreciseTime;

mod args;
use args::parse_args;

mod eval;
use eval::macro_averaged_evaluation;

mod model;
use model::{get_word_counts_from_corpus, get_tfdif_vectors};

mod preprocessing;

mod save;
use save::{read_vector_from_compressed_file, read_matrix_from_compressed_file};


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


//this does work but sadly the problem is in "svd" part (so not the one that we written) and the program is way too slow
fn perform_la_svd(matrix: Vec<Vec<f64>>) -> SVD<f64>{
    println!("{}", "Performing svd...");
    let (n_rows,n_cols,data) = matrix_to_vec(matrix);
    let mut la_matrix = Matrix1::new(n_rows,n_cols,data);
    let start = PreciseTime::now();
    let svd = SVD::new(&la_matrix);
    let end = PreciseTime::now();
    println!("{} seconds for svd performing.", start.to(end));
    svd
}

 fn la_svd_to_matrix(svd: SVD<f64>,n_elements: usize) -> Matrix1<f64> {
    println!("{}", "Transforming svd...");
    let start = PreciseTime::now();
    let mut s = svd.get_s();
    let s = &s.sub_matrix(0..s.rows(),0..n_elements);
    let mut u = svd.get_u();
    let result = u * s;
    println!("{:?}",result);
    let end = PreciseTime::now();
    println!("{} seconds for svd transforming.", start.to(end));
    result.clone()
}

//la::Matrix to Vec<f64>>
fn from_la_to_vec(mut lamatrix: Matrix1<f64>) -> (usize,usize,Vec<f64>){
    let tmp = lamatrix.mt();
    return (tmp.cols(),tmp.rows(),tmp.get_mut_data().to_vec());
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
    let start = PreciseTime::now();

    let args = parse_args();

    if args.is_present("autopilot") { // has to be called as "cargo run -- -a" (so as not to confuse with cargo arguments)
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

    let end = PreciseTime::now();
    println!("This program took {} seconds to run", start.to(end));
}
