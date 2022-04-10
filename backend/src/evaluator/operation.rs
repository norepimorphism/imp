// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::operand::{self, Operand};
use ahash::RandomState;
use std::{collections::HashMap, fmt};

pub struct Operation {
    pub sig: &'static [operand::Kind],
    pub exe: fn(&[operand::RawOperand]) -> Result<Operand, Error>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[static_init::dynamic]
pub static OPERATIONS: HashMap<&'static str, Operation, RandomState> = HashMap::from_iter([
    ("e", konst::E),
    ("pi", konst::PI),
    ("add", arith::ADD),
    ("sub", arith::SUB),
    ("mul", arith::MUL),
    ("div", arith::DIV),
    // ("sum", arith::SUM),
    // ("prod", arith::PROD),
    // ("int", calc::INT),
    ("sin", trig::SIN),
    ("cos", trig::COS),
    ("tan", trig::TAN),
    // ("arcsin", trig::ARCSIN),
    // ("arccos", trig::ARCCOS),
    // ("arctan", trig::ARCTAN),
    // ("deg", trig::DEG),
    // ("rad", trig::RAD),
]);

macro_rules! access_operand {
    ($parent:expr, Rational) => {
        $parent.rational
    };
    ($parent:expr, StrLit) => {
        $parent.str_lit
    };
    ($parent:expr, Symbol) => {
        $parent.symbol
    }
}

macro_rules! def_operation {
    ($name:ident, [ $( $in_id:tt : $in_ty:ident ),* ], $out_op_ty:tt, $out:expr $(,)?) => {
        pub const $name: $crate::evaluator::operation::Operation = $crate::evaluator::operation::Operation {
            sig: &[ $($crate::evaluator::operand::Kind::$in_ty),* ],
            exe: |
                #[allow(unused_variables)]
                ops
            | {
                $(
                    let $in_id = unsafe { &*access_operand!(&ops[0], $in_ty) };
                    // HACK: LOL.
                    #[allow(unused_variables)]
                    let ops = &ops[1..];
                )*

                Ok($crate::evaluator::operation::Operand::$out_op_ty(
                    $out( $($in_id),* )
                ))
            },
        };
    };
}

mod konst {
    use rust_decimal::Decimal;

    macro_rules! def_constant {
        ($name:ident, $val:expr) => {
            def_operation!(
                $name,
                [],
                Rational,
                || crate::parser::Rational { val: $val },
            );
        };
    }

    def_constant!(E, Decimal::E);
    def_constant!(PI, Decimal::PI);
}

mod arith {
    use crate::parser::Rational;

    macro_rules! def_infix_binary {
        ($name:ident, $op:tt) => {
            def_operation!(
                $name,
                [a: Rational, b: Rational],
                Rational,
                |a: &Rational, b: &Rational| Rational { val: a.val $op b.val },
            );
        };
    }

    def_infix_binary!(ADD, +);
    def_infix_binary!(SUB, -);
    def_infix_binary!(MUL, *);
    def_infix_binary!(DIV, /);
}

mod calc {

}

mod trig {
    use crate::parser::Rational;
    use rust_decimal::MathematicalOps as _;

    macro_rules! def_trig_fn {
        ($name:ident, $fn:ident) => {
            def_operation!(
                $name,
                [a: Rational],
                Rational,
                |a: &Rational| Rational { val: a.val.$fn() },
            );
        };
    }

    def_trig_fn!(SIN, sin);
    def_trig_fn!(COS, cos);
    def_trig_fn!(TAN, tan);
}
