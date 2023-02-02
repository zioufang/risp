use core::fmt;
use std::error::Error;

use crate::{
    lexer::{tokenize, Token},
    object::LispObj,
};

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Error: {}", self.err)
    }
}

impl Error for ParseError {}

pub fn parse(program: &str) -> Result<LispObj, ParseError> {
    let mut tokens = tokenize(program);
    parse_tokens(&mut tokens)
}

fn parse_tokens(tokens: &mut Vec<Token>) -> Result<LispObj, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }

    let mut obj: Vec<LispObj> = Vec::new();
    while !tokens.is_empty() {
        if let Some(token) = tokens.pop() {
            match token {
                Token::Integer(i) => obj.push(LispObj::Integer(i)),
                Token::Symbol(s) => obj.push(LispObj::Symbol(s)),
                Token::LParen => {
                    tokens.push(Token::LParen);
                    let sub_obj = parse_tokens(tokens)?;
                    obj.push(sub_obj);
                }
                Token::RParen => {
                    return Ok(LispObj::List(obj));
                }
            }
        }
    }
    Err(ParseError {
        err: "Missing RParen".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let list = parse("(+ 1 2)").unwrap();
        assert_eq!(
            list,
            LispObj::List(vec![
                LispObj::Symbol("+".to_string()),
                LispObj::Integer(1),
                LispObj::Integer(2),
            ])
        );
    }
}
