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
use select::predicate::{Attr, Class, Name, Predicate};

//stage1: preprocessing

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
fn preprocess_html(filename: &str){
    let mut text = "";
    let document = Document::from(include_str!("Arrangement.html"));
    for p in document.find(Name("p")) {
        text = text.to_owned().push_str(p.text());
        println!("{}",text);
    }

    }
//        find(Class("userstuff")) {
//        println!("{} ({:?})", node.text(),node.attr("p").unwrap());
//    }


//function to remove stuff from txt
//dunno yet good way of removing stuff except straightforward one
//but like it scares me how many times we will have to go through texts then
//maybe lets just hope that tf idf will take care of it?
//Maybe lets use GREP here?
//if we will create a text file with all relevant gutenberg shit
// we can just use grep on the content
//maybe with regex its gonna be faster
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


//main preprocessing function, where text is identified and True/False (stopwords) is in parameters

//function to make ngrams out of everything
//it is made on strings so far, im not sure will we need it or not and in which form
//is there a need in word tokenizer then or could we just skip a step?
fn make_ngrams<'a>(n: usize, text: &'a str) -> Vec<std::vec::Vec<&str>>{
    let mut ngrams: Vec<_> = text.split(' ').ngrams(n).collect();
    return ngrams;
}

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
    preprocess_html("Arrangement.html");


}
