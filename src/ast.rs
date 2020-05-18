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

    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let func = match Function::parse(tokens) {
            Ok(result) => result,
            Err(msg) => return Err(msg),
        };

        Ok(Program::new(func))
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

    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let mut tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::IntKeyword {
            return Err(format!("Expected int token, got {}", tok.token_type()))
        }

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::Identifier {
            return Err(format!("Expected an identifier, got {}", tok.token_type()))
        }
        let function_identifier = String::from(tok.token());

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::OpenParen {
            return Err(format!("Expected an open paren, got {}", tok.token_type()))
        }

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::CloseParen {
            return Err(format!("Expected a close paren, got {}", tok.token_type()))
        }

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::OpenBracket {
            return Err(format!("Expected an open bracket, got {}", tok.token_type()))
        }

        let ret = match Return::parse(tokens) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        let func = Function::new(&function_identifier, ret);

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::CloseBracket {
            return Err(format!("Expected a close bracket, got {}", tok.token_type()));
        }

        Ok(func)
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

    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let mut tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::ReturnKeyword {
            return Err(format!("Expected a return keyword, got {}", tok.token_type()));
        }

        let exp = match IntConst::parse(tokens) {
            Ok(expression) => expression,
            Err(msg) => return Err(msg),
        };
        let ret = Return::new(exp);

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::SemiColon {
            return Err(format!("Expected a semicolon, got {}", tok.token_type()));
        }

        Ok(ret)
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

    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::IntLiteral {
            return Err(format!("Expected int token, got {}", tok.token_type()))
        }

        let val: i32 = match tok.token().parse() {
            Ok(value) => value,
            Err(msg) => return Err(format!("Failed to parse int: {}", msg)),
        };

        Ok(IntConst::new(val))
    }
}
