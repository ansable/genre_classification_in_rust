extern crate scanlex;
extern crate select;
extern crate stopwords;

#[macro_use]
extern crate lazy_static;

use std::fs;
use std::collections::HashMap;

mod preprocessing;
use preprocessing::preprocess_file;
use preprocessing::tokenize_text;

// function to build empty hashmap with all words that appear in vocabulary
// fn build_vocab(train_directory: &str) -> HashMap<std::string::String, usize> {
//     let train_dir = fs::read_dir("./train").unwrap();

//     let mut vocab = HashMap::new();
//     for train_file in train_dir {
//         for token in preprocess_file(false, train_file.unwrap().path().to_str().unwrap()) {
//             if !vocab.contains_key(&token) {
//                 vocab.insert(token, 0);
//             }
//         }
//         println!("{:?}", vocab);
//     }
//     vocab
// }

fn main() {
    // type_of_the_function
    // assert_eq!(type_of_file("james.txt"), "txt");
    // assert_eq!(type_of_file("marko.html"), "html");
    // assert!(type_of_file("anna is more than a file").is_err());
    // remove_stopwords
    // assert_eq!(remove_stopwords("a house tired is"), &["house", "tired"]);
    // tokenize text
    // assert_eq!(tokenize_text("??? who are! (CAT)))"),&["who", "are", "CAT"]);

    let mut vocab = vec![];

    let lion = &preprocess_file(false, "train/ghost_of_alive.txt", vocab);
    println!("{:?}", lion);

    // let baby = &preprocess_file(false, "train/A Final Warning.html", vocab);
    // println!("{:?}", baby);

    // let vocab = build_vocab("./train");

    // println!("{:?}", vocab);
}
