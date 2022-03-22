mod input;

use crate::lexer::Token;
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
                        Operand::Number(it) => {
                            let Number::Int(it) = it;
                            Tree::new(it.to_string(), Vec::new())
                        }
                        Operand::Symbol(it) => Tree::new(it.name.clone(), Vec::new()),
                    })
                    .collect(),
            )
        }

        Tree::new(
            "*".to_string(),
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
                Err(Error::ExpectedExpr) => {
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
            Err(Error::ExpectedExpr)
        }
    }
}

/// An expression.
#[derive(Debug)]
pub struct Expr {
    /// The operation.
    pub operation: Operation,
    /// The operands.
    pub operands: Vec<Operand>,
}

impl Expr {
    /// Parses a tokenized input to create an [`Expr`].
    fn parse<I: Iterator<Item = Token>>(
        input: &mut input::Stream<I>,
        is_root: bool,
    ) -> Result<Self, Error> {
        // This is either the left parenthesis or the operation, depending on whether or not
        // parentheses are used.
        let start = input.peek().ok_or_else(|| Error::ExpectedExpr)?;
        let has_parens = *start == Token::LParen;

        // Only root-level expressions may omit parentheses; non-root expressions must be surrounded
        // by them.
        if !is_root && !has_parens {
            return Err(Error::ExpectedToken(Token::LParen));
        }

        if has_parens {
            // Advance the iterator to skip the left parenthesis.
            input.advance();

            // The next token is now the operation.
        }

        let operation = input
            .next()
            .ok_or_else(|| Error::ExpectedOperation)
            .and_then(|ref token| Operation::parse(token))?;

        let mut operands = Vec::new();
        loop {
            match Operand::parse(input) {
                Ok(operand) => {
                    operands.push(operand);
                }
                Err(Error::ExpectedOperand) | Err(Error::InvalidOperand) => {
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

#[derive(Debug)]
pub struct Operation {
    pub name: String,
}

impl Operation {
    fn parse(input: &Token) -> Result<Self, Error> {
        let name = match input {
            Token::Plus => Ok("add".to_string()),
            Token::Minus => Ok("sub".to_string()),
            Token::Star => Ok("mul".to_string()),
            Token::Slash => Ok("div".to_string()),
            Token::Symbol(name) => Ok(name.clone()),
            _ => Err(Error::InvalidOperation),
        }?;

        Ok(Self { name })
    }
}

#[derive(Debug)]
pub enum Operand {
    Expr(Expr),
    Number(Number),
    Symbol(Symbol),
}

impl Operand {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let determinant = input
            .peek()
            .ok_or_else(|| Error::ExpectedOperand)?;

        match determinant {
            Token::LParen => Expr::parse(input, false).map(Operand::Expr),
            Token::Number(_) => Number::parse(input).map(Operand::Number),
            Token::Symbol(_) => Symbol::parse(input).map(Operand::Symbol),
            _ => Err(Error::InvalidOperand),
        }
    }
}

#[derive(Debug)]
pub enum Number {
    Int(u64)
}

impl Number {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let number = input
            .next()
            .ok_or_else(|| Error::ExpectedNumber)?;

        if let Token::Number(value) = number {
            value
                .parse::<u64>()
                .map_err(|_| Error::InvalidNumber)
                .map(Self::Int)
        } else {
            Err(Error::InvalidNumber)
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let symbol = input
            .next()
            .ok_or_else(|| Error::ExpectedSymbol)?;

        if let Token::Symbol(name) = symbol {
            Ok(Self { name })
        } else {
            Err(Error::InvalidSymbol)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ExpectedExpr,
    ExpectedNumber,
    ExpectedOperand,
    ExpectedOperation,
    ExpectedSymbol,
    ExpectedToken(Token),
    InvalidNumber,
    InvalidOperand,
    InvalidOperation,
    InvalidSymbol,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
