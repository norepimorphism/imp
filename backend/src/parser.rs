use crate::{
    error::{self, Error},
    lexer::Token,
    op::{Expr, Operand, Operation, StrLit, Symbol},
    saved::{self, Saved},
};
use std::fmt;

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
                        it @ _ => Tree::new(it.to_string(), Vec::new()),
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
    pub fn parse(mut input: impl Iterator<Item = Saved<Token>>) -> Result<Self, Error> {
        let input = saved::Iter::new(input.collect());
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
                    pos: _,
                }) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if input.peek().is_none() {
            // All tokens have been processed.
            Ok(Self { exprs })
        } else {
            Err(Error::expected(error::Class::Expr, input.next_pos()))
        }
    }
}

impl Expr {
    /// Parses a tokenized input to create an expression.
    fn parse(
        input: &mut saved::Iter<Token>,
        is_root: bool,
    ) -> Result<Self, Error> {
        // This is either the left parenthesis or the operation, depending on whether or not
        // parentheses are used.
        let start = input
            .peek()
            .ok_or_else(|| Error::expected(error::Class::Expr, input.next_pos()))?;
        let has_parens = start.inner == Token::LParen;

        // Only root-level expressions may omit parentheses; non-root expressions must be surrounded
        // by them.
        if !is_root && !has_parens {
            return Err(Error::expected(error::Class::Token, input.next_pos()));
        }

        if has_parens {
            // Advance the iterator to skip the left parenthesis.
            let _ = input.next();

            // The next token is now the operation.
        }

        let operation = input
            .next()
            .ok_or_else(|| Error::expected(error::Class::Operation, input.next_pos()))
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
                    pos: _,
                }) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if has_parens {
            input.expect(&Token::RParen, error::Class::Token)?;
        }

        Ok(Self {
            operation,
            operands,
        })
    }
}

impl Operation {
    fn parse(input: Saved<Token>) -> Result<Self, Error> {
        let name = match input.inner {
            Token::Plus => Ok("add".to_string()),
            Token::Minus => Ok("sub".to_string()),
            Token::Star => Ok("mul".to_string()),
            Token::Slash => Ok("div".to_string()),
            Token::Dollar => todo!(),
            Token::Symbol(name) => Ok(name),
            _ => Err(Error {
                kind: error::Kind::Invalid,
                class: error::Class::Operation,
                pos: input.pos,
            }),
        }?;

        Ok(Self { name })
    }
}

impl Operand {
    fn parse(input: &mut saved::Iter<Token>) -> Result<Self, Error> {
        let determinant = input
            .peek()
            .ok_or_else(|| Error::expected(error::Class::Operand, input.next_pos()))?;

        match determinant.inner {
            Token::LParen => Expr::parse(input, false).map(Operand::Expr),
            Token::StrLit(_) => StrLit::parse(input).map(Operand::StrLit),
            Token::Symbol(_) => Symbol::parse(input).map(Operand::Symbol),
            _ => Err(Error::expected(error::Class::Operand, determinant.pos)),
        }
    }
}

impl StrLit {
    fn parse(input: &mut saved::Iter<Token>) -> Result<Self, Error> {
        let lit = input
            .next()
            .ok_or_else(|| Error::expected(error::Class::StrLit, input.prev_pos()))?;

        if let Token::StrLit(content) = lit.inner {
            Ok(Self { content })
        } else {
            Err(Error::invalid(error::Class::StrLit, lit.pos))
        }
    }
}

impl Symbol {
    fn parse(input: &mut saved::Iter<Token>) -> Result<Self, Error> {
        let symbol = input
            .next()
            .ok_or_else(|| Error::expected(error::Class::Symbol, input.prev_pos()))?;

        if let Token::Symbol(name) = symbol.inner {
            Ok(Self { name })
        } else {
            Err(Error::invalid(error::Class::Symbol, symbol.pos))
        }
    }
}
