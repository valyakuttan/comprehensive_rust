use std::iter::Peekable;
use std::str::Chars;
/// # 29.6 Exercise: Rewriting with Result
///
/// The following implements a very simple parser for an expression language. However,
/// it handles errors by panicking. Rewrite it to instead use idiomatic error handling
/// and propagate errors to a return from `main`. Feel free to use `thiserror` and `anyhow`.
///
/// HINT: start by fixing error handling in the `parse` function. Once that is working
/// correctly, update `Tokenizer` to implement `Iterator<Item=Result<Token, TokenizerError>>
/// and handle that in the parser.
///
/// ```
///
/// use std::iter::Peekable;
/// use std::str::Chars;
///
/// /// An arithmetic operator.
/// #[derive(Debug, PartialEq, Clone, Copy)]
/// enum Op {
///     Add,
///     Sub,
/// }
///
/// /// A token in the expression language.
/// #[derive(Debug, PartialEq)]
/// enum Token {
///     Number(String),
///     Identifier(String),
///     Operator(Op),
/// }
///
/// /// An expression in the expression language.
/// #[derive(Debug, PartialEq)]
/// enum Expression {
///     /// A reference to a variable.
///     Var(String),
///     /// A literal number.
///     Number(u32),
///     /// A binary operation.
///     Operation(Box<Expression>, Op, Box<Expression>),
/// }
///
/// fn tokenize(input: &str) -> Tokenizer {
///     return Tokenizer(input.chars().peekable());
/// }
///
/// struct Tokenizer<'a>(Peekable<Chars<'a>>);
///
/// impl<'a> Iterator for Tokenizer<'a> {
///     type Item = Token;
///
///     fn next(&mut self) -> Option<Token> {
///         let Some(c) = self.0.next() else {
///             return None;
///         };
///         match c {
///             '0'..='9' => {
///                 let mut num = String::from(c);
///                 while let Some(c @ '0'..='9') = self.0.peek() {
///                     num.push(*c);
///                     self.0.next();
///                 }
///                 Some(Token::Number(num))
///             }
///             'a'..='z' => {
///                 let mut ident = String::from(c);
///                 while let Some(c @ 'a'..='z' | c @ '_' | c @ '0'..='9') = self.0.peek() {
///                     ident.push(*c);
///                     self.0.next();
///                 }
///                 Some(Token::Identifier(ident))
///             }
///             '+' => Some(Token::Operator(Op::Add)),
///             '-' => Some(Token::Operator(Op::Sub)),
///             _ => panic!("Unexpected character {c}"),
///         }
///     }
/// }
///
/// fn parse(input: &str) -> Expression {
///     let mut tokens = tokenize(input);
///
///     fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Expression {
///         let Some(tok) = tokens.next() else {
///             panic!("Unexpected end of input");
///         };
///         let expr = match tok {
///             Token::Number(num) => {
///                 let v = num.parse().expect("Invalid 32-bit integer'");
///                 Expression::Number(v)
///             }
///             Token::Identifier(ident) => Expression::Var(ident),
///             Token::Operator(_) => panic!("Unexpected token {tok:?}"),
///         };
///         // Look ahead to parse a binary operation if present.
///         match tokens.next() {
///             None => expr,
///             Some(Token::Operator(op)) => {
///                 Expression::Operation(Box::new(expr), op, Box::new(parse_expr(tokens)))
///             }
///             Some(tok) => panic!("Unexpected token {tok:?}"),
///         }
///     }
///
///     parse_expr(&mut tokens)
/// }
///
/// fn main_function() {
///     let expr = parse("10+foo+20-30");
///     println!("{expr:?}");
/// }
///
/// ```
///
use thiserror::Error;

/// An arithmetic operator.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// A token in the expression language.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// An expression in the expression language.
#[derive(Debug, PartialEq)]
enum Expression {
    /// A reference to a variable.
    Var(String),
    /// A literal number.
    Number(u32),
    /// A binary operation.
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

#[derive(Debug, Error)]
enum TokenizerError {
    #[error("Unexpected character '{0}' in input")]
    UnexpectedCharacter(char),
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Result<Token, TokenizerError>> {
        let Some(c) = self.0.next() else {
            return None;
        };
        match c {
            '0'..='9' => {
                let mut num = String::from(c);
                while let Some(c @ '0'..='9') = self.0.peek() {
                    num.push(*c);
                    self.0.next();
                }
                Some(Ok(Token::Number(num)))
            }
            'a'..='z' => {
                let mut ident = String::from(c);
                while let Some(c @ 'a'..='z' | c @ '_' | c @ '0'..='9') = self.0.peek() {
                    ident.push(*c);
                    self.0.next();
                }
                Some(Ok(Token::Identifier(ident)))
            }
            '+' => Some(Ok(Token::Operator(Op::Add))),
            '-' => Some(Ok(Token::Operator(Op::Sub))),
            _ => Some(Err(TokenizerError::UnexpectedCharacter(c))),
        }
    }
}

#[derive(Debug, Error)]
enum ParserError {
    #[error("Tokenizer error: {0}")]
    TokenizerError(#[from] TokenizerError),
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token),
    #[error("Invalid number")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

fn parse(input: &str) -> Result<Expression, ParserError> {
    let mut tokens = tokenize(input);

    fn parse_expr(tokens: &mut Tokenizer) -> Result<Expression, ParserError> {
        let Some(tok) = tokens.next().transpose()? else {
            return Err(ParserError::UnexpectedEOF);
        };
        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse()?;
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => return Err(ParserError::UnexpectedToken(tok)),
        };
        // Look ahead to parse a binary operation if present.
        Ok(match tokens.next() {
            None => expr,
            Some(Ok(Token::Operator(op))) => {
                Expression::Operation(Box::new(expr), op, Box::new(parse_expr(tokens)?))
            }
            Some(Err(e)) => return Err(e.into()),
            Some(Ok(tok)) => return Err(ParserError::UnexpectedToken(tok)),
        })
    }

    parse_expr(&mut tokens)
}

pub fn main() -> anyhow::Result<()> {
    let expr = parse("10+foo+20-30")?;
    println!("{expr:?}");
    Ok(())
}
