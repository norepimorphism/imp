use std::fmt;

pub fn process() -> () {

}

#[derive(Default)]
pub struct Interp;

pub enum Output {
    Text,
    Graphic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
