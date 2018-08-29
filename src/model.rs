use std;
use std::fs::File;
use std::io::{BufRead, BufReader};

use zip::ZipArchive;

use preprocessing::VOCAB;
use preprocessing::preprocess_file;

fn read_filenames_and_labels(
    labels_file: &str,
) -> Vec<(std::string::String, std::string::String)> {
    let labels_file = File::open(labels_file).expect("File not found");
    let buf = BufReader::new(labels_file);

    let lines: Vec<std::string::String> = buf.lines()
        .map(|l| l.expect("Line could not be processed"))
        .collect();

    let mut filenames_and_labels: Vec<(std::string::String, std::string::String)> = vec![];

    for line in lines {
        let line_as_vec: Vec<std::string::String> =
            line.split_whitespace().map(|s| s.to_string()).collect();
        let filename = &line_as_vec[0];
        let label = &line_as_vec[1];

        filenames_and_labels.push((filename.to_string(), label.to_string()));
    }
    filenames_and_labels
}

// Function to get words from the whole training corpus
pub fn get_word_counts_from_corpus(
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

    let mut word_counts: Vec<Vec<(std::string::String, usize)>> = vec![];

    let filenames_and_labels: Vec<(std::string::String, std::string::String)> =
        read_filenames_and_labels(labels_file);
    let mut labels_ordered: Vec<std::string::String> = vec![];
    let number_of_files = zip_archive.len();

    for i in 1..number_of_files {
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

        println!("{}{}{}{}{}{:?}", "Processing document [", i, " / ", number_of_files-1, "] : ", filename_short);

        let word_count = preprocess_file(file, filter_stopwords, training_mode);

        match word_count {
            Some(v) => word_counts.push(v),
            None => continue,
        }
    }
    (word_counts, labels_ordered)
}

pub fn get_tfdif_vectors(files: Vec<Vec<(std::string::String, usize)>>) -> Vec<Vec<f64>> {
    let mut tfidf_vectors: Vec<Vec<f64>> = vec![];
    let mut word_idf_scores: Vec<f64> = vec![];

    let vocab = VOCAB.lock().unwrap().to_vec();

    for word in vocab.iter() {
        let mut word_in_document_count = 0;
        for doc in files.iter() {
            match doc.binary_search_by_key(&word, |&(ref w, _c)| w) {
                Ok(_) => word_in_document_count += 1,
                Err(_) => continue,
            }
        }

        if word_in_document_count > 0 {
            word_idf_scores.push((files.len() as f64 / word_in_document_count as f64).log2());
        } else {
            word_idf_scores.push(0f64); // word_in_document_count can be 0 when analysing test set
        }
    }

    for doc in files.iter() {
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