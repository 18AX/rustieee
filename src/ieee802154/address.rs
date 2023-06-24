use crate::address::Address;

/// The PAN ID is an ID used to identify a group of devices. An address must be
/// associated with a PAN ID.
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PanId(u16);

impl const Address for PanId {
    type Inner = u16;

    fn broadcast() -> Self {
        PanId(0xFFFF)
    }
    fn value(&self) -> Self::Inner {
        self.0
    }
}

impl PanId {
    pub const fn new(address: u16) -> Self {
        PanId(address)
    }
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressKind {
    Short(ShortAddress),
    Long(LongAddress),
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShortAddress(PanId, u16);

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LongAddress(PanId, u64);

impl const Address for ShortAddress {
    type Inner = u16;

    fn broadcast() -> Self {
        ShortAddress(PanId::broadcast(), 0xFFFF)
    }

    fn value(&self) -> Self::Inner {
        self.1
    }
}

impl ShortAddress {
    pub const fn new(pan: PanId, address: u16) -> Self {
        ShortAddress(pan, address)
    }

    pub const fn pan_id(&self) -> PanId {
        self.0
    }
}

impl const Address for LongAddress {
    type Inner = u64;

    fn broadcast() -> Self {
        LongAddress(PanId::broadcast(), 0xFFFFFFFF)
    }

    fn value(&self) -> Self::Inner {
        self.1
    }
}

impl LongAddress {
    pub const fn new(pan: PanId, address: u64) -> Self {
        LongAddress(pan, address)
    }

    pub const fn pan_id(&self) -> PanId {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        address::Address,
        ieee802154::address::{LongAddress, PanId, ShortAddress},
    };

    #[test]
    fn broadcast_for_short_address() {
        assert_eq!(ShortAddress::broadcast().value(), 0xFFFF);
        assert_eq!(ShortAddress::broadcast().pan_id(), PanId::broadcast());
    }

    #[test]
    fn broadcast_for_long_address() {
        assert_eq!(LongAddress::broadcast().value(), 0xFFFFFFFF);
        assert_eq!(LongAddress::broadcast().pan_id(), PanId::broadcast());
    }
}
