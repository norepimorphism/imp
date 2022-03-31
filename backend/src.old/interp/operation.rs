use crate::parser::Operand;

pub struct Operation {
    pub operand_cnt: usize,
    pub execute: fn(&[Operand]) -> Operand,
}
