/// # 12.3 Exercise: Expression Evaluation
///
/// Let’s write a simple recursive evaluator for arithmetic expressions.
///
/// - The `Box` type here is a smart pointer. An expression can be “boxed” with
///   `Box::new`` as seen in the tests. To evaluate a boxed expression, use the
///   deref operator to “unbox” `it: eval(*boxed_expr)`.
///
/// - Some expressions cannot be evaluated and will return an error. The `Res` type
///   represents either a successful value or an error with a message. This is very
///   similar to the standard-library `Result`.
///

/// An operation to perform on two subexpressions.

#[allow(dead_code)]
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A literal value
    Value(i64),
}
use Expression::{Op, Value};

/// The result of evaluating an expression.
#[derive(Debug, PartialEq, Eq)]
enum Res {
    /// Evaluation was successful, with the given result.
    Ok(i64),
    /// Evaluation failed, with the given error message.
    Err(String),
}
// Allow `Ok` and `Err` as shorthands for `Res::Ok` and `Res::Err`.
use Res::{Err, Ok};

fn eval(e: Expression) -> Res {
    match e {
        Op { op, left, right } => match (eval(*left), eval(*right)) {
            (Ok(v1), Ok(v2)) => match op {
                Operation::Add => Ok(v1 + v2),
                Operation::Sub => Ok(v1 - v2),
                Operation::Mul => Ok(v1 * v2),
                Operation::Div => {
                    if v2 != 0 {
                        Ok(v1 + v2)
                    } else {
                        Err("division by zero".to_string())
                    }
                }
            },

            (Err(msg), _) => Err(msg),
            (_, Err(msg)) => Err(msg),
        },

        Value(v) => Ok(v),
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}

#[allow(dead_code)]
pub fn main() {
    let result = eval(Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(20)),
    });

    assert_eq!(result, Ok(30));
}
