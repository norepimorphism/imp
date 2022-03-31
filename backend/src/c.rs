mod err;
mod tokens;

pub use err::Error;

use crate::{b::{self, Token}, span::Span};
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
#[derive(Clone, Debug)]
pub struct Expr {
    pub operation: Span<Operation>,
    pub operands: Vec<Span<Operand>>,
}

impl Expr {
    fn parse(tokens: &mut Tokens) -> Result<Span<Self>, Span<Error>> {
        let l_paren = tokens.expect(
            err::Subject::Expr,
            |token| token.inner == Token::LParen,
        )?;

        let operation = Self::parse_operation(tokens)?;

        // TODO
        let operands = vec![Self::parse_operand(tokens)?];

        let r_paren = tokens.expect(
            err::Subject::Token(Some(Token::RParen)),
            |token| token.inner == Token::RParen,
        )?;

        Ok(Span::new(
            Self { operation, operands },
            (l_paren.range.start)..(r_paren.range.end),
        ))
    }

    fn parse_operation(tokens: &mut Tokens) -> Result<Span<Operation>, Span<Error>> {
        let Some(Span {
            inner: Token::Symbol(name),
            range,
        }) = tokens.next() else {
            return Err(tokens.fail(Error::expected(err::Subject::Operation)));
        };

        Ok(Span::new(Operation { name: name.to_string() }, range))
    }
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub name: String,
}

impl Expr {
    fn parse_operand(tokens: &mut Tokens) -> Result<Span<Operand>, Span<Error>> {
        let determinant = tokens
            .peek()
            .ok_or_else(|| tokens.fail(Error::expected(err::Subject::Operand)))?;

        match determinant.inner {
            Token::LParen => Expr::parse(tokens).map(|expr| expr.map(Operand::Expr)),
            // TODO
            // Token::Rational(it),
            // Token::StrLit(it),
            // Token::StrLit(it),
            _ => Err(tokens.fail(Error::expected(err::Subject::Operand))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operand {
    Expr(Expr),
    Rational(Rational),
    StrLit(StrLit),
    Symbol(Symbol),
}

#[derive(Clone, Debug)]
pub struct Rational {
    pub value: f64,
}

#[derive(Clone, Debug)]
pub struct StrLit {
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct Output {
    pub ast: Vec<Span<Expr>>,
}
