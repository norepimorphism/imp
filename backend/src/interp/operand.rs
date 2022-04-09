// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::parser::{self, Rational, StrLit, Symbol};
use std::{fmt, mem::ManuallyDrop};

pub enum Operand {
    Rational(Rational),
    StrLit(StrLit),
    Symbol(Symbol),
}

impl From<parser::Operand> for Operand {
    fn from(it: parser::Operand) -> Self {
        match it {
            parser::Operand::Expr(it) => super::eval_expr(it).unwrap(),
            parser::Operand::Rational(it) => Self::Rational(it),
            parser::Operand::StrLit(it) => Self::StrLit(it),
            parser::Operand::Symbol(it) => Self::Symbol(it),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rational(it) => it.fmt(f),
            Self::StrLit(it) => it.fmt(f),
            Self::Symbol(it) => it.fmt(f),
        }
    }
}

impl Operand {
    pub fn is_type_valid(&self, expected: Kind) -> bool {
        expected == Kind::new(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    Rational,
    StrLit,
    Symbol,
}

impl Kind {
    fn new(operand: &Operand) -> Self {
        match operand {
            Operand::Rational(_) => Self::Rational,
            Operand::StrLit(_) => Self::StrLit,
            Operand::Symbol(_) => Self::Symbol,
        }
    }
}

pub union Raw {
    pub rational: ManuallyDrop<Rational>,
    pub str_lit: ManuallyDrop<StrLit>,
    pub symbol: ManuallyDrop<Symbol>,
}

impl Raw {
    pub fn new(operand: Operand) -> Self {
        match operand {
            Operand::Rational(it) => Self {
                rational: ManuallyDrop::new(it),
            },
            Operand::StrLit(it) => Self {
                str_lit: ManuallyDrop::new(it),
            },
            Operand::Symbol(it) => Self {
                symbol: ManuallyDrop::new(it),
            },
        }
    }
}