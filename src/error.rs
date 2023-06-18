use thiserror_no_std::Error;

use crate::{composer, parser};

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("composer: {0}")]
    Composer(#[from] composer::Error),

    #[error("parser: {0}")]
    Parser(#[from] parser::Error),
}
