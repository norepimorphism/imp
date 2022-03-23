mod input;

use crate::{error::{self, Error}, lexer::Token};
use std::fmt;

/// Translates lexical tokens into an [AST](`Ast`).
pub fn parse(input: impl Iterator<Item = Token>) -> Result<Ast, Error> {
    Ast::parse(input::Stream::new(input))
}

/// An abstract syntax tree (AST).
#[derive(Debug, Default)]
pub struct Ast {
    /// Top-level expressions.
    pub exprs: Vec<Expr>,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use termtree::Tree;

        fn make_leaves(expr: &Expr) -> Tree<String> {
            Tree::new(
                expr.operation.name.clone(),
                expr.operands
                    .iter()
                    .map(|operand| match operand {
                        Operand::Expr(it) => make_leaves(it),
                        Operand::Ratio(it) => {
                            Tree::new(it.to_string(), Vec::new())
                        }
                        Operand::StrLit(it) => Tree::new(it.content.clone(), Vec::new()),
                        Operand::Symbol(it) => Tree::new(it.name.clone(), Vec::new()),
                    })
                    .collect(),
            )
        }

        Tree::new(
            "(root)".to_string(),
            self.exprs.iter().map(make_leaves).collect()
        )
        .fmt(f)
    }
}

impl Ast {
    /// Parses a tokenized input to create an [`Ast`].
    fn parse<I: Iterator<Item = Token>>(mut input: input::Stream<I>) -> Result<Self, Error> {
        // These are the top-level expressions.
        let mut exprs = Vec::new();

        // Parse expressions until [`Expr::parse`] fails.
        loop {
            match Expr::parse(&mut input, true) {
                Ok(expr) => {
                    exprs.push(expr);
                }
                Err(Error {
                    kind: error::Kind::Expected,
                    class: error::Class::Expr(_),
                }) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if input.is_empty() {
            // All tokens have been processed.
            Ok(Self { exprs })
        } else {
            Err(Error {
                kind: error::Kind::Expected,
                class: error::Class::Expr(None),
            })
        }
    }
}

/// An expression.
#[derive(Clone, Debug, Default)]
pub struct Expr {
    /// The operation.
    pub operation: Operation,
    /// The operands.
    pub operands: Vec<Operand>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {})",
            self.operation,
            self.operands
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Expr {
    /// Parses a tokenized input to create an [`Expr`].
    fn parse<I: Iterator<Item = Token>>(
        input: &mut input::Stream<I>,
        is_root: bool,
    ) -> Result<Self, Error> {
        // This is either the left parenthesis or the operation, depending on whether or not
        // parentheses are used.
        let start = input.peek().ok_or_else(|| Error {
            kind: error::Kind::Expected,
            class: error::Class::Expr(None),
        })?;
        let has_parens = *start == Token::LParen;

        // Only root-level expressions may omit parentheses; non-root expressions must be surrounded
        // by them.
        if !is_root && !has_parens {
            return Err(Error {
                kind: error::Kind::Expected,
                class: error::Class::Token(Some(Token::LParen)),
            });
        }

        if has_parens {
            // Advance the iterator to skip the left parenthesis.
            input.advance();

            // The next token is now the operation.
        }

        let operation = input
            .next()
            .ok_or_else(|| Error {
                kind: error::Kind::Expected,
                class: error::Class::Operation(None),
            })
            .and_then(Operation::parse)?;

        let mut operands = Vec::new();
        loop {
            match Operand::parse(input) {
                Ok(operand) => {
                    operands.push(operand);
                }
                Err(Error {
                    kind: _,
                    class: error::Class::Operand(_),
                }) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if has_parens {
            input.expect(&Token::RParen)?;
        }

        Ok(Self {
            operation,
            operands,
        })
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Operation {
    pub name: String,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Operation {
    fn parse(input: Token) -> Result<Self, Error> {
        let name = match input {
            Token::Plus => Ok("add".to_string()),
            Token::Minus => Ok("sub".to_string()),
            Token::Star => Ok("mul".to_string()),
            Token::Slash => Ok("div".to_string()),
            Token::Dollar => todo!(),
            Token::Symbol(name) => Ok(name),
            _ => Err(Error {
                kind: error::Kind::Invalid,
                class: error::Class::Operation(Some(Operation { name: input.to_string() })),
            }),
        }?;

        Ok(Self { name })
    }
}

#[derive(Clone, Debug)]
pub enum Operand {
    /// An [Expr](expression).
    Expr(Expr),
    /// A number.
    Number(Number),
    /// A string literal.
    StrLit(StrLit),
    /// A symbol.
    Symbol(Symbol),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr(it) => it.fmt(f),
            Self::Ratio(it) => it.fmt(f),
            Self::StrLit(it) => it.fmt(f),
            Self::Symbol(it) => it.fmt(f),
        }
    }
}

impl Operand {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let determinant = input
            .peek()
            .ok_or_else(|| Error {
                kind: error::Kind::Expected,
                class: error::Class::Operand(None),
            })?;

        match determinant {
            Token::LParen => Expr::parse(input, false).map(Operand::Expr),
            Token::Ratio(_) => Ratio::parse(input).map(Operand::Ratio),
            Token::StrLit(_) => StrLit::parse(input).map(Operand::StrLit),
            Token::Symbol(_) => Symbol::parse(input).map(Operand::Symbol),
            _ => Err(Error {
                kind: error::Kind::Invalid,
                class: error::Class::Operand(None),
            }),
        }
    }
}

pub enum Number {
    Rational(Ratio)
}

/// A rational number.
#[derive(Clone, Debug)]
pub struct Ratio {
    pub value: f64,
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Ratio {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let number = input
            .next()
            .ok_or_else(|| Error {
                kind: error::Kind::Expected,
                class: error::Class::Ratio(None),
            })?;

        if let Token::Ratio(value) = number {
            value
                .parse::<f64>()
                .map_err(|_| Error {
                    kind: error::Kind::Invalid,
                    class: error::Class::Ratio(None),
                })
                .map(|value| Self { value })
        } else {
            Err(Error {
                kind: error::Kind::Invalid,
                class: error::Class::Ratio(None),
            })
        }
    }
}

/// A string literal.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct StrLit {
    pub content: String,
}

impl fmt::Display for StrLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.content)
    }
}

impl StrLit {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let lit = input
            .next()
            .ok_or_else(|| Error {
                kind: error::Kind::Expected,
                class: error::Class::StrLit(None),
            })?;

        if let Token::StrLit(content) = lit {
            Ok(Self { content })
        } else {
            Err(Error {
                kind: error::Kind::Invalid,
                class: error::Class::StrLit(None),
            })
        }
    }
}

/// A symbol.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Symbol {
    pub name: String,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Symbol {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let symbol = input
            .next()
            .ok_or_else(|| Error {
                kind: error::Kind::Expected,
                class: error::Class::Symbol(None),
            })?;

        if let Token::Symbol(name) = symbol {
            Ok(Self { name })
        } else {
            Err(Error {
                kind: error::Kind::Invalid,
                class: error::Class::Symbol(None),
            })
        }
    }
}
