use crate::{
    error::{self, Error},
    lexer::Token,
    op::{Expr, Operand, OperationId, StrLit, Symbol},
    span::{self, Span},
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
                expr.operation_id.name.clone(),
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
            "AST".to_string(),
            self.exprs.iter().map(make_leaves).collect()
        )
        .fmt(f)
    }
}

impl Ast {
    /// Parses a tokenized input to create an AST.
    pub fn parse(input: impl Iterator<Item = Span<Token>>) -> Result<Self, Error> {
        // Tokenized input.
        let mut input = span::Iter::new(input.collect());
        // Top-level expressions.
        let mut exprs = Vec::new();

        // Parse expressions until all tokens have been processed.
        while !input.is_empty() {
            exprs.push(Expr::parse(&mut input, true)?);
        }

        Ok(Self { exprs })
    }
}

impl Expr {
    /// Parses a tokenized input to create an expression.
    fn parse(
        input: &mut span::Iter<Token>,
        is_root: bool,
    ) -> Result<Self, Error> {
        // This is either the left parenthesis or the operation, depending on whether or not
        // parentheses are used.
        let start = input.peek_or(error::Class::Expr)?;
        let has_parens = start.inner == Token::LParen;

        // Only root-level expressions may omit parentheses; non-root expressions must be surrounded
        // by them.
        if !is_root && !has_parens {
            return Err(Error::expected(
                error::Class::Token(Some(Token::LParen)),
                start.range,
            ));
        }

        if has_parens {
            // Advance the iterator to skip the left parenthesis.
            let _ = input.next();

            // The next token is now the operation.
        }

        let operation_id = input
            .next_or(error::Class::OperationId)
            .and_then(OperationId::parse)?;

        let mut operands = Vec::new();

        // Parse operands until all tokens within the expression have been processed.
        loop {
            // TODO: Clean this up.
            match input.peek() {
                Some(it) => {
                    if has_parens && (it.inner == Token::RParen) {
                        break;
                    }
                }
                None => {
                    break;
                }
            }

            // TODO: [`Operand::parse`] *must* advance the iterator, or else this loop will execute
            // indefinitely.
            operands.push(Operand::parse(input)?);
        }

        if has_parens {
            input.expect_or(&Token::RParen, error::Class::Token(Some(Token::RParen)))?;
        }

        Ok(Self {
            operation_id,
            operands,
        })
    }
}

impl OperationId {
    fn parse(input: Span<Token>) -> Result<Self, Error> {
        let name = match input.inner {
            Token::Plus => Ok("add".to_string()),
            Token::Minus => Ok("sub".to_string()),
            Token::Star => Ok("mul".to_string()),
            Token::Slash => Ok("div".to_string()),
            Token::Dollar => todo!(),
            Token::Symbol(name) => Ok(name),
            _ => Err(Error {
                kind: error::Kind::Invalid,
                class: error::Class::OperationId,
                range: input.range,
            }),
        }?;

        Ok(Self { name })
    }
}

impl Operand {
    fn parse(input: &mut span::Iter<Token>) -> Result<Self, Error> {
        let determinant = input.peek_or(error::Class::Operand)?;

        match determinant.inner {
            Token::LParen => Expr::parse(input, false).map(Operand::Expr),
            Token::StrLit(_) => StrLit::parse(input).map(Operand::StrLit),
            Token::Symbol(_) => Symbol::parse(input).map(Operand::Symbol),
            _ => Err(Error::expected(error::Class::Operand, determinant.range)),
        }
    }
}

impl StrLit {
    fn parse(input: &mut span::Iter<Token>) -> Result<Self, Error> {
        let lit = input.next_or(error::Class::StrLit)?;

        if let Token::StrLit(content) = lit.inner {
            Ok(Self { content })
        } else {
            Err(Error::invalid(error::Class::StrLit, lit.range))
        }
    }
}

impl Symbol {
    fn parse(input: &mut span::Iter<Token>) -> Result<Self, Error> {
        let symbol = input.next_or(error::Class::Symbol)?;

        if let Token::Symbol(name) = symbol.inner {
            Ok(Self { name })
        } else {
            Err(Error::invalid(error::Class::Symbol, symbol.range))
        }
    }
}
