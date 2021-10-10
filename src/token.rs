#[derive(Eq, PartialEq, Debug)]
pub enum PlusOrMinus {
    Plus,
    Minus,
}

impl PlusOrMinus {
    pub fn eval(&self, a: i64, b: i64) -> i64 {
        match self {
            PlusOrMinus::Plus => a + b,
            PlusOrMinus::Minus => a - b,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum MulOrDiv {
    Mul,
    Div,
}

impl MulOrDiv {
    pub fn eval(&self, a: i64, b: i64) -> Result<i64, OperatorError> {
        use MulOrDiv::*;
        match (self, a, b) {
            (Mul, a, b) => Ok(a * b),
            (Div, _, 0) => Err(OperatorError::DivisionByZero),
            (Div, a, b) => Ok(a / b),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Number(i64),
    PlusOrMinus(PlusOrMinus),
    MulOrDiv(MulOrDiv),
}

#[derive(Eq, PartialEq, Debug)]
pub enum OperatorError {
    DivisionByZero,
}