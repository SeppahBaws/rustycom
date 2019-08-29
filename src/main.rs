use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

mod rusty_lexer;
mod token;
mod ast;

fn main() {
    if cfg!(not(target_os = "linux")) {
        panic!("this compiler currently only works on linux!");
    }

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = read_string(filename);
    println!("File name: {:?}", filename);
    println!("file content:\n{}", contents);


    println!("\n=================================\n");


    let mut tokens = rusty_lexer::RustyLexer::parse(contents.as_str());
    // println!("{:?}", tokens);

    let prog = ast::Program::parse(&mut tokens);
    println!("Parsing done: {:?}", prog);

    println!("Generating assembly...");
    generate_asm(&prog);

    println!("Compiling...");
    Command::new("gcc")
        .arg("assembly.s")
        .arg("-o")
        .arg("out")
        .status().expect("Failed to compile with gcc");
}

fn read_string(filename: &String) -> String {
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");
    
    contents
}

fn generate_asm(program: &ast::Program) {
    // Get the assembly code from the program node
    let contents = program.to_asm();
    
    // Write to file
    use std::error::Error;
    let mut file = match File::create("assembly.s") {
        Err(why) => panic!("Couldn't create file: {}", why.description()),
        Ok(file) => file,
    };
    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why.description()),
        Ok(_) => println!("Successfully wrote to file"),
    }
}