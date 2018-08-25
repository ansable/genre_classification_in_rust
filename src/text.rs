use std;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
pub struct Text {
    pub filename: std::string::String,
    pub label: std::string::String,
}

pub fn read_filenames_and_labels(train_labels_file: &str) -> Vec<Text> {
    let train_labels_file = File::open(train_labels_file).expect("File not found");
    let buf = BufReader::new(train_labels_file);

    let lines: Vec<std::string::String> = buf.lines()
        .map(|l| l.expect("Line could not be processed"))
        .collect();

    let mut filenames_and_labels: Vec<Text> = vec![];

    for line in lines {
        let line_as_vec: Vec<std::string::String> =
            line.split_whitespace().map(|s| s.to_string()).collect();
        let filename = &line_as_vec[0];
        let label = &line_as_vec[1];

        filenames_and_labels.push(Text {
            filename: filename.to_string(),
            label: label.to_string(),
        })
    }
    filenames_and_labels
}
