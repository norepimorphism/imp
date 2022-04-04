//! The IMPL parser.

mod err;
mod tokens;

pub use err::Error;

use crate::{
    b::{self, Token},
    span::Span,
};
use std::fmt;
use tokens::Tokens;

pub fn process(input: b::Output) -> Result<Output, Span<Error>> {
    let mut tokens = Tokens::new(input.tokens);
    let mut output = Output::default();

    while !tokens.is_empty() {
        output.ast.push(Expr::parse(&mut tokens)?);
    }

    Ok(output)
}

/// An S-expression.
#[derive(Debug)]
pub struct Expr {
    pub operation: Span<Operation>,
    pub operands: Vec<Span<Operand>>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {})",
            self.operation.inner,
            self.operands
                .iter()
                .map(|operand| operand.inner.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Expr {
    fn parse(tokens: &mut Tokens) -> Result<Span<Self>, Span<Error>> {
        let l_paren = tokens.expect(err::Subject::Expr, |token| token.inner == Token::LParen)?;

        let operation = Operation::parse(tokens)?;

        let mut operands = Vec::new();
        while let Some(operand) = Operand::parse(tokens)? {
            operands.push(operand);
        }

        let r_paren = tokens.expect(err::Subject::Token(Some(Token::RParen)), |token| {
            token.inner == Token::RParen
        })?;

        Ok(Span::new(
            Self {
                operation,
                operands,
            },
            (l_paren.range.start)..(r_paren.range.end),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub name: String,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Operation {
    fn parse(tokens: &mut Tokens) -> Result<Span<Self>, Span<Error>> {
        let Some(Span {
            inner: Token::Symbol(name),
            range,
        }) = tokens.next() else {
            return Err(tokens.fail(Error::expected(err::Subject::Operation)));
        };

        Ok(Span::new(
            Self {
                name: name.to_string(),
            },
            range,
        ))
    }
}

#[derive(Debug)]
pub enum Operand {
    Expr(Expr),
    Rational(Rational),
    StrLit(StrLit),
    Symbol(Symbol),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr(it) => it.fmt(f),
            Self::Rational(it) => it.fmt(f),
            Self::StrLit(it) => it.fmt(f),
            Self::Symbol(it) => it.fmt(f),
        }
    }
}

impl Operand {
    fn parse(tokens: &mut Tokens) -> Result<Option<Span<Operand>>, Span<Error>> {
        let determinant = tokens
            .peek()
            .ok_or_else(|| tokens.expected(err::Subject::Operand))?;

        if let Token::LParen = determinant.inner {
            return Expr::parse(tokens)
                .map(|expr| expr.map(Self::Expr))
                .map(Some);
        }

        if let Token::RParen = determinant.inner {
            return Ok(None);
        }

        tokens.advance();

        match determinant.inner {
            Token::Rational(val) => {
                let val = val.parse::<f64>().unwrap();
                Ok(Operand::Rational(Rational { val }))
            }
            Token::StrLit(content) => Ok(Operand::StrLit(StrLit { content })),
            Token::Symbol(name) => Ok(Operand::Symbol(Symbol { name })),
            _ => Err(Error::expected(err::Subject::Operand)),
        }
        .map(|it| Some(Span::new(it, determinant.range.clone())))
        .map_err(|e| Span::new(e, determinant.range))
    }
}

#[derive(Debug)]
pub struct Rational {
    pub val: f64,
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Clone, Debug)]
pub struct StrLit {
    pub content: String,
}

impl fmt::Display for StrLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.content)
    }
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: String,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Default)]
pub struct Output {
    pub ast: Vec<Span<Expr>>,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.ast
                .iter()
                .map(|expr| expr.inner.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
