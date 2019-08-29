use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

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
    // println!("{:?}", tokens);

    let prog = ast::Program::parse(&mut tokens);
    println!("Parsing done: {:?}", prog);

    demo_generate_asm(&prog);
    if cfg!(target_os = "linux") {
        Command::new("gcc")
                .arg("assembly.s")
                .arg("-o out");
    }
}

fn read_string(filename: &String) -> String {
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");
    
    contents
}

fn demo_generate_asm(program: &ast::Program) {
    use std::error::Error;

    let mut file = match File::create("assembly.s") {
        Err(why) => panic!("Couldn't create file: {}", why.description()),
        Ok(file) => file,
    };

    let func_name = program.get_func().get_identifier().to_owned();
    let ret_val = program.get_func().get_statement().get_expression().get_value();
    
    let contents: String = format!(" .globl {}\n{}:\n movl ${}, %eax\n ret\n", func_name, func_name, ret_val);
    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why.description()),
        Ok(_) => println!("Successfully wrote to file"),
    }
}