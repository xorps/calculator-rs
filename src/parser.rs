use crate::nel::Nel;
use crate::token::{MulOrDiv, PlusOrMinus, Token};

#[derive(Eq, PartialEq, Debug)]
enum State {
    Nil(Vec<Token>),
    Integer(Vec<Token>, Nel<char>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum ParseError {
    ReadNumberError(String),
    UnexpectedChar(char),
}

fn read_number(nel: Nel<char>) -> Result<i64, ParseError> {
    let Nel {head, tail} = nel;
    let s: String = std::iter::once(head).chain(tail.into_iter()).collect();
    s.parse().map_err(|_| ParseError::ReadNumberError(s))
}

fn push<T>(mut vec: Vec<T>, item: T) -> Vec<T> {
    vec.push(item);
    vec
}

fn step(state: State, c: char) -> Result<State, ParseError> {
    Ok(match state {
        State::Nil(tokens) => match c {
            c if c.is_ascii_digit() => State::Integer(tokens, Nel::new(c)),
            c if c.is_whitespace() => State::Nil(tokens),
            '+' => State::Nil(push(tokens, Token::PlusOrMinus(PlusOrMinus::Plus))),
            '-' => State::Nil(push(tokens, Token::PlusOrMinus(PlusOrMinus::Minus))),
            '*' => State::Nil(push(tokens, Token::MulOrDiv(MulOrDiv::Mul))),
            '/' => State::Nil(push(tokens, Token::MulOrDiv(MulOrDiv::Div))),
            c => return Err(ParseError::UnexpectedChar(c)),
        },
        State::Integer(tokens, nel) => match c {
            c if c.is_ascii_digit() => State::Integer(tokens, nel.push(c)),
            c if c.is_whitespace() => State::Nil(push(tokens, Token::Number(read_number(nel)?))),
            '+' => State::Nil(push(push(tokens, Token::Number(read_number(nel)?)), Token::PlusOrMinus(PlusOrMinus::Plus))),
            '-' => State::Nil(push(push(tokens, Token::Number(read_number(nel)?)), Token::PlusOrMinus(PlusOrMinus::Minus))),
            '*' => State::Nil(push(push(tokens, Token::Number(read_number(nel)?)), Token::MulOrDiv(MulOrDiv::Mul))),
            '/' => State::Nil(push(push(tokens, Token::Number(read_number(nel)?)), Token::MulOrDiv(MulOrDiv::Div))),
            c => return Err(ParseError::UnexpectedChar(c)),
        }
    })
}

pub fn parse(input: &str) -> Result<Vec<Token>, ParseError> {
    let state = input.chars().try_fold(State::Nil(vec![]), step)?;
    match state {
        State::Nil(tokens) => Ok(tokens),
        State::Integer(tokens, nel) => Ok(push(tokens, Token::Number(read_number(nel)?))),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!(parse("33 * 5"), Ok(vec!(Token::Number(33), Token::MulOrDiv(MulOrDiv::Mul), Token::Number(5))));
    }
}