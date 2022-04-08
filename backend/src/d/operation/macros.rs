// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! def_operation {
    ($name:ident, [ $($in_ty:tt),* ], [ $($in_id:tt),* ], $out_op_ty:tt, $out:expr $(,)?) => {
        pub const $name: $crate::d::operation::Operation = $crate::d::operation::Operation {
            sig: &[ $($crate::d::operand::Kind::$in_ty),* ],
            exe: |ops| {
                $(
                    // HACK: LOL.
                    let $in_id = unsafe { &ops[0].rational };
                    let ops = &ops[1..];
                )*

                Ok($crate::d::operation::Operand::$out_op_ty($out))
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
            $crate::c::Rational { val: $op },
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
            $crate::c::Rational { val: a.val.$op() },
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
            $crate::c::Rational { val: a.val $op b.val },
        );
    };
}
