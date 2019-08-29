use crate::token::{
    Token,
    TokenType,
};

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Program {
    function: Function,
}

impl Program {
    fn new(func: Function) -> Self {
        Self {
            function: func,
        }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> Self {
        let func = Function::parse(tokens);
        Program::new(func)
    }
}

#[derive(Debug)]
pub struct Function {
    identifier: String,
    statement: Return,
}

impl Function {
    fn new(id: &String, ret: Return) -> Self {
        Self {
            identifier: id.clone(),
            statement: ret,
        }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> Self {
        let mut tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::IntKeyword {
            // TODO: fail
        }

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::Identifier {
            // TODO: fail
        }
        let function_identifier = String::from(tok.token());

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::OpenParen {
            // TODO: fail
        }

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::CloseParen {
            // TODO: fail
        }

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::OpenBracket {
            // TODO: fail
        }

        let ret = Return::parse(tokens);
        let func = Function::new(&function_identifier, ret);

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::CloseBracket {
            // TODO: fail
        }

        func
    }
}

#[derive(Debug)]
pub struct Return {
    expression: Expression,
}

impl Return {
    fn new(exp: Expression) -> Self {
        Self {
            expression: exp,
        }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> Self {
        let mut tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::ReturnKeyword {
            // TODO: fail
        }

        // TODO: check the type of the next token without having to pop it twice
        // tok = tokens.pop_front().unwrap();
        // if tok.token_type() != TokenType::IntLiteral {
        //     // TODO: fail
        // }

        let exp = Expression::parse(tokens);
        let ret = Return::new(exp);

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::SemiColon {
            // TODO: fail
        }

        ret
    }
}

#[derive(Debug)]
pub struct Expression {
    val: i32,
}

impl Expression {
    fn new(v: i32) -> Self {
        Self {
            val: v,
        }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> Self {
        let tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::IntLiteral {
            // TODO: fail
        }

        let val: i32 = tok.token().parse().expect("Failed to parse string to int");

        Expression::new(val)
    }
}
