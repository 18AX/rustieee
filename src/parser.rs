use thiserror_no_std::Error;

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Error, Debug)]
pub enum Error {}

pub trait Parser<'a> {
    type Value;

    fn parse(slice: &'a [u8]) -> Result<Self::Value, Error>;
}
