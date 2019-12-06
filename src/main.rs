extern crate rayon;

use std::env;
use std::collections::HashSet;
use rayon::prelude::*;

fn main() {
    let args : Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let results : Vec<String> = args[1..].iter().map(compress).collect();
    println!("{:?}", results);
}

fn compress(text: &String) -> std::string::String {
    let mut words : HashSet<String> = HashSet::new();
    for i in 0..text.len() - 1 {
        let word =  &text[i..i+2];
        if !word.contains(' ') {
            if text.matches(word).count() > 1 {
                words.insert(word.to_string());
            }
        }
    }
    let mut result = String::from(text);
    for word in words {
        result = result.replace(&word, "%");
    }
    // println!("{}", result);
    return result;
}
