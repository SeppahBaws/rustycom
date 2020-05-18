extern crate logos;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
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

    #[token = "return "]
    ReturnKeyword,

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

    pub fn token(&self) -> String {
        self.token.to_string()
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}

impl TokenType {
    fn get_display_name(&self) -> String {
        match self {
            TokenType::End => { String::from("End") },
            TokenType::Error => { String::from("Error") },
            TokenType::IntKeyword => { String::from("IntKeyword") },
            TokenType::Identifier => { String::from("Identifier") },
            TokenType::OpenParen => { String::from("OpenParen") },
            TokenType::CloseParen => { String::from("CloseParen") },
            TokenType::OpenBracket => { String::from("OpenBracket") },
            TokenType::CloseBracket => { String::from("CloseBracket") },
            TokenType::ReturnKeyword => { String::from("ReturnKeyword") },
            TokenType::IntLiteral => { String::from("IntLiteral") },
            TokenType::SemiColon => { String::from("SemiColon") },
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_display_name())
    }
}