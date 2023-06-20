use alloc::vec::Vec;
use thiserror_no_std::Error;

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid header")]
    InvalidHeader,
}

pub trait Composer<'a> {
    type Value;

    fn compose(value: &'a Self::Value) -> Result<Vec<u8>, Error>;
}
