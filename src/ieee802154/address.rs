use crate::address::Address;

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressKind {
    Short(ShortAddress),
    Long(LongAddress),
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShortAddress(u8);

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LongAddress(u64);

impl const Address for ShortAddress {
    type Inner = u8;

    fn broadcast() -> Self {
        ShortAddress(0xFF)
    }

    fn value(&self) -> Self::Inner {
        self.0
    }
}

impl const Address for LongAddress {
    type Inner = u64;

    fn broadcast() -> Self {
        LongAddress(0xFFFFFFFF)
    }

    fn value(&self) -> Self::Inner {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        address::Address,
        ieee802154::address::{LongAddress, ShortAddress},
    };

    #[test]
    fn broadcast_for_short_address() {
        assert_eq!(ShortAddress::broadcast().value(), 0xFF)
    }

    #[test]
    fn broadcast_for_long_address() {
        assert_eq!(LongAddress::broadcast().value(), 0xFFFFFFFF)
    }
}
