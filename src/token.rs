extern crate logos;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenType {
    #[end]
    End,
    #[error]
    Error,

    #[token = "int"]
    IntKeyword,

    #[regex = "[a-zA-Z]+"]
    Identifier,

    #[token = "("]
    OpenParen,
    #[token = ")"]
    CloseParen,
    #[token = "{"]
    OpenBracket,
    #[token = "}"]
    CloseBracket,

    #[token = "return"]
    ReturnStatement,

    #[regex = "[0-9]+"]
    IntLiteral,

    #[token = ";"]
    SemiColon,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    token: String,
}

impl Token {
    pub fn new(tok_type: &TokenType, tok: &str) -> Token {
        
        Self {
            token_type: Clone::clone(tok_type),
            token: String::from(tok),
        }
    }

    pub fn token(self) -> String {
        self.token
    }

    pub fn token_type(self) -> TokenType {
        self.token_type
    }
}
