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
    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let function = match Function::parse(tokens) {
            Ok(result) => result,
            Err(msg) => return Err(msg),
        };

        Ok(Self { function })
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
    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let mut tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::IntKeyword {
            return Err(format!("Expected int token, got {}", tok.token_type()))
        }

        tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::Identifier {
            return Err(format!("Expected an identifier, got {}", tok.token_type()))
        }
        let identifier = String::from(tok.token());

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

        let statement = match Return::parse(tokens) {
            Ok(value) => value,
            Err(msg) => return Err(msg),
        };
        let func = Self { identifier, statement };

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
    expression: Expression,
}

impl Return {
    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let mut tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::ReturnKeyword {
            return Err(format!("Expected a return keyword, got {}", tok.token_type()));
        }

        let expression = match Expression::parse(tokens) {
            Ok(expression) => expression,
            Err(msg) => return Err(msg),
        };
        let ret = Self { expression };

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
        asm.push_str(&self.expression.to_asm());
        asm.push_str(" ret");

        asm
    }
}

#[derive(Debug)]
pub struct Expression {
    int_const: Option<Box<IntConst>>,
    unary_op: Option<Box<(TokenType, Expression)>>,
}

impl Expression {
    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let tok = tokens.front().unwrap();

        return match tok.token_type() {
            TokenType::IntLiteral => {
                let int = match IntConst::parse(tokens) {
                    Ok(int) => int,
                    Err(msg) => return Err(msg),
                };

                Ok(Self {
                    int_const: Some(Box::new(int)),
                    unary_op: None,
                })
            },
            TokenType::LogicalNegation => {
                let _ = tokens.pop_front().unwrap(); // pop the token from the front

                let exp = match Expression::parse(tokens) {
                    Ok(exp) => exp,
                    Err(msg) => return Err(msg),
                };

                Ok(Self {
                    int_const: None,
                    unary_op: Some(Box::new((TokenType::LogicalNegation, exp))),
                })
            },
            TokenType::Negation => {
                let _ = tokens.pop_front().unwrap(); // pop the token from the front

                let exp = match Expression::parse(tokens) {
                    Ok(exp) => exp,
                    Err(msg) => return Err(msg),
                };

                Ok(Self {
                    int_const: None,
                    unary_op: Some(Box::new((TokenType::Negation, exp)))
                })
            },
            TokenType::BitwiseCompliment => {
                let _ = tokens.pop_front().unwrap(); // pop the token from the front

                let exp = match Expression::parse(tokens) {
                    Ok(exp) => exp,
                    Err(msg) => return Err(msg),
                };

                Ok(Self {
                    int_const: None,
                    unary_op: Some(Box::new((TokenType::BitwiseCompliment, exp))),
                })
            },
            token_type => Err(format!("Failed to parse Expression, got {}", token_type)),
        }
    }

    pub fn to_asm(&self) -> String {
        let mut asm = String::new();

        if self.int_const.is_some() {
            asm.push_str(&self.int_const.as_ref().unwrap().to_asm());
        } else if self.unary_op.is_some() {
            // let a = *self.unary_op.unwrap();

            let (tok_type, exp) = &**self.unary_op.as_ref().unwrap();

            // let (tok_type, exp) = *self.unary_op.unwrap();
            // let (tok_type, exp) = *self.unary_op.as_ref().unwrap();
            // let a = self.unary_op.as_ref().unwrap().unbox();

            // asm.push_str(&exp.to_asm());
            // a.
            asm.push_str(&exp.to_asm());

            match tok_type {
                TokenType::Negation => {
                    asm.push_str(" neg %eax\n");
                },
                TokenType::BitwiseCompliment => {
                    asm.push_str(" not %eax\n");
                },
                TokenType::LogicalNegation => {
                    asm.push_str(" cmpl $0, %eax\n");
                    asm.push_str(" movl $0, %eax\n");
                    asm.push_str(" sete %al\n");
                },
                _ => {},
            }
        }

        asm
    }
}

#[derive(Debug)]
pub struct IntConst {
    val: i32,
}

impl IntConst {
    pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        let tok = tokens.pop_front().unwrap();
        if tok.token_type() != TokenType::IntLiteral {
            return Err(format!("Expected int token, got {}", tok.token_type()));
        }

        let val: i32 = match tok.token().parse() {
            Ok(value) => value,
            Err(msg) => return Err(format!("Failed to parse int: {}", msg)),
        };

        Ok(Self { val })
    }

    pub fn to_asm(&self) -> String {
        let mut asm = String::new();

        /*
         * To put an int in the EAX register looks like this:
         * movl $2, %eax
         */
        asm.push_str(" movl $");
        asm.push_str(&self.val.to_string());
        asm.push_str(", %eax\n");
        asm
    }
}
