use crate::ieee802154::{
    address::AddressKind, control_field::StandardControlField,
    security_header::AuxiliarySecurityHeader,
};

mod offset {
    pub(crate) const BEACON_ORDER: usize = 0;
    pub(crate) const SUPER_FRAME_ORDER: usize = 4;
    pub(crate) const FINAL_CAP_SLOT: usize = 8;
    pub(crate) const BLE: usize = 12;
    pub(crate) const PAN_COORDINATOR: usize = 14;
    pub(crate) const ASSOCIATION_PERMIT: usize = 15;
}

mod mask {
    use super::offset;

    pub(crate) const BEACON_ORDER: u16 = 0xF;
    pub(crate) const SUPER_FRAME_ORDER: u16 = 0xF << offset::SUPER_FRAME_ORDER;
    pub(crate) const FINAL_CAP_SLOT: u16 = 0xF << offset::FINAL_CAP_SLOT;
    pub(crate) const BLE: u16 = 0x1 << offset::BLE;
    pub(crate) const PAN_COORDINATOR: u16 = 0x1 << offset::PAN_COORDINATOR;
    pub(crate) const ASSOCIATION_PERMIT: u16 = 0x1 << offset::ASSOCIATION_PERMIT;
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct BeaconFrame<'a> {
    pub header: BeaconHeader,
    pub payload: BeaconPayload<'a>,
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct BeaconHeader {
    pub control: StandardControlField,
    pub seq_no: u8,
    pub src_addr: Option<AddressKind>,
    pub dst_addr: Option<AddressKind>,
    pub aux: AuxiliarySecurityHeader,
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct BeaconPayload<'a> {
    pub super_frame: SuperFrame,
    // TODO: add GTS and Pending address
    pub data: &'a [u8],
}

/// Size in byte of a super frame
pub const SUPER_FRAME_SIZE: usize = 2;

/// Super frame structure
/// Chapter 7.3.1.4
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct SuperFrame {
    pub beacon_order: u8,
    pub super_frame_order: u8,
    pub final_cap_slot: u8,
    pub low_energy: bool,
    pub pan_coordinator: bool,
    pub association_permit: bool,
}

impl SuperFrame {
    pub const fn from_bytes(data: &[u8; SUPER_FRAME_SIZE]) -> Self {
        let data: u16 = u16::from_le_bytes(*data);

        SuperFrame {
            beacon_order: (data & mask::BEACON_ORDER >> offset::BEACON_ORDER) as u8,
            super_frame_order: ((data & mask::SUPER_FRAME_ORDER) >> offset::SUPER_FRAME_ORDER)
                as u8,
            final_cap_slot: ((data & mask::FINAL_CAP_SLOT) >> offset::FINAL_CAP_SLOT) as u8,
            low_energy: ((data & mask::BLE) >> offset::BLE) == 0x1,
            pan_coordinator: ((data & mask::PAN_COORDINATOR) >> offset::PAN_COORDINATOR) == 0x1,
            association_permit: ((data & mask::ASSOCIATION_PERMIT) >> offset::ASSOCIATION_PERMIT)
                == 0x1,
        }
    }

    pub const fn bytes(&self) -> [u8; 2] {
        ((((self.beacon_order as u16) << offset::BEACON_ORDER) & mask::BEACON_ORDER)
            | (((self.super_frame_order as u16) << offset::SUPER_FRAME_ORDER)
                & mask::SUPER_FRAME_ORDER)
            | (((self.final_cap_slot as u16) << offset::FINAL_CAP_SLOT) & mask::FINAL_CAP_SLOT)
            | (((self.low_energy as u16) << offset::BLE) & mask::BLE)
            | (((self.pan_coordinator as u16) << offset::PAN_COORDINATOR) & mask::PAN_COORDINATOR)
            | (((self.association_permit as u16) << offset::ASSOCIATION_PERMIT)
                & mask::ASSOCIATION_PERMIT))
            .to_le_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::SuperFrame;

    #[test]
    fn from_bits_parse_correct_values() {
        let bytes: [u8; 2] = [0b10000111, 0b10010010];

        let super_frame = SuperFrame::from_bytes(&bytes);

        assert_eq!(super_frame.beacon_order, 0x7);
        assert_eq!(super_frame.super_frame_order, 0x8);
        assert_eq!(super_frame.final_cap_slot, 0x2);
        assert_eq!(super_frame.low_energy, true);
        assert_eq!(super_frame.pan_coordinator, false);
        assert_eq!(super_frame.association_permit, true);
    }

    #[test]
    fn from_bits_eq_bits() {
        let bytes: [u8; 2] = [0b10100101, 0b01010010];

        assert_eq!(SuperFrame::from_bytes(&bytes).bytes(), bytes);
    }
}
