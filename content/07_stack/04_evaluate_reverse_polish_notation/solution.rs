// Evaluate Reverse Polish Notation — reference solution.
// Operand stack; an operator pops right then left and pushes the checked
// result, and the first error stops evaluation. Time O(n), space O(n).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpnError {
    InvalidToken(String),
    MissingOperand,
    DivisionByZero,
    Overflow,
    Empty,
    TooManyOperands,
}

pub fn eval_rpn(tokens: &[&str]) -> Result<i32, RpnError> {
    let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());
    for &token in tokens {
        let apply: fn(i32, i32) -> Option<i32> = match token {
            "+" => i32::checked_add,
            "-" => i32::checked_sub,
            "*" => i32::checked_mul,
            "/" => i32::checked_div,
            _ => {
                let value = token
                    .parse()
                    .map_err(|_| RpnError::InvalidToken(token.to_string()))?;
                stack.push(value);
                continue;
            }
        };
        let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) else {
            return Err(RpnError::MissingOperand);
        };
        if token == "/" && rhs == 0 {
            return Err(RpnError::DivisionByZero);
        }
        stack.push(apply(lhs, rhs).ok_or(RpnError::Overflow)?);
    }
    match stack[..] {
        [result] => Ok(result),
        [] => Err(RpnError::Empty),
        _ => Err(RpnError::TooManyOperands),
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
