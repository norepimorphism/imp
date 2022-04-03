use crate::{c::Rational, d::{operand::{self, Operand}, operation::Operation}};

macro_rules! bi_rat {
    ($name:ident, $op:tt) => {
        pub const $name: Operation = Operation {
            sig: &[operand::Kind::Rational, operand::Kind::Rational],
            exe: |ops| {
                let (a, b) = unsafe { (&ops[0].rational, &ops[1].rational) };

                Ok(Operand::Rational(Rational { val: a.val $op b.val }))
            },
        };
    };
}

bi_rat!(ADD, +);
bi_rat!(SUB, -);
bi_rat!(MUL, *);
bi_rat!(DIV, /);
