#![allow(unused)]

use crate::macro_rules::*;
use crate::parser::*;
use anyhow::Result;

pub struct Evaluator<'a> {
    parser: Parser<'a>,
}

impl<'a> Evaluator<'a> {
    pub fn new_evaluator(inp_expr: &'a str) -> Self {
        Evaluator {
            parser: Parser::new_parser(inp_expr),
        }
    }

    fn eval_ast(ast: Box<AST>) -> f64 {
        // The ast is a number
        if let Some(f) = ast.num {
            return f;
        }
        let left_operand = Self::eval_ast(ast.left_operand.unwrap());
        let right_operand = Self::eval_ast(ast.right_operand.unwrap());
        match ast.operator.unwrap() {
            Operator::Add => left_operand + right_operand,
            Operator::Deduct => left_operand - right_operand,
            Operator::Multiply => left_operand * right_operand,
            Operator::Divide => left_operand / right_operand,
            Operator::Power => left_operand.powf(right_operand),
        }
    }

    pub fn eval(&mut self) -> Result<f64> {
        Ok(Self::eval_ast(Box::new(self.parser.parse()?)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn eval_ast_test() {
        let ast_1 = num_ast!(1.0);
        let ast_2 = oper_ast!(Operator::Add, num_ast!(2.9), num_ast!(3.1));
        let ast_3 = oper_ast!(Operator::Power, num_ast!(2.0), num_ast!(3.0));

        assert_eq!(1.0, Evaluator::eval_ast(Box::new(ast_1)));
        assert_eq!(6.0, Evaluator::eval_ast(Box::new(ast_2)));
        assert_eq!(8.0, Evaluator::eval_ast(Box::new(ast_3)));
    }
}
