use super::Operation;
use crate::parser::{Operand, Rational};

pub(super) const ADD_RATIONAL: Operation = Operation {
    operand_cnt: 2,
    execute: |operands| {
        if let Operand::Rational(a) = &operands[0] {
            if let Operand::Rational(b) = &operands[1] {
                Operand::Rational(Rational {
                    value: a.value + b.value,
                })
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    },
};
