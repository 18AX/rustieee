#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct BeaconFrame<'a> {
    // TODO
    payload: &'a [u8],
}
