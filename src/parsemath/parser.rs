use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }
}

impl<'a> Parser<'a> {
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let added_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = added_expr;
        }
        Ok(left_expr)
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren()?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter valid operator {:?}",
                self.current_token
            ))),
        }
    }

    fn check_paren(&mut self) -> Result<(), ParseError> {
        if self.current_token == Token::RightParen {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                Token::RightParen,
                self.current_token
            )))
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsemath::ast::Node::*;

    #[test]
    fn test_addition() {
        let mut parser = Parser::new("1+2").unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_subtraction() {
        let mut parser = Parser::new("1-2").unwrap();
        let expected = Subtract(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_multiplication() {
        let mut parser = Parser::new("1*2").unwrap();
        let expected = Multiply(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_division() {
        let mut parser = Parser::new("1/2").unwrap();
        let expected = Divide(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_power() {
        let mut parser = Parser::new("1^2").unwrap();
        let expected = Caret(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_negative() {
        let mut parser = Parser::new("-1").unwrap();
        let expected = Negative(Box::new(Number(1.0)));
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_several_additions() {
        let mut parser = Parser::new("1+2+3").unwrap();
        let expected = Add(
            Box::new(Add(Box::new(Number(1.0)), Box::new(Number(2.0)))),
            Box::new(Number(3.0)),
        );
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_addition_and_multiplication() {
        let mut parser = Parser::new("1+2*3").unwrap();
        let expected = Add(
            Box::new(Number(1.0)),
            Box::new(Multiply(Box::new(Number(2.0)), Box::new(Number(3.0)))),
        );
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_parentheses() {
        let mut parser = Parser::new("1*(2+3)").unwrap();
        let expected = Multiply(
            Box::new(Number(1.0)),
            Box::new(Add(Box::new(Number(2.0)), Box::new(Number(3.0)))),
        );
        assert_eq!(parser.parse().unwrap(), expected);

        let mut parser2 = Parser::new("(1+2)*3").unwrap();
        let expected2 = Multiply(
            Box::new(Add(Box::new(Number(1.0)), Box::new(Number(2.0)))),
            Box::new(Number(3.0)),
        );
        assert_eq!(parser2.parse().unwrap(), expected2);
    }

    #[test]
    fn test_power_with_negative() {
        let mut parser = Parser::new("-1^2").unwrap();
        let expected = Caret(
            Box::new(Negative(Box::new(Number(1.0)))),
            Box::new(Number(2.0)),
        );
        assert_eq!(parser.parse().unwrap(), expected)
    }

    #[test]
    fn test_multiplication_of_parentheses() {
        let mut parser = Parser::new("(1+2)(3+4)").unwrap();
        let expected = Multiply(
            Box::new(Add(Box::new(Number(1.0)), Box::new(Number(2.0)))),
            Box::new(Add(Box::new(Number(3.0)), Box::new(Number(4.0)))),
        );
        assert_eq!(parser.parse().unwrap(), expected)
    }
}
