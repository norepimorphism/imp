// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! def_operation {
    ($name:ident, [ $($in_ty:tt),* ], [ $($in_id:tt),* ], $out_op_ty:tt, $out:expr $(,)?) => {
        pub const $name: $crate::interp::operation::Operation = $crate::interp::operation::Operation {
            sig: &[ $($crate::interp::operand::Kind::$in_ty),* ],

            exe: |
                #[allow(unused_variables)]
                ops
            | {
                $(
                    // TODO: Don't hardcode `.rational`.
                    let $in_id = unsafe { &ops[0].rational };
                    // HACK: LOL.
                    #[allow(unused_variables)]
                    let ops = &ops[1..];
                )*

                Ok($crate::interp::operation::Operand::$out_op_ty($out))
            },
        };
    };
}

/// Defines a nullary operation that returns a rational.
macro_rules! nullary_r {
    ($name:ident, $op:expr) => {
        def_operation!(
            $name,
            [],
            [],
            Rational,
            $crate::parser::Rational { val: $op },
        );
    };
}

/// Defines a unary operation that accepts and returns a rational.
macro_rules! unary_r_r {
    ($name:ident, $op:tt) => {
        def_operation!(
            $name,
            [Rational],
            [a],
            Rational,
            $crate::parser::Rational { val: a.val.$op() },
        );
    };
}

/// Defines a binary operation that acceptes two rationals and returns a rational.
macro_rules! binary_rr_r {
    ($name:ident, $op:tt) => {
        def_operation!(
            $name,
            [Rational, Rational],
            [a, b],
            Rational,
            $crate::parser::Rational { val: a.val $op b.val },
        );
    };
}
