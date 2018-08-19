extern crate scanlex;
extern crate select;
extern crate stopwords;

mod preprocessing;
use preprocessing::preprocess_file;
use preprocessing::tokenize_text;

fn main() {
    // Tests here for now TODO delete them
    // type_of_the_function
    // assert_eq!(type_of_file("james.txt"), "txt");
    // assert_eq!(type_of_file("marko.html"), "html");
    // assert!(type_of_file("anna is more than a file").is_err());
    // remove_stopwords
    // assert_eq!(remove_stopwords("a house tired is"), &["house", "tired"]);
    // tokenize text
    // assert_eq!(tokenize_text("??? who are! (CAT)))"),&["who", "are", "CAT"]);

    let lion = &preprocess_file(false, "train/a_lion.txt");
    println!("{:?}", lion);

    let baby = &preprocess_file(false, "train/baby.html");
    println!("{:?}", baby);
}
