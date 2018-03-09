use std::iter::*;

pub mod sexp;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token {
    Name(String),
    Str(String),
    Number(i64),
    Bool(bool),
    Quote,
    OpenParen,
    CloseParen
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LexError {
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
pub fn lex(input: &String) -> Result<Vec<Token>, LexError> {

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
            '"' => {
                vec.push(read_string(&mut iter)?);
            }
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

fn read_string<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<Token, LexError> {

    let mut s = String::new();

    // We want to skip the quote.
    iter.next();

    while let Some(&c) = iter.peek() {
        match c {
            '"' => break,
            '\\' => {
                iter.next();
                s.push(match iter.peek() {
                    Some(& 'n') => '\n',
                    Some(& 'r') => '\r',
                    Some(& '\\') => '\\',
                    Some(& '"') => '"',
                    Some(&c) => return Err(LexError::UnknownChar(c)),
                    None => return Err(LexError::UnexpectedTermination)
                })
            },
            '\n' | '\r' => return Err(LexError::UnknownChar(c)),
            v => s.push(v)
        }
        iter.next();
    }

    Ok(Token::Str(s))

}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedTermination
}

pub fn parse<T: Iterator<Item = Token>>(iter: &mut Peekable<T>) -> Result<sexp::Sexp, ParseError> {
    match iter.peek() {
        Some(&Token::OpenParen) => {
            let mut subs = Vec::new();
            iter.next();
            while let Some(tok) = iter.peek() {
                match tok {
                    &Token::CloseParen => break,
                    _ => {
                        subs.push(parse(iter)?);
                        iter.next();
                    }
                }
            }
            Ok(sexp::Sexp::List(subs))
        },
        Some(&Token::Quote) => {
            iter.next();
            let sub = parse(iter)?;
            Ok(sexp::Sexp::List(vec![sexp::Sexp::Symbol(String::from("quote")), sub]))
        },
        Some(&Token::Number(n)) => Ok(sexp::Sexp::Integer(n)),
        Some(&Token::Name(ref s)) => Ok(sexp::Sexp::Symbol(s.clone())),
        Some(&Token::Str(ref s)) => Ok(sexp::Sexp::Str(s.clone())),
        Some(&Token::Bool(b)) => Ok(sexp::Sexp::Boolean(b)),
        Some(&Token::CloseParen) => Err(ParseError::UnexpectedToken(Token::CloseParen)),
        None => return Err(ParseError::UnexpectedTermination)
    }
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

    #[test]
    fn test_read_string() {
        let s1 = super::read_string(&mut String::from("\"\"").chars().peekable());
        assert_eq!(s1, Ok(Token::Str(String::from(""))));
        let s2 = super::read_string(&mut String::from("\"foo\"").chars().peekable());
        assert_eq!(s2, Ok(Token::Str(String::from("foo"))));
        let s3 = super::read_string(&mut String::from("\" t \\r e \\n s \\\" t \\\\ s \"").chars().peekable());
        assert_eq!(s3, Ok(Token::Str(String::from(" t \r e \n s \" t \\ s "))));
    }

}
