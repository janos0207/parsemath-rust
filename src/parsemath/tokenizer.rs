use std::iter::Peekable;
use std::str::Chars;

use super::token::Token;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("42");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(42.0))
    }

    #[test]
    fn test_decimal_integer() {
        let mut tokenizer = Tokenizer::new("42.1");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(42.1))
    }

    #[test]
    fn test_operators() {
        let test_set = vec![
            ("+", Token::Add),
            ("-", Token::Subtract),
            ("*", Token::Multiply),
            ("/", Token::Divide),
            ("^", Token::Caret),
        ];

        for (str, token) in test_set.into_iter() {
            let mut tokenizer = Tokenizer::new(str);
            assert_eq!(tokenizer.next().unwrap(), token)
        }
    }

    #[test]
    fn test_parentheses() {
        let test_set = vec![("(", Token::LeftParen), (")", Token::RightParen)];

        for (str, token) in test_set.into_iter() {
            let mut tokenizer = Tokenizer::new(str);
            assert_eq!(tokenizer.next().unwrap(), token)
        }
    }

    #[test]
    fn test_eof() {
        let mut tokenizer = Tokenizer::new("");
        assert_eq!(tokenizer.next().unwrap(), Token::EOF)
    }

    #[test]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("a");
        assert_eq!(tokenizer.next(), None)
    }
}
