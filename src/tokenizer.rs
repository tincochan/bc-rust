#![allow(unused)]

use crate::parser::Operator;
use anyhow::{anyhow, Error, Result};
use std::iter::{Iterator, Peekable};

pub struct Tokenizer<'a> {
    expr: Peekable<std::str::Chars<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Deduct,
    Multiply,
    Divide,
    Num(f64),
    LeftParen,
    RightParen,
    Power,
    EOF,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "Add"),
            Self::Deduct => write!(f, "Deduct"),
            Self::Multiply => write!(f, "Multiply"),
            Self::Divide => write!(f, "Divide"),
            Self::Num(n) => write!(f, "Num({})", n),
            Self::LeftParen => write!(f, "LeftParen"),
            Self::RightParen => write!(f, "RightParen"),
            Self::Power => write!(f, "Power"),
            Self::EOF => write!(f, "EOF"),
        }
    }
}

impl Token {
    pub fn token_to_operator(&self) -> Result<Operator> {
        match self {
            Token::Add => Ok(Operator::Add),
            Token::Deduct => Ok(Operator::Deduct),
            Token::Multiply => Ok(Operator::Multiply),
            Token::Divide => Ok(Operator::Divide),
            Token::Power => Ok(Operator::Power),
            _ => Err(anyhow!("TODO 0 {:?}", self)),
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new_tokenizer(inp_expr: &'a str) -> Self {
        Tokenizer {
            expr: inp_expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_char) = self.expr.peek() {
            if next_char.is_whitespace() {
                self.expr.next();
            }
            break;
        }
        match self.expr.next() {
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Deduct),
            Some('*') | Some('x') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Power),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some(next_char @ '0'..='9') => {
                let mut num_token = String::new();
                num_token.push(next_char);
                while let Some('0'..='9') | Some('.') = self.expr.peek() {
                    num_token.push(self.expr.next().unwrap());
                }
                Some(Token::Num(num_token.parse().unwrap()))
            }
            Some(_) => None,
            None => Some(Token::EOF),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tokenize() {
        let inp_expr = String::from("1 + 2 * 3");
        let mut tokenizer = Tokenizer::new_tokenizer(&inp_expr);
        assert_eq!(tokenizer.next(), Some(Token::Num(1.0)));
        assert_eq!(tokenizer.next(), Some(Token::Add));
        assert_eq!(tokenizer.next(), Some(Token::Num(2.0)));
        assert_eq!(tokenizer.next(), Some(Token::Multiply));
        assert_eq!(tokenizer.next(), Some(Token::Num(3.0)));
    }

    #[test]
    fn tokenize_f64_num() {
        let inp_expr = String::from("1 + 2.67 * 3.1");
        let mut tokenizer = Tokenizer::new_tokenizer(&inp_expr);
        assert_eq!(tokenizer.next(), Some(Token::Num(1.0)));
        assert_eq!(tokenizer.next(), Some(Token::Add));
        assert_eq!(tokenizer.next(), Some(Token::Num(2.67)));
        assert_eq!(tokenizer.next(), Some(Token::Multiply));
        assert_eq!(tokenizer.next(), Some(Token::Num(3.1)));
    }

    #[test]
    fn tokenize_f64_num_err() {
        let inp_expr = String::from("1 + (2.67 * 3.1)");
        let mut tokenizer = Tokenizer::new_tokenizer(&inp_expr);
        assert_eq!(tokenizer.next(), Some(Token::Num(1.0)));
        assert_eq!(tokenizer.next(), Some(Token::Add));
        assert_eq!(tokenizer.next(), Some(Token::LeftParen));
        assert_eq!(tokenizer.next(), Some(Token::Num(2.67)));
        assert_eq!(tokenizer.next(), Some(Token::Multiply));
        assert_eq!(tokenizer.next(), Some(Token::Num(3.1)));
        assert_eq!(tokenizer.next(), Some(Token::RightParen));
    }
}
