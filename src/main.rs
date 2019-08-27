use std::env;
use std::fs::File;
use std::io::prelude::*;

mod rusty_lexer;
mod token;
mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = read_string(filename);
    println!("File name: {:?}", filename);
    println!("file content:\n{}", contents);


    println!("\n=================================\n");


    let mut tokens = rusty_lexer::RustyLexer::parse(contents.as_str());
    println!("{:?}", tokens);

    let first_token = tokens.pop_front().expect("Can't pop front");
    println!("first token: {:?}", first_token);
}

fn read_string(filename: &String) -> String {
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");
    
    contents
}