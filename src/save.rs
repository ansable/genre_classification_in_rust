// Module for saving/loading models (serialising/deserialising)
use std;
use std::fs;
use std::fs::File;
use std::io::Read;

use serde_pickle;

use zip::ZipArchive;


pub fn save_matrix_to_file(matrix: Vec<Vec<f64>>, file: &str) -> () {
    let matrix_pickled = serde_pickle::to_vec(&matrix, true).unwrap();
    fs::write(file, matrix_pickled).expect("Unable to write to file");
}

pub fn read_matrix_from_compressed_file(zip_file: &str) -> Vec<Vec<f64>> {
    let zip_file = File::open(zip_file).expect("Unable to read archive");
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    let mut f = zip_archive.by_index(0).unwrap(); // zip file cannot be accessed directly, has to be read from a zip archive
    let mut contents = Vec::new();
    f.read_to_end(&mut contents).unwrap();
    serde_pickle::from_slice(&contents).unwrap()
}

pub fn save_vector_to_file(vector: Vec<std::string::String>, file: &str) -> () {
    let vector_pickled = serde_pickle::to_vec(&vector, true).unwrap();
    fs::write(file, vector_pickled).expect("Unable to write to file");
}

pub fn read_vector_from_compressed_file(zip_file: &str) -> Vec<std::string::String> {
    let zip_file = File::open(zip_file).expect("Unable to read archive");
    let mut zip_archive = ZipArchive::new(zip_file).unwrap();
    let mut f = zip_archive.by_index(0).unwrap();
    let mut contents = Vec::new();
    f.read_to_end(&mut contents).unwrap();
    serde_pickle::from_slice(&contents).unwrap()
}
