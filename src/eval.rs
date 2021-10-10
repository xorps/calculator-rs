use crate::token::{Token, OperatorError, MulOrDiv, PlusOrMinus};

#[derive(Debug)]
pub enum State {
    Nil,
    _1(i64),
    _2(i64, PlusOrMinus),
    _3(i64, PlusOrMinus, i64),
    _4(i64, PlusOrMinus, i64, MulOrDiv),
    _5(i64, MulOrDiv),
}

#[derive(Debug)]
pub enum EvalError {
    OperatorError(OperatorError),
    UnexpectedStateWithToken(State, Token),
    FinalizeUnexpectedState(State),
}

impl From<OperatorError> for EvalError {
    fn from(err: OperatorError) -> Self {
        Self::OperatorError(err)
    }
}

fn step(state: State, token: Token) -> Result<State, EvalError> {
    use State::*;
    use Token::*;
    Ok(match (state, token) {
        (Nil,                Number(n)) =>            _1(n),
        (_1(n),              PlusOrMinus(op)) =>      _2(n, op),
        (_1(n),              MulOrDiv(op)) =>         _5(n, op),
        (_2(a, op),          Number(b)) =>            _3(a, op, b),
        (_3(a, op, b),       PlusOrMinus(next_op)) => _2(op.eval(a, b), next_op),
        (_3(a, op, b),       MulOrDiv(next_op)) =>    _4(a, op, b, next_op),
        (_4(a, op, b, op_2), Number(c)) =>            _1(op.eval(a, op_2.eval(b, c)?)),
        (_5(a, op),          Number(b)) =>            _1(op.eval(a, b)?),
        (state, token) => return Err(EvalError::UnexpectedStateWithToken(state, token)),
    })
}

pub fn eval(mut tokens: impl Iterator<Item=Token>) -> Result<i64, EvalError> {
    let state = tokens.try_fold(State::Nil, step)?;
    match state {
        State::_1(n) => Ok(n),
        State::_3(a, op, b) => Ok(op.eval(a, b)),
        state => Err(EvalError::FinalizeUnexpectedState(state)),
    }
}