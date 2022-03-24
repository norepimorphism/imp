mod input;

use crate::{
    error::{self, Error},
    lexer::Token,
    op::{Expr, Operand, Operation, StrLit, Symbol},
};
use std::fmt;

/// Translates lexical tokens into an AST.
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

        // TODO: Clean this up; document.

        fn make_leaves(expr: &Expr) -> Tree<String> {
            Tree::new(
                expr.operation.name.clone(),
                expr.operands
                    .iter()
                    .map(|operand| match operand {
                        Operand::Expr(it) => make_leaves(it),
                        Operand::Number(it) => Tree::new(format!("{}", it), Vec::new()),
                        Operand::StrLit(it) => Tree::new(format!("{}", it), Vec::new()),
                        Operand::Symbol(it) => Tree::new(format!("{}", it), Vec::new()),
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
    /// Parses a tokenized input to create an AST.
    fn parse<I: Iterator<Item = Token>>(mut input: input::Stream<I>) -> Result<Self, Error> {
        // Top-level expressions.
        let mut exprs = Vec::new();

        // Parse expressions until [`Expr::parse`] fails.
        loop {
            match Expr::parse(&mut input, true) {
                Ok(expr) => {
                    exprs.push(expr);
                }
                Err(Error {
                    kind: error::Kind::Expected,
                    class: error::Class::Expr,
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
            Err(Error::expected(error::Class::Expr))
        }
    }
}

impl Expr {
    /// Parses a tokenized input to create an expression.
    fn parse<I: Iterator<Item = Token>>(
        input: &mut input::Stream<I>,
        is_root: bool,
    ) -> Result<Self, Error> {
        // This is either the left parenthesis or the operation, depending on whether or not
        // parentheses are used.
        let start = input
            .peek()
            .ok_or_else(|| Error::expected(error::Class::Expr))?;
        let has_parens = *start == Token::LParen;

        // Only root-level expressions may omit parentheses; non-root expressions must be surrounded
        // by them.
        if !is_root && !has_parens {
            return Err(Error::expected(error::Class::Token));
        }

        if has_parens {
            // Advance the iterator to skip the left parenthesis.
            input.advance();

            // The next token is now the operation.
        }

        let operation = input
            .next()
            .ok_or_else(|| Error::expected(error::Class::Operation))
            .and_then(Operation::parse)?;

        let mut operands = Vec::new();
        loop {
            match Operand::parse(input) {
                Ok(operand) => {
                    operands.push(operand);
                }
                Err(Error {
                    kind: _,
                    class: error::Class::Operand,
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

impl Operation {
    fn parse(input: Token) -> Result<Self, Error> {
        let name = match input {
            Token::Plus => Ok("add".to_string()),
            Token::Minus => Ok("sub".to_string()),
            Token::Star => Ok("mul".to_string()),
            Token::Slash => Ok("div".to_string()),
            Token::Dollar => todo!(),
            Token::Symbol(name) => Ok(name),
            _ => Err(Error::invalid(error::Class::Operation)),
        }?;

        Ok(Self { name })
    }
}

impl Operand {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let determinant = input
            .peek()
            .ok_or_else(|| Error::expected(error::Class::Operand))?;

        match determinant {
            Token::LParen => Expr::parse(input, false).map(Operand::Expr),
            Token::StrLit(_) => StrLit::parse(input).map(Operand::StrLit),
            Token::Symbol(_) => Symbol::parse(input).map(Operand::Symbol),
            _ => Err(Error::expected(error::Class::Operand)),
        }
    }
}

impl StrLit {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let lit = input
            .next()
            .ok_or_else(|| Error::expected(error::Class::StrLit))?;

        if let Token::StrLit(content) = lit {
            Ok(Self { content })
        } else {
            Err(Error::invalid(error::Class::StrLit))
        }
    }
}

impl Symbol {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let symbol = input
            .next()
            .ok_or_else(|| Error::expected(error::Class::Symbol))?;

        if let Token::Symbol(name) = symbol {
            Ok(Self { name })
        } else {
            Err(Error::invalid(error::Class::Symbol))
        }
    }
}
