use crate::c::Rational;
use super::{expect_operand, Error, Operand, Operation};

macro_rules! bi_rat {
    ($name:ident, $op:tt) => {
        pub const $name: Operation = Operation {
            exe: |ops| {
                expect_operand!(Operand::Rational(a), ops, 0);
                expect_operand!(Operand::Rational(b), ops, 1);

                Ok(Operand::Rational(Rational { val: a.val $op b.val }))
            },
        };
    };
}

bi_rat!(ADD, +);
bi_rat!(SUB, -);
bi_rat!(MUL, *);
bi_rat!(DIV, /);
