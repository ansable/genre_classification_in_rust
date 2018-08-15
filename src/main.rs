//so far i ordered all imports according to functions, so it would be easier for you
//to understand what comes from what
//TODO: order them afterwards according common sense
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
//imports for stopwords function
extern crate stopwords;
use std::collections::HashSet;
use stopwords::{Spark, Language, Stopwords};

extern crate ngrams;
use ngrams::Ngram;

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
//TODO: function to extract p tags from html and make a text out of it

//function to remove stuff from txt
//dunno yet good way of removing stuff except straightforward one
//but like it scares me how many times we will have to go through texts then
//maybe lets just hope that tf idf will take care of it?
//but yeah have to remove the thing
fn preprocess_txt(filename: &str) ->  Result<(), std::io::Error> {
    let gutenberg = "This eBook is for the use of anyone anywhere in the United States and most
other parts of the world at no cost and with almost no restrictions
whatsoever. You may copy it, give it away or re-use it under the terms of
the Project Gutenberg License included with this eBook or online at
www.gutenberg.org.  If you are not located in the United States, you'll have
to check the laws of the country where you are located before using this ebook.";
    let input = File::open(filename).expect("Could not open settings file");
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

//function to tokenize text, using word tokenize

//function to apply stopwords
//fn remove_stopwords<'a>(line: &'a str) -> Vec<&'a str>{
//    let stopwords: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
//    line.retain(|s:String| !stopwords.contains(&s));
//    return line;
//}


//main preprocessing function, where text is identified and True/False (stopwords) is in parameters

//function to make ngrams out of everything
//fn make_ngrams<'a>(n: usize, text: &'a str) -> Vec<&'a str>{
//    let mut ngrams: Vec<_> = text.split(' ').ngrams(n).collect();
//    return ngrams;
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
    preprocess_txt("src/ch.txt");
    //remove_stopwords

}
