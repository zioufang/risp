#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    let fmt_expr = expr.replace("(", " ( ").replace(")", " ) ");
    let words = fmt_expr.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                if let Ok(i) = word.parse::<i64>() {
                    tokens.push(Token::Integer(i))
                } else {
                    tokens.push(Token::Symbol(word.to_string()))
                }
            }
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let tokens = tokenize("(+ 1 2)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }
}
