use std;
use std::io::Read;
use std::sync::Mutex;

use scanlex;

use select::document::Document;
use select::predicate::Attr;
use select::predicate::Name;

use stopwords::{Language, Spark, Stopwords};

use zip;


lazy_static! {
    pub static ref VOCAB: Mutex<Vec<std::string::String>> = Mutex::new(vec![]);
}

// function to identify type of the file. Returns String with: "html", "txt" or "other"
fn type_of_file(filename: &str) -> &str {
    let split_filename: Vec<&str> = filename.split(".").collect();

    // match against string following last dot in filename (should indicate file type)
    match split_filename[split_filename.len() - 1] {
        "txt" => "txt",
        "html" => "html",
        _ => "unknown",
    }
}

// Takes HTML file as parameter, extracts text from p tags and returns its contents as a vector of tokens
// following crate was consulted for reference:
// https://github.com/utkarshkukreti/select.rs/blob/master/tests/node_tests.rs
fn read_html_file(
    file: zip::read::ZipFile,
    filter_stopwords: bool,
    training_mode: bool,
) -> Result<Vec<(std::string::String, usize)>, std::io::Error> {
    let document = Document::from_read(file);

    let mut text = String::from(" ");

    for main_div in document.unwrap().find(Attr("id", "chapters")) {
        for p in main_div.find(Name("p")) {
            match &p.children().next() {
                Some(child) => text = format!("{}", text.to_owned() + &child.text()),
                None => continue,
            }
        }
    }

    return Ok(tokenize_and_count(&text, filter_stopwords, training_mode));
}

// Takes text file as argument, returns its contents as a vector of tokens
fn read_txt_file(
    mut file: zip::read::ZipFile,
    filter_stopwords: bool,
    training_mode: bool,
) -> Result<Vec<(std::string::String, usize)>, std::io::Error> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error encountered while processing file");

    return Ok(tokenize_and_count(
        &contents,
        filter_stopwords,
        training_mode,
    ));
}

// function to tokenize text, count tokens and build vocabulary
pub fn tokenize_and_count<'a>(
    text: &'a str,
    filter_stopwords: bool,
    training_mode: bool,
) -> Vec<(std::string::String, usize)> {
    let text = &str::replace(text, "'", " "); // scanlex crashes and burns upon encountering

    let mut scanner = scanlex::Scanner::new(&text);
    let mut tokens_and_counts: Vec<(std::string::String, usize)> = vec![];
    let mut vocab = VOCAB.lock().unwrap();

    let stopwords: Vec<_> = Spark::stopwords(Language::English)
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect();

    loop {
        let current_token = scanner.get();

        if current_token.is_iden() {
            let current_word = &current_token.to_iden().unwrap().to_lowercase();

            if filter_stopwords {
                if stopwords.contains(&current_word.to_string()) {
                    continue;
                }
            }

            if training_mode {
                if !vocab.contains(&current_word) {
                    vocab.push(current_word.to_string());
                }
            } else {
                if !vocab.contains(&current_word) {
                    continue;
                }
            }

            let mut token_in_tokens = false;
            let mut position_counter = 0;

            for &(ref token, _count) in tokens_and_counts.iter() {
                if token == &current_word.to_string() {
                    token_in_tokens = true;
                    break;
                }
                position_counter += 1;
            }
            if token_in_tokens {
                tokens_and_counts[position_counter].1 += 1;
            } else {
                tokens_and_counts.push((current_word.to_string(), 1));
            }
        } else if current_token.finished() {
            break;
        }
    }
    // sort vectors so that we can look for words in O(log n) time using binary search - important for calculating TF-IDF scores
    vocab.sort_unstable();
    tokens_and_counts.sort_unstable();
    tokens_and_counts
}

// main preprocessing function, where text is identified and True/False (stopwords) is in parameters
pub fn preprocess_file(
    file: zip::read::ZipFile,
    filter_stopwords: bool,
    training_mode: bool,
) -> Option<Vec<(std::string::String, usize)>> {
    let sanitized_name = file.sanitized_name();
    let filename = sanitized_name.to_str().unwrap();
    match type_of_file(filename) {
        "txt" => Some(read_txt_file(file, filter_stopwords, training_mode).unwrap()),
        "html" => Some(read_html_file(file, filter_stopwords, training_mode).unwrap()),
        _ => None,
    }
}
