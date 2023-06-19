#![no_std]
#![feature(const_trait_impl)]

extern crate alloc;

use alloc::vec::Vec;
use composer::Composer;
use error::Error;
use parser::Parser;

pub mod address;
pub mod composer;
pub mod error;
pub mod parser;

#[cfg(feature = "ieee802154")]
pub mod ieee802154;

pub fn parse<'a, P: Parser<'a>>(raw: &'a [u8]) -> Result<P::Value, Error> {
    Ok(P::parse(raw)?)
}

pub fn compose<'a, C: Composer<'a>>(input: &'a C::Value) -> Result<Vec<u8>, Error> {
    Ok(C::compose(input)?)
}
