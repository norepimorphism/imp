use crate::lexer::Token;
use std::fmt;
use tokens::Tokens;

pub fn parse(tokens: impl Iterator<Item = Token>) -> Result<Ast, Error> {
    Ast::parse(Tokens::new(tokens))
}

#[derive(Debug, Default)]
pub struct Ast(Vec<Expr>);

impl Ast {
    fn parse<I: Iterator<Item = Token>>(mut tokens: Tokens<I>) -> Result<Self, Error> {
        let mut exprs = Vec::new();
        while !tokens.is_empty() {
            exprs.push(Expr::parse(&mut tokens)?);
        }

        Ok(Ast(exprs))
    }
}

#[derive(Debug)]
pub struct Expr {
    operation: Operation,
    operands: Vec<Operand>,
}

impl Expr {
    pub fn parse<I: Iterator<Item = Token>>(tokens: &mut Tokens<I>) -> Result<Self, Error> {
        tokens.expect(&Token::LeftParen)?;

        let operation = Operation::parse(tokens)?;
        let mut operands = Vec::new();

        while tokens.expect(&Token::RightParen).is_err() {
            operands.push(Operand::parse(tokens)?);
        }

        Ok(Self { operation, operands })
    }
}

#[derive(Debug)]
pub struct Operation(String);

impl Operation {
    fn parse<I: Iterator<Item = Token>>(tokens: &mut Tokens<I>) -> Result<Self, Error> {
        let token = tokens
            .next()
            .ok_or_else(|| Error::ExpectedOperation)?;

        println!("token: {:?}", token);

        match token {
            Token::Plus => Ok("add".to_string()),
            Token::Minus => Ok("sub".to_string()),
            Token::Star => Ok("mul".to_string()),
            Token::Slash => Ok("div".to_string()),
            Token::Word(name) => Ok(name.clone()),
            _ => Err(Error::InvalidOperation),
        }
        .map(Self)
    }
}

#[derive(Debug)]
pub enum Operand {
    Expr(Expr),
    Number(Number),
    StringLit(String),
    Var(String),
}

impl Operand {
    fn parse<I: Iterator<Item = Token>>(tokens: &mut Tokens<I>) -> Result<Self, Error> {
        [
            EXPR_PARSER,
            NUMBER_PARSER,
            STRING_LIT_PARSER,
            VAR_PARSER,
        ]
        .into_iter()
        .find_map()
    }
}

struct OperandParser {
    range: fn(impl Iterator<Item = Token>) -> Option<Range>,
    parse: fn(Vec<Token>) -> Option<Operand>,
}

#[derive(Debug)]
pub enum Number {
    Int(i32),
    Float(f32),
}

impl Number {
    fn parse<I: Iterator<Item = Token>>(tokens: &mut Tokens<I>) -> Result<Self, Error> {
        Ok(Number) // TODo
    }
}

pub struct StringLit;

pub struct Var;

#[derive(Debug)]
pub enum Error {
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
