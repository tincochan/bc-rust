#![allow(unused)]

use crate::macro_rules::*;
use crate::tokenizer::*;
use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Deduct,
    Multiply,
    Divide,
    Power,
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        match self {
            &Operator::Add | &Operator::Deduct => match other {
                &Operator::Add | &Operator::Deduct => true,
                _ => false,
            },
            &Operator::Multiply | &Operator::Divide => match other {
                &Operator::Multiply | &Operator::Divide => true,
                _ => false,
            },
            &Operator::Power => {
                if let &Operator::Power = other {
                    return true;
                }
                false
            }
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            &Operator::Add | &Operator::Deduct => {
                if *other == Operator::Add || *other == Operator::Deduct {
                    return Some(Ordering::Equal);
                }
                Some(Ordering::Less)
            }
            &Operator::Multiply | &Operator::Divide => {
                if *other == Operator::Add || *other == Operator::Deduct {
                    return Some(Ordering::Greater);
                }
                if *other == Operator::Power {
                    return Some(Ordering::Less);
                }
                Some(Ordering::Equal)
            }
            &Operator::Power => {
                if *other == Operator::Power {
                    return Some(Ordering::Equal);
                }
                Some(Ordering::Greater)
            }
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct AST {
    pub operator: Option<Operator>,
    pub left_operand: Option<Box<AST>>,
    pub right_operand: Option<Box<AST>>,
    pub num: Option<f64>,
}

impl PartialEq for AST {
    fn eq(&self, other: &Self) -> bool {
        return self.operator == other.operator
            && self.left_operand == other.left_operand
            && self.right_operand == other.right_operand
            && self.num == other.num;
    }
}

pub struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new_parser(inp_expr: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new_tokenizer(inp_expr).peekable(),
        }
    }

    fn parse_unit(
        &mut self,
        curr_ast: &mut Option<AST>,
        unit: AST,
    ) -> Result<()> {
        let next_token: &Token;
        match self.tokenizer.peek().unwrap() {
            tk @ (Token::Add
            | Token::Deduct
            | Token::Multiply
            | Token::Divide
            | Token::Power) => next_token = tk,
            Token::EOF => {
                curr_ast.as_mut().unwrap().right_operand = some_box!(unit);
                return Ok(());
            }

            Token::RightParen => {
                curr_ast.as_mut().unwrap().right_operand = some_box!(unit);
                return Ok(());
            }

            // invalid format
            invalid_tk @ _ => {
                return Err(anyhow!(
                    "a number cannot followed by the Token::{}",
                    invalid_tk
                ));
            }
        }

        let next_opr = next_token.token_to_operator()?;
        let prev_opr = curr_ast.as_ref().unwrap().operator.as_ref().unwrap();
        if *prev_opr < next_opr {
            let mut tmp_ast = Some(unit);
            self.gen_ast(&mut tmp_ast, false)?;
            curr_ast.as_mut().unwrap().right_operand =
                some_box!(tmp_ast.unwrap());
            return Ok(());
        }

        curr_ast.as_mut().unwrap().right_operand = some_box!(unit);
        Ok(())
    }

    fn gen_ast(
        &mut self,
        curr_ast: &mut Option<AST>,
        in_parentheses: bool,
    ) -> Result<()> {
        while let Some(tt) = self.tokenizer.next() {
            match tt {
                Token::Add => {
                    if let None = curr_ast {
                        return Err(anyhow!("TODO 1"));
                    }
                    *curr_ast = init_opt_oper_ast!(
                        Operator::Add,
                        curr_ast.as_ref().unwrap().clone()
                    );
                }

                Token::Deduct => {
                    if let None = curr_ast {
                        return Err(anyhow!("TODO 2"));
                    }
                    *curr_ast = init_opt_oper_ast!(
                        Operator::Deduct,
                        curr_ast.as_ref().unwrap().clone()
                    );
                }

                Token::Multiply => {
                    if let None = curr_ast {
                        return Err(anyhow!("TODO 3"));
                    }
                    *curr_ast = init_opt_oper_ast!(
                        Operator::Multiply,
                        curr_ast.as_ref().unwrap().clone()
                    );
                }

                Token::Divide => {
                    if let None = curr_ast {
                        return Err(anyhow!("TODO 4"));
                    }
                    *curr_ast = init_opt_oper_ast!(
                        Operator::Divide,
                        curr_ast.as_ref().unwrap().clone()
                    );
                }

                Token::Power => {
                    if let None = curr_ast {
                        return Err(anyhow!("TODO 5"));
                    }
                    *curr_ast = init_opt_oper_ast!(
                        Operator::Power,
                        curr_ast.as_ref().unwrap().clone()
                    );
                }

                Token::LeftParen => {
                    let mut tmp_ast = None;
                    self.gen_ast(&mut tmp_ast, true)?;
                    if let None = curr_ast {
                        *curr_ast = tmp_ast;
                        continue;
                    }
                    self.parse_unit(curr_ast, tmp_ast.unwrap())?;
                }

                Token::RightParen => {
                    if !in_parentheses {
                        return Err(anyhow!(
                            "met right parenthese without left parenthese"
                        ));
                    }
                    return Ok(());
                }

                Token::Num(f) => {
                    // the first number
                    if let None = curr_ast {
                        *curr_ast = some_num_ast!(f);
                        continue;
                    }
                    self.parse_unit(curr_ast, num_ast!(f))?;
                }

                Token::EOF => {
                    if in_parentheses {
                        return Err(anyhow!("met EOF in the parentheses"));
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<AST> {
        let mut ast = None;
        self.gen_ast(&mut ast, false).and_then(|_| Ok(ast.unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::{Operator, Parser, AST};
    #[test]
    fn operator_order_test() {
        assert_eq!(true, Operator::Add == Operator::Deduct);
        assert_eq!(true, Operator::Add < Operator::Multiply);
        assert_eq!(true, Operator::Deduct < Operator::Divide);
        assert_eq!(true, Operator::Power > Operator::Multiply);
    }
    #[test]
    fn parse_test() {
        let mut p_1 = Parser::new_parser("2");
        assert_eq!(num_ast!(2.0), p_1.parse().unwrap());

        let mut p_2 = Parser::new_parser("1 + 2");
        assert_eq!(
            oper_ast!(Operator::Add, num_ast!(1.0), num_ast!(2.0)),
            p_2.parse().unwrap()
        );

        let mut p_3 = Parser::new_parser("1 + 2 + 3");
        assert_eq!(
            oper_ast!(
                Operator::Add,
                oper_ast!(Operator::Add, num_ast!(1.0), num_ast!(2.0)),
                num_ast!(3.0)
            ),
            p_3.parse().unwrap()
        );

        let mut p_4 = Parser::new_parser("1 + 2 * 3");
        assert_eq!(
            oper_ast!(
                Operator::Add,
                num_ast!(1.0),
                oper_ast!(Operator::Multiply, num_ast!(2.0), num_ast!(3.0))
            ),
            p_4.parse().unwrap()
        );

        let mut p_5 = Parser::new_parser("1 * (2 + 3)");
        assert_eq!(
            oper_ast!(
                Operator::Multiply,
                num_ast!(1.0),
                oper_ast!(Operator::Add, num_ast!(2.0), num_ast!(3.0))
            ),
            p_5.parse().unwrap()
        );

        let mut p_6 = Parser::new_parser("(1 + 2) * 3");
        assert_eq!(
            oper_ast!(
                Operator::Multiply,
                oper_ast!(Operator::Add, num_ast!(1.0), num_ast!(2.0)),
                num_ast!(3.0)
            ),
            p_6.parse().unwrap()
        );

        let mut p_7 = Parser::new_parser("3 ^ (1 * (2 + 3))");
        assert_eq!(
            oper_ast!(
                Operator::Power,
                num_ast!(3.0),
                oper_ast!(
                    Operator::Multiply,
                    num_ast!(1.0),
                    oper_ast!(Operator::Add, num_ast!(2.0), num_ast!(3.0))
                )
            ),
            p_7.parse().unwrap()
        );
    }
}
