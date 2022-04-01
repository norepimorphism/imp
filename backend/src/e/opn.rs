mod arith;
mod calc;
mod trig;

pub struct Operation {
    exe: fn(&[Operand]) -> Operand,
}
