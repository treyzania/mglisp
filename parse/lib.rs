use std::iter::*;

#[derive(Clone, Debug)]
enum Token {
    Name(String),
    Str(String),
    Number(i64),
    Bool(bool),
    Quote,
    OpenParen,
    CloseParen
}

enum LexError {
    UnknownToken
}

/// Parses a vector of tokens.
///
/// A lot of this is stolen from https://adriann.github.io/rust_parser.html.
fn lex(input: &String) -> Result<Vec<Token>, LexError> {

    let mut vec = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            '-' | '0'...'9' => {
                vec.push(read_number(&mut iter)?);
            },
            // We catch the `-` case above.
            'a'...'z' | 'A'...'Z' | '*' | '+' | '/' | '=' | '_' | '$' | '%' => {
                vec.push(Token::Name(read_name(&mut iter)?));
            },
            '#' => {
                iter.next();
                match iter.peek().cloned() {
                    Some('t') => vec.push(Token::Bool(true)),
                    Some('f') => vec.push(Token::Bool(false)),
                    _ => return Err(LexError::UnknownToken)
                }
            },
            '\'' => {
                iter.next();
                vec.push(Token::Quote);
            },
            '(' => {
                iter.next();
                vec.push(Token::OpenParen);
            },
            ')' => {
                iter.next();
                vec.push(Token::CloseParen);
            }
            ' ' | '\n' => { iter.next(); },
            _ => return Err(LexError::UnknownToken)
        }
    }

    Ok(vec)

}

fn read_number<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<Token, LexError> {

    let sign = if *iter.peek().unwrap() == '-' {
        iter.next();
        -1
    } else {
        1
    };

    if let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i64>()) {

        // This is so hacky but so slick.
        let mut num = 0;
        while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i64>()) {
            num = num * 10 + digit;
            iter.next();
        }

        Ok(Token::Number(num * sign))

    } else {
        Ok(Token::Name(String::from("-")))
    }

}

fn read_name<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<String, LexError> {
    unimplemented!();
}
