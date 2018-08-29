extern crate clap;
extern crate scanlex;
extern crate select;
extern crate stopwords;
extern crate time;

use time::PreciseTime;

#[macro_use]
extern crate lazy_static;

extern crate la;
use la::SVD;
//TODO: rename it to some laMatrix
use la::Matrix as Matrix1;

extern crate ndarray_linalg;
use ndarray_linalg::svd::SVD;

extern crate serde;
extern crate serde_pickle;

extern crate rusty_machine;
use rusty_machine::learning::SupModel;
use rusty_machine::learning::naive_bayes::{Multinomial, NaiveBayes};
use rusty_machine::linalg::Matrix;

use std::fs;

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

fn read_matrix_from_file(file: &str) -> Vec<Vec<f64>> {
    let matrix = fs::read(file).expect("Unable to read file");
    serde_pickle::from_slice(&matrix).unwrap()
}

fn save_vector_to_file(vector: Vec<std::string::String>, file: &str) -> () {
    let vector_pickled = serde_pickle::to_vec(&vector, true).unwrap();
    fs::write(file, vector_pickled).expect("Unable to write to file");
}

fn read_vector_from_file(file: &str) -> Vec<std::string::String> {
    let vector = fs::read(file).expect("Unable to read file");
    serde_pickle::from_slice(&vector).unwrap()
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

//SVD (using la crate)

fn perform_svd(matrix: Vec<Vec<f64>>) -> (SVD<f64>, Matrix1<f64>){
    let (n_rows,n_cols,data) = matrix_to_vec(matrix);
    let mut la_matrix = Matrix1::new(n_rows,n_cols,data);
    let svd = SVD::new(&la_matrix);
    return (svd,la_matrix);
}

//this thing rn under reconstruction, i dont like shapes im getting

fn svd_to_matrix(double: (SVD<f64>,Matrix1<f64>),n_elements:usize) -> () {
    let (svd,matrix) = double;
//    let mut v = svd.get_v();
//    let v = &Matrix1::sub_matrix(&v,n_elements,v.cols());
    let mut s = svd.get_s();
    let mut diagonal = vec![];
    for i in 0..s.rows() {
        for x in 0..s.cols() {
            let item = s.get(i,x);
            if item!=0.0 {
                diagonal.push(item);
            }
        }
    }
    let s = &Matrix1::diag(diagonal);
    let transform = s.dot(svd.get_u());
    println!("{:?}", transform);
//why dont they have a special method for this cases?
    // great rhetorical question to ask
    //so here we go
    // a terribly ineffective iteration
    //there should be a better way, but i dont have internet so should use my own brains

//    println!("{:?}", diagonal);
//    //n*n orthogonal, we will not need it, so its more a reference for you
//    //so you will not have to read bunch of shitty docs
////    let v = svd.get_v();
//    //now we have to cut it to n*n matrix built around s diagonal
//    let new_diagonal = &diagonal[0..n_cols];
//    let mut first_step = Matrix1::diag(new_diagonal.to_vec());
//    println!("{:?}", first_step);
//    let mut smaller_matrix = Matrix1::sub_matrix(&first_step,first_step.rows(),n_elements);
//    println!("{:?}", smaller_matrix);
//    //now we're using same selected amount of elements to make a submatrix out of V
////    let mut v = Matrix1::get_rows(&v,n_elements);
//    //now we have two ways of getting the matrix we need, so we can check that it was actually
//    //correctly performed
//    let transform1 = &u.dot(&smaller_matrix);
//    println!("{:?}", transform1);
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

    let tfidf_matrix_train = read_matrix_from_file("matrix_train.pickle");
    let labels_train = read_vector_from_file("labels_train.pickle");

    let tfidf_matrix_test = read_matrix_from_file("matrix_test.pickle");
    let labels_test = read_vector_from_file("labels_test.pickle");
    let smth = get_tfdif_vectors(all_files,labels_ordered);
    svd_to_matrix(perform_svd(smth),2);
    //so yep this shit works
    //now it's time to create a matrix
//    println!("{:?}",svd.get_u());
   // println!("{:?}",svd.get_s());
//    println!("{:?}",svd.get_v());
    // get_naive_bayes_predictions(tfidf_matrix_train, labels_train);

    let end = PreciseTime::now();
    println!("This program took {} seconds to run", start.to(end));
}
