use std::iter::*;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Token {
    Name(String),
    Str(String),
    Number(i64),
    Bool(bool),
    Quote,
    OpenParen,
    CloseParen
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum LexError {
    UnknownChar(char),
    UnexpectedTermination
}

#[inline]
fn is_name_char(c: char, start: bool) -> bool {
    // This function isn't that pretty.
    if start {
        match c {
            'a'...'z' | 'A'...'Z' | '*' | '+' | '/' | '=' | '_' | '$' | '%' => true,
            _ => false
        }
    } else {
        match c {
            'a'...'z' | 'A'...'Z' | '*' | '+' | '/' | '=' | '_' | '$' | '%' | '0'...'9' => true,
            _ => false
        }
    }
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
            v if is_name_char(v, true) => {
                vec.push(read_name(&mut iter)?);
            },
            '#' => {
                iter.next();
                match iter.peek().cloned() {
                    Some('t') => vec.push(Token::Bool(true)),
                    Some('f') => vec.push(Token::Bool(false)),
                    Some(c) => return Err(LexError::UnknownChar(c)),
                    _ => return Err(LexError::UnexpectedTermination)
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
            ' ' | '\n' | '\r' => { iter.next(); },
            c @ _ => return Err(LexError::UnknownChar(c))
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

    if let Some(Ok(_)) = iter.peek().map(|c| c.to_string().parse::<i64>()) {

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

fn read_name<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<Token, LexError> {

    let mut name = String::new();

    // Push the first char, we know it's good.
    name.push(*iter.peek().unwrap());
    iter.next();

    // Now we go over the rest of the rest of the input.
    while let Some(&c) = iter.peek() {
        if is_name_char(c, false) {
            name.push(c);
            iter.next();
        } else {
            return Err(LexError::UnknownChar(c));
        }
    }

    Ok(Token::Name(name))

}

#[cfg(test)]
pub mod tests {

    use super::Token;

    #[test]
    fn test_read_numbers() {
        let n1 = super::read_number(&mut String::from("12345").chars().peekable());
        assert_eq!(n1, Ok(Token::Number(12345)));
        let n2 = super::read_number(&mut String::from("-1337").chars().peekable());
        assert_eq!(n2, Ok(Token::Number(-1337)));
    }

    #[test]
    fn test_read_name() {
        let n1 = super::read_name(&mut String::from("hello").chars().peekable());
        assert_eq!(n1, Ok(Token::Name(String::from("hello"))));
        let n2 = super::read_name(&mut String::from("a").chars().peekable());
        assert_eq!(n2, Ok(Token::Name(String::from("a"))));
    }

}
