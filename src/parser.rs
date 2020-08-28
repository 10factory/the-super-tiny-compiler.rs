use std::iter::Peekable;
use std::vec::IntoIter;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Expr {
    NumberLiteral(String),
    StringLiteral(String),
    CallExpression(String, Vec<Expr>),
}

#[derive(Debug)]
pub struct Program {
    body: Vec<Expr>,
}

fn walk(tokens: &mut Peekable<IntoIter<Token>>) -> Expr {
    if let Some(token) = tokens.next() {
        return match token {
            Token::Number(value) => Expr::NumberLiteral(String::from(value)),
            Token::TString(value) => Expr::StringLiteral(String::from(value)),
            Token::Paren(value) if value == '(' => {
                if let Some(next) = tokens.next() {
                    match next {
                        Token::Name(name) => {
                            let mut params: Vec<Expr> = Vec::new();

                            while let Some(param) = tokens.peek() {
                                match param {
                                    Token::Paren(paren) if paren == &')' => break,
                                    _ => params.push(walk(tokens)),
                                }
                            }

                            let _ = tokens.next();

                            return Expr::CallExpression(String::from(name), params);
                        }
                        _ => panic!("Expected CallExpression after ("),
                    }
                } else {
                    panic!("Expected CallExpression after (");
                }
            }
            _ => unreachable!(),
        };
    } else {
        unreachable!();
    };
}

pub fn ast_from_tokens(tks: Vec<Token>) -> Program {
    let mut tokens = tks.into_iter().peekable();

    let mut body: Vec<Expr> = Vec::new();

    while let Some(_) = tokens.peek() {
        body.push(walk(&mut tokens))
    }

    Program { body: body }
}
