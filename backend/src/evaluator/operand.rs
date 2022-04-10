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
    pub fn kind(&self) -> Kind {
        match self {
            Self::Rational(_) => Kind::Rational,
            Self::StrLit(_) => Kind::StrLit,
            Self::Symbol(_) => Kind::Symbol,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    Rational,
    StrLit,
    Symbol,
}

impl Operand {
    pub fn raw(self) -> RawOperand {
        match self {
            Self::Rational(it) => RawOperand {
                rational: ManuallyDrop::new(it),
            },
            Self::StrLit(it) => RawOperand {
                str_lit: ManuallyDrop::new(it),
            },
            Self::Symbol(it) => RawOperand {
                symbol: ManuallyDrop::new(it),
            },
        }
    }
}

pub union RawOperand {
    pub rational: ManuallyDrop<Rational>,
    pub str_lit: ManuallyDrop<StrLit>,
    pub symbol: ManuallyDrop<Symbol>,
}
