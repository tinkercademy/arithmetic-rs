#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(String),
    BinOp(char),
    LParen(char),
    RParen(char),
}

struct NumExpr {
    lhs: i32,
    rhs: i32,
    op: char,
}

struct Expr {
    num_expr: NumExpr,
    op: char,
    expr: Box<Expr>
}

fn tokenize<I: Iterator<Item=char>>(inp: &mut I) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut numbuff = String::new();
    for c in inp.by_ref() {
        match c {
            '0'..='9' => numbuff.push(c),
            _ => {
                if !numbuff.is_empty() {
                    tokens.push(Token::Number(numbuff.clone()));
                    numbuff.clear();
                }
                match c {
                    '+' | '-' | '*' | '/' => tokens.push(Token::BinOp(c)),
                    '(' => tokens.push(Token::LParen(c)),
                    ')' => tokens.push(Token::RParen(c)),
                    ' ' => (),
                    _ => panic!("Unexpected character: {}", c),
                }
            },
        }
    }
    tokens
}

/// Attempts to check the parentheses of the input.
/// Returns boolean based on whether the parentheses are balanced or not.
fn check_paren(tokens: &[Token]) -> bool {
    let mut stack = Vec::new();
    for t in tokens {
        match t {
            Token::LParen(_) => stack.push(t),
            Token::RParen(_) => {
                if let Some(Token::LParen(_)) = stack.pop() {
                    continue;
                } else {
                    return false;
                }
            },
            _ => (),
        }
    }
    stack.is_empty()
}

//
fn parse_expr(tokens: &[Token]) -> Expr {
    todo!()
}

fn main() {
    let expr = "((123 + 456)) * (789 / 1204085) + (97937 * 93749735) ";
    let tokens = tokenize(&mut expr.chars());
    println!("{:?}", tokens);
    //let expr = parse_expr_helper(&tokens);
    //println!("{:?}", expr);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_tokenize() {
        let expr = "((123 + 456)) * (789 / 1204085) + (97937 * 93749735) ";
        let tokens = super::tokenize(&mut expr.chars());
        let desired_tokens = vec![
            super::Token::LParen('('),
            super::Token::LParen('('),
            super::Token::Number("123".to_string()),
            super::Token::BinOp('+'),
            super::Token::Number("456".to_string()),
            super::Token::RParen(')'),
            super::Token::RParen(')'),
            super::Token::BinOp('*'),
            super::Token::LParen('('),
            super::Token::Number("789".to_string()),
            super::Token::BinOp('/'),
            super::Token::Number("1204085".to_string()),
            super::Token::RParen(')'),
            super::Token::BinOp('+'),
            super::Token::LParen('('),
            super::Token::Number("97937".to_string()),
            super::Token::BinOp('*'),
            super::Token::Number("93749735".to_string()),
            super::Token::RParen(')'),
        ];
        for (t, d) in tokens.iter().zip(desired_tokens.iter()) {
            assert_eq!(t, d);
        }
    }

    #[test]
    fn test_check_paren() {
        let expr = "((123 + 456)) * (789 / 1204085) + (97937 * 93749735) ";
        let tokens = super::tokenize(&mut expr.chars());
        assert!(super::check_paren(&tokens));

        let failed_expr = "( (1) + 2 ( * 3 )";
        let failed_tokens = super::tokenize(&mut failed_expr.chars());
        assert!(!super::check_paren(&failed_tokens));
    }

}
