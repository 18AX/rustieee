use crate::parser::Parser;

use super::{frame::Frame, Ieee802154};

impl<'a> Parser<'a> for Ieee802154 {
    type Value = Frame<'a>;

    fn parse(_slice: &'a [u8]) -> Result<Self::Value, crate::parser::Error> {
        todo!()
    }
}
