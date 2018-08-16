//so far i ordered all imports according to functions, so it would be easier for you
//to understand what comes from what
//TODO: order them afterwards according common sense
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
//imports for stopwords function
extern crate stopwords;
use std::collections::HashSet;
use stopwords::{Spark, Language, Stopwords};

extern crate scanlex;
use  scanlex::{Scanner,Token};

extern crate ngrams;
use ngrams::Ngram;


extern crate select;
use select::document::Document;
use std::io::Cursor;
use select::predicate::{Name};

//preprocessing - probs lets put it in separate file

//function to identify type of the file. Returns String with: "html", "txt" or "other"
    fn type_of_file(filename: &str) -> &str{
    if filename.contains(".txt"){
        return "txt";
    }
    else if filename.contains(".html") {
        return "html";
    }
    else {
        return "other";
    }
}

//function to extract p tags from html and make a text out of it
//tests and stuff to work easier with a crate
//https://github.com/utkarshkukreti/select.rs/blob/master/tests/node_tests.rs
fn preprocess_html(filename: &str) -> String{
    let mut f = File::open(filename).expect("file not found");
    let document = Document::from_read(f);
    assert!(document.is_ok());
    let mut text = String::from(" ");
    for p in document.unwrap().find(Name("p")) {
        text = format!("{}", text.to_owned() + &p.text());
        println!("{}", text);
    }
    return text;
    }


//function to remove stuff from txt
//not finished: it's just reading text so far
//reason: seems like very unproductive
//idea: use grep? don't do preprocessing for txt at all?
fn preprocess_txt(filename: &str) ->  Result<(), std::io::Error> {
    let input = File::open(filename).expect("Could not open settings file");
    let buffered = BufReader::new(input);
    for line in buffered.lines(){
        println!("{}", line?);
    }

    Ok(())
}

//function to tokenize text, using word tokenize
fn tokenize_text<'a>(text: &'a str) -> Vec<std::string::String>{
    let tokenized_text: Vec<_> = scanlex::Scanner::new(text).filter_map(|t| t.to_iden()).collect();
    return tokenized_text;
}




//function to apply stopwords
//fn remove_stopwords(line: String) -> std::string::String{
//    let stopwords: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
//    line.retain(|s| !stopwords.contains(s));
//    return line;
//}

//function to make ngrams out of everything
fn make_ngrams<'a>(n: usize, text: &'a str) -> Vec<std::vec::Vec<&str>>{
    let mut ngrams: Vec<_> = text.split(' ').ngrams(n).collect();
    return ngrams;
}

//main preprocessing function, where text is identified and True/False (stopwords) is in parameters
//fn preprocess(with_stopwords: bool, filename: &str) -> File{
//    match type_of_file(filename){
//        "txt" => preprocess_txt(filename),
//        "html" => preprocess_html(filename),
//        "other" => println!("This type of file is not supported")
//    }
//    if with_stopwords==False{
//        //remove_stopwords()
//    }
//}


fn main() {
    println!("Rust project created!");
    //tests for everything
    //TODO: obv delete in the end
    //stage1
    //type_of_the_function
    assert_eq!(type_of_file("james.txt"), "txt");
    assert_eq!(type_of_file("marko.html"), "html");
    assert_eq!(type_of_file("anna is more than a file"), "other");
    //remove_stopwords
    //tokenize text
    assert_eq!(tokenize_text("??? who are! (CAT)))"),&["who","are","CAT"]);
    let s = preprocess_html("Arrangement.html");
    println!("{}", s);

}
