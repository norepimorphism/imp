use crate::{
    c::Rational,
    d::{
        operand::{self, Operand},
        operation::Operation,
    },
};

macro_rules! unary {
    ($name:ident, $op:tt) => {
        pub const $name: Operation = Operation {
            sig: &[operand::Kind::Rational],
            exe: |ops| {
                let a = unsafe { &ops[0].rational };

                Ok(Operand::Rational(Rational { val: a.val.$op() }))
            },
        };
    };
}

unary!(SIN, sin);
unary!(COS, cos);
unary!(TAN, tan);
unary!(ARCSIN, asin);
unary!(ARCCOS, acos);
unary!(ARCTAN, atan);
