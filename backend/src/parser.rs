use crate::{
    error::{self, Error},
    lexer::Token,
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
            // TODO: [`Expr::parse`] *must* advance the iterator, or else this loop will execute
            // indefinitely.
            exprs.push(Expr::parse(&mut input, true)?);
        }

        Ok(Self { exprs })
    }
}

/// An expression.
#[derive(Clone, Debug, Default)]
pub struct Expr {
    /// The operation ID.
    pub operation_id: OperationId,
    /// The operands.
    pub operands: Vec<Operand>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {})",
            self.operation_id,
            self.operands
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
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
            return Err(Error::expected(error::Class::Token(Some(Token::LParen)), start.range));
        }

        if has_parens {
            // Advance the iterator to skip the left parenthesis.
            let _ = input.next();

            // The next token is now the operation ID.
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
                        // The next token is a right parenthesis, which terminates the expression.

                        // Skip the right parenthesis.
                        let _ = input.next();

                        break;
                    }
                }
                None => {
                    if has_parens {
                        // The expression is terminated by a right parenthesis, but none was found.
                        return Err(Error::expected(
                            error::Class::Token(Some(Token::RParen)),
                            input.next_range(),
                        ));
                    }

                    break;
                }
            }

            // TODO: [`Operand::parse`] *must* advance the iterator, or else this loop will execute
            // indefinitely.
            operands.push(Operand::parse(input)?);
        }

        Ok(Self {
            operation_id,
            operands,
        })
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct OperationId {
    /// The name.
    pub name: String,
}

impl fmt::Display for OperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl OperationId {
    fn parse(input: Span<Token>) -> Result<Self, Error> {
        let name = match input.inner {
            Token::Plus => Ok("add".to_string()),
            Token::Minus => Ok("sub".to_string()),
            Token::Star => Ok("mul".to_string()),
            Token::Slash => Ok("div".to_string()),
            Token::Symbol(name) => Ok(name),
            _ => Err(Error::invalid(error::Class::OperationId, input.range)),
        }?;

        Ok(Self { name })
    }
}

/// An operand.
#[derive(Clone, Debug)]
pub enum Operand {
    /// An expression.
    Expr(Expr),
    /// A rational number.
    Rational(Rational),
    /// A string literal.
    StrLit(StrLit),
    /// A symbol.
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
    fn parse(input: &mut span::Iter<Token>) -> Result<Self, Error> {
        let determinant = input.peek_or(error::Class::Operand)?;

        // Identify the next operand by its first token.
        match determinant.inner {
            Token::LParen => Expr::parse(input, false).map(Operand::Expr),
            Token::Rational(_) => Rational::parse(input).map(Operand::Rational),
            Token::StrLit(_) => StrLit::parse(input).map(Operand::StrLit),
            Token::Symbol(_) => Symbol::parse(input).map(Operand::Symbol),
            _ => Err(Error::expected(error::Class::Operand, determinant.range)),
        }
    }
}

/// A rational number.
#[derive(Clone, Debug)]
pub struct Rational {
    pub value: f64,
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
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

macro_rules! impl_parse_for_newtype {
    ($ty:tt, $field_name:tt, $field_def:expr) => {
        impl $ty {
            fn parse(input: &mut span::Iter<Token>) -> Result<Self, Error> {
                let input = input.next_or(error::Class::$ty)?;

                if let Token::$ty($field_name) = input.inner {
                    Ok(Self { $field_name: $field_def })
                } else {
                    Err(Error::invalid(error::Class::$ty, input.range))
                }
            }
        }
    };
}

impl_parse_for_newtype!(Rational, value, value.parse().unwrap());
impl_parse_for_newtype!(StrLit, content, content);
impl_parse_for_newtype!(Symbol, name, name);
