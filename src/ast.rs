use crate::token::{
    Token,
    TokenType,
};

pub struct Program {
    function: Function,
}

impl Program {
    pub fn new(func: Function) -> Self {
        Self {
            function: func,
        }
    }
}

pub struct Function {
    statement: Return,
}

impl Function {
    pub fn new(ret: Return) -> Self {
        Self {
            statement: ret,
        }
    }
}

pub struct Return {
    expression: Expression,
}

impl Return {
    pub fn new(exp: Expression) -> Self {
        Self {
            expression: exp,
        }
    }
}

pub struct Expression {
    val: i32,
}

impl Expression {
    pub fn new(v: i32) -> Self {
        Self {
            val: v,
        }
    }
}

// pub struct Return {
//     expression: Expression,
// }

// impl Return {
//     pub fn new() ->  {

//     }
// }

// #[derive(Debug)]
// pub struct IntExpression {
//     val: i32,
// }

// impl IntExpression {
//     pub fn parse_expression(tok: &str) -> Self {
//         Self {
//             val: tok.parse().unwrap(),
//         }
//     }

//     // pub fn parse_expression_vec(tokens: &Vec<Token>, index: &mut usize) -> Self {
//     //     let current_token = tokens[index];

//     //     if current_token.token_type() != TokenType::IntLiteral {
//     //         println!("Invalid TokenType! Expected TokenType::IntLiteral, got {:?}", current_token.token_type());
//     //         return Self {
//     //             val: -1,
//     //         };
//     //     }

//     //     let val: i32 = current_token.token().parse().unwrap();

//     //     index += 1;

//     //     Self {
//     //         val: val,
//     //     }
//     // }
// }