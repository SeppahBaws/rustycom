extern crate logos;
use logos::Logos;

use std::collections::VecDeque;

use crate::token::{
    Token,
    TokenType,
};

pub struct RustyLexer {
    
}

impl RustyLexer {
    pub fn parse(contents: &str) -> VecDeque<Token> {
        let mut lexer = TokenType::lexer(contents);

        let mut tokens: VecDeque<Token> = VecDeque::new();

        loop {
            match lexer.token {
                TokenType::End => break,
                TokenType::Error => {
                    println!("Error while parsing! Unidentified token {}", lexer.slice());
                    break;
                },
                _ => {
                    let token = Token::new(&lexer.token, &lexer.slice());
                    tokens.push_back(token);
                    // println!("{:?}", token);
                }
            }

            lexer.advance();
        }

        tokens
    }
}
