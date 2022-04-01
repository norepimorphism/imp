pub use crate::c::{self, Output};

use std::fmt;

pub fn process(_: &mut c::Output) {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
