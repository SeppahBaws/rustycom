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

    pub fn to_asm(&self) -> String {
        let mut asm = String::new();

        /*
         * the "program" bit of assembly only has to define the start function:
         * .globl main
         * <program contents>
         */
        asm.push_str(" .globl ");
        asm.push_str(&self.function.identifier);
        asm.push('\n');
        asm.push_str(&self.function.to_asm());
        asm.push('\n');

        asm
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

    pub fn to_asm(&self) -> String {
        let mut asm = String::new();

        /*
         * a function may look like this:
         * main:
         *  <function body>
         */
        asm.push_str(&self.identifier);
        asm.push_str(":\n");
        asm.push_str(&self.statement.to_asm());

        asm
    }
}

#[derive(Debug)]
pub struct Return {
    expression: IntConst,
}

impl Return {
    fn new(exp: IntConst) -> Self {
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

        let exp = IntConst::parse(tokens);
        let ret = Return::new(exp);

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::SemiColon {
            // TODO: fail
        }

        ret
    }

    pub fn to_asm(&self) -> String {
        let mut asm = String::new();
        
        /*
         * Return statement looks like this:
         * movl $2, %eax
         * ret
         */
        asm.push_str(" movl $");
        asm.push_str(&self.expression.val.to_string());
        asm.push_str(", %eax\n");
        asm.push_str(" ret");

        asm
    }
}

#[derive(Debug)]
pub struct IntConst {
    val: i32,
}

impl IntConst {
    fn new(v: i32) -> Self {
        IntConst {
            val: v,
        }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> Self {
        let tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::IntLiteral {
            // TODO: fail
        }

        let val: i32 = tok.token().parse().expect("Failed to parse string to int");

        IntConst::new(val)
    }
}
