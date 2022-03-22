mod input;

use crate::lexer::Token;
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

        fn make_leaves(expr: &Expr) -> Tree<&str> {
            Tree::new(
                expr.operation.name.as_str(),
                expr
                    .operands
                    .iter()
                    .map(|operand| match operand {
                        Operand::Expr(it) => make_leaves(it),
                        Operand::Var(it) => Tree::new(it.as_str(), vec![]),
                    })
                    .collect(),
            )
        }

        let mut tree = Tree::new("v", Vec::new());
        for expr in self.exprs.iter() {
            tree.push(make_leaves(expr));
        }

        tree.fmt(f)
    }
}

impl Ast {
    /// Parses a tokenized input to create an [`Ast`].
    fn parse<I: Iterator<Item = Token>>(mut input: input::Stream<I>) -> Result<Self, Error> {
        // These are the top-level expressions.
        let mut exprs = Vec::new();

        // Parse expressions until all tokens have been processed.
        while !input.is_empty() {
            exprs.push(Expr::parse(&mut input, true)?);
        }

        Ok(Self { exprs })
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
        let start = input
            .peek()
            .ok_or_else(|| Error::ExpectedExpr)?;
        let has_parens = *start == Token::LParen;

        // Only root-level expressions may omit parentheses; non-root expressions must be surrounded
        // by them.
        if !is_root && !has_parens {
            return Err(Error::ExpectedToken(Token::LParen));
        }

        if has_parens {
            // Advance the iterator to skip the left parenthesis.
            let _ = input.next();

            // The next token is now the operation.
        }

        let raw_operation = input
            .next()
            .ok_or_else(|| Error::ExpectedOperation)?;
        let operation = Operation::parse(&raw_operation)?;

        let mut operands = Vec::new();
        while let Some(next) = input.peek() {
            if has_parens && (*next == Token::RParen) {
                break;
            }

            let operand = match next {
                Token::LParen => Expr::parse(input, false).map(Operand::Expr),
                Token::Word(name) => Ok(Operand::Var(name.clone())),
                _ => Err(Error::InvalidOperand)
            }?;

            operands.push(operand);
        }

        if has_parens {
            input.expect(&Token::RParen)?;
        }

        Ok(Self { operation, operands })
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
            Token::Word(name) => Ok(name.clone()),
            _ => Err(Error::InvalidOperation),
        }?;

        Ok(Self { name })
    }
}

#[derive(Debug)]
pub enum Operand {
    Expr(Expr),
    Var(String),
}

impl Operand {
    fn parse<I: Iterator<Item = Token>>(input: &mut input::Stream<I>) -> Result<Self, Error> {
        let first = input
            .peek()
            .ok_or_else(|| Error::ExpectedOperand)?;

        match first {
            Token::LParen => Expr::parse(input, false).map(Self::Expr),
            Token::Word(name) => {
                let _ = input.next();

                Ok(Self::Var(name.clone()))
            }
            _ => Err(Error::InvalidOperand)
        }

    }
}

#[derive(Debug)]
pub enum Error {
    ExpectedExpr,
    ExpectedOperand,
    ExpectedOperation,
    ExpectedToken(Token),
    InvalidOperand,
    InvalidOperation,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
