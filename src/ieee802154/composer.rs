use crate::composer::Composer;

use super::{frame::Frame, Ieee802154};

impl<'a> Composer<'a> for Ieee802154 {
    type Value = Frame<'a>;

    fn compose(_value: &Self::Value) -> Result<alloc::vec::Vec<u8>, crate::composer::Error> {
        todo!()
    }
}
