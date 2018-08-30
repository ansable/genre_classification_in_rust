// Course:      Efficient Linear Algebra and Machine Learning
// Assignment:  Final Project
// Authors:     Anna Soboleva, Marko Ložajić
//
// Honor Code:  We pledge that this program represents our own work.

/// Module for saving/loading models (serializing/deserializing).
/// The saving functions are currently unused in the project, but they were used to store the term-document vectors in order to
/// facilitate evaluation, and were kept for the sake of completeness / future reference.
use std;
use std::fs;
use std::fs::File;
use std::io::Read;

use serde_pickle;

use zip::ZipArchive;

// Function to save a matrix (two-dimensional vector of floats) to a file in the pickle format, enabling later quick access.
// In practice, this function was used to save the term-document matrix.
pub fn save_matrix_to_file(matrix: Vec<Vec<f64>>, file: &str) -> () {
    let matrix_pickled = serde_pickle::to_vec(&matrix, true).unwrap();
    fs::write(file, matrix_pickled).expect("Unable to write to file");
}

// Loads the aforementioned matrix from its pickled home (has to be compressed beforehand)
pub fn read_matrix_from_compressed_file(zip_file: &str) -> Vec<Vec<f64>> {
    let zip_file = File::open(zip_file).expect("Unable to read archive");
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    let mut f = zip_archive.by_index(0).unwrap(); // zip file cannot be accessed directly, has to be read from a zip archive
    let mut contents = Vec::new();
    f.read_to_end(&mut contents).unwrap();
    serde_pickle::from_slice(&contents).unwrap()
}

// Function to save vector of strings to a file in the pickle format.
// Was used to store the labels vector in the order corresponding to the term-document matrix rows.
pub fn save_vector_to_file(vector: Vec<std::string::String>, file: &str) -> () {
    let vector_pickled = serde_pickle::to_vec(&vector, true).unwrap();
    fs::write(file, vector_pickled).expect("Unable to write to file");
}

// Loads the aforementioned vector from its pickled home (has to be compressed beforehand)
pub fn read_vector_from_compressed_file(zip_file: &str) -> Vec<std::string::String> {
    let zip_file = File::open(zip_file).expect("Unable to read archive");
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    let mut f = zip_archive.by_index(0).unwrap();
    let mut contents = Vec::new();
    f.read_to_end(&mut contents).unwrap();
    serde_pickle::from_slice(&contents).unwrap()
}
