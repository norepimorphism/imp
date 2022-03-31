//! The IMPL parser.
//!
//! The parser recieves tokens from the lexer as input, and produces an Abstract Syntax Tree (AST)
//! as output.

pub mod error;

pub use error::Error;

use crate::{error::Span, lexer::Token};
use std::{fmt, iter::Peekable};

/// Parses a tokenized input to create an AST.
pub fn parse(tokens: Peekable<impl Iterator<Item = Span<Token>>>) -> Result<Expr, Span<Error>> {
    Expr::parse(&mut tokens, true)
}

/// An S-expression.
#[derive(Clone, Debug)]
pub struct Expr {
    /// The unique identifier of the [operation](crate::interp::Operation) to be executed.
    pub operation_id: Span<OperationId>,
    /// The operands to the operation.
    pub operands: Vec<Span<Operand>>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {})",
            self.operation_id.inner,
            self.operands
                .iter()
                .map(|it| it.inner.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Expr {
    /// Parses a tokenized input to create an expression.
    fn parse(tokens: &mut Tokens, is_root: bool) -> Result<Self, Span<Error>> {
        let has_parens = Self::parse_opening_paren(tokens, is_root)?;
        let operation_id = Self::parse_with_parser(tokens, error::Subject::OperationId, &OPERATION_ID_PARSER)?;
        let operands = Self::parse_operands_and_closing_paren(tokens, has_parens)?;

        Ok(Expr {
            operation_id,
            operands,
        })
    }

    fn parse_opening_paren(tokens: &mut Peekable<impl Iterator<Item = Span<Token>>>, is_root: bool) -> Result<bool, Span<Error>> {
        if is_root {
            // Root expressions *do not* require surrounding parentheses. This next token is either
            // a left parenthesis or the operation ID.
            let next_token = tokens.try_peek(error::Subject::Expr)?;
            let has_parens = next_token.inner == Token::LParen;

            if has_parens {
                // This expression is surrounded by parentheses. Advance the iterator to skip the
                // left parenthesis.
                let _ = tokens.next();
            }

            Ok(has_parens)
        } else {
            // Non-root expressions *do* require surrounding parntheses.
            Self::parse_with_parser(tokens, error::Subject::Token(Some(Token::LParen)), &L_PAREN_PARSER)?;

            Ok(true)
        }
    }

    fn parse_with_parser<T>(tokens: &Peekable<impl Iterator<Item = Span<Token>>>, subject: error::Subject, parser: &Parser<T>) -> Result<Span<T>, Span<Error>> {
        let next = tokens.peek().ok_or_else(|| Error::expected(subject))?;
        let result = (parser.parse)(&next.inner);

        result
            .ok_or_else(|| Span::new(Error::invalid(subject), span_range))
            .map(|product| Span::new(product, span_range))
    }

    fn parse_operands_and_closing_paren(
        tokens: &mut Tokens,
        has_parens: bool,
    ) -> Result<Vec<Span<Operand>>, Span<Error>> {
        let mut operands = Vec::new();

        // Parse operands until all tokens within the expression have been processed.
        loop {
            // TODO: Clean this up.
            match tokens.peek() {
                Some(token) => {
                    if has_parens && (token.inner == Token::RParen) {
                        // The next token is a right parenthesis, which terminates the expression.

                        // Skip the right parenthesis.
                        let _ = tokens.next();

                        break;
                    }
                }
                None => {
                    if has_parens {
                        // The expression is terminated by a right parenthesis, but none was found.
                        return Err(
                            Span::new(
                                Error::expected(error::Subject::Token(Some(Token::RParen))),
                                tokens.next_span_range(),
                            ));
                    }

                    break;
                }
            }

            // TODO: [`Operand::parse`] *must* advance the iterator, or else this loop will execute
            // indefinitely.
            operands.push(Operand::parse(tokens)?);
        }

        Ok(operands)
    }
}

struct OperationId {
    name: String,
}

impl fmt::Display for OperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct Rational {
    value: f64,
}

struct StrLit {
    content: String,
}

struct Symbol {
    name: String,
}
