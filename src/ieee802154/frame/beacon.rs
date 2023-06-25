use crate::ieee802154::{
    address::AddressKind, control_field::StandardControlField,
    security_header::AuxiliarySecurityHeader,
};

use self::gts::Gts;

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
    pub gts: Gts,
    // TODO: add pending address
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

pub mod gts {

    #[cfg(feature = "ufmt")]
    use ufmt::uwrite;

    use crate::ieee802154::address::{PanId, ShortAddress};

    pub mod offset {
        pub const GTS_DESCRIPTOR_COUNT: usize = 0;
        pub const GTS_PERMIT: usize = 0x7;
        pub const GTS_STARTING_SLOT: usize = 0;
        pub const GTS_DESC_LENGTH: usize = 3;
    }

    pub mod mask {
        use super::offset;

        pub const GTS_DESCRIPTOR_COUNT: u8 = 0x7 << offset::GTS_DESCRIPTOR_COUNT;
        pub const GTS_PERMIT: u8 = 0x1 << offset::GTS_PERMIT;
        pub const GTS_DIRECTION: u8 = 0x7F;
        pub const GTS_STARTING_SLOT: u8 = 0xF << offset::GTS_STARTING_SLOT;
        pub const GTS_DESC_LENGTH: u8 = 0xF << offset::GTS_DESC_LENGTH;
    }

    pub const MAX_GTS_DESCRIPTOR: usize = 0x7;
    /// GTS descriptor size in bytes
    pub(crate) const GTS_DESCRIPTOR_SIZE: usize = 0x3;

    #[derive(Debug, Clone)]
    pub struct Gts {
        pub permit: bool,
        /// Coordinator accepting GTS request
        pub descriptors: heapless::Vec<GtsDescriptor, MAX_GTS_DESCRIPTOR>,
    }

    #[cfg(feature = "ufmt")]
    impl ufmt::uDebug for Gts {
        fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
        where
            W: ufmt::uWrite + ?Sized,
        {
            uwrite!(f, "Gts {{ permit: {} descriptors: [ ", self.permit)?;

            for desc in &self.descriptors {
                uwrite!(f, "{:?} ", desc)?;
            }

            uwrite!(f, "] }}")
        }
    }

    impl Gts {
        pub fn from_bytes(pan: PanId, data: &[u8]) -> Result<Self, crate::parser::Error> {
            if data.is_empty() {
                return Err(crate::parser::Error::InvalidPayload);
            }

            let gts_spec: u8 = data[0];

            let desciptor_count: usize =
                ((gts_spec & mask::GTS_DESCRIPTOR_COUNT) >> offset::GTS_DESCRIPTOR_COUNT).into();
            let permit: bool = gts_spec & mask::GTS_PERMIT != 0;
            let mut descriptors = heapless::Vec::new();

            if desciptor_count != 0 {
                // GTS Spec + GTS direction + GTS list
                if desciptor_count * GTS_DESCRIPTOR_SIZE + 2 < data.len() {
                    return Err(crate::parser::Error::InvalidPayload);
                }

                let gts_direction: u8 = data[1] & mask::GTS_DIRECTION;

                for i in 0..desciptor_count {
                    let index = GTS_DESCRIPTOR_SIZE * i + 2;
                    let gts_desc_info: u8 = data[index + 2];
                    unsafe {
                        descriptors.push_unchecked(GtsDescriptor {
                            address: ShortAddress::new(
                                pan,
                                u16::from_le_bytes([data[index], data[index + 1]]),
                            ),
                            starting_slot: gts_desc_info & mask::GTS_STARTING_SLOT,
                            length: (gts_desc_info & mask::GTS_DESC_LENGTH)
                                >> offset::GTS_DESC_LENGTH,
                            direction: GtsDirection::from_bit((gts_direction & (0x1 << i)) != 0),
                        })
                    }
                }
            }

            Ok(Gts {
                permit,
                descriptors,
            })
        }
    }

    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum GtsDirection {
        Receive,
        Transmit,
    }

    impl From<bool> for GtsDirection {
        fn from(value: bool) -> Self {
            GtsDirection::from_bit(value)
        }
    }

    impl GtsDirection {
        fn from_bit(value: bool) -> Self {
            match value {
                true => GtsDirection::Receive,
                false => GtsDirection::Transmit,
            }
        }
    }

    /// Format of a GTS descriptor.
    /// Figure 7-11
    #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
    #[derive(Debug, Clone)]
    pub struct GtsDescriptor {
        pub address: ShortAddress,
        pub starting_slot: u8,
        pub length: u8,
        pub direction: GtsDirection,
    }

    #[cfg(test)]
    mod tests {
        use crate::{
            address::Address,
            ieee802154::{
                address::{PanId, ShortAddress},
                frame::beacon::gts::GtsDirection,
            },
        };

        use super::Gts;

        #[test]
        fn from_bytes_zero_gts_descriptors() {
            let payload = [0x0];

            let gts = Gts::from_bytes(PanId::broadcast(), &payload).unwrap();

            assert_eq!(gts.permit, false);
            assert_eq!(gts.descriptors.len(), 0);
        }

        #[test]
        fn from_bytes_max_gts_descriptors() {
            let payload: [u8; 23] = [
                0x87, 0xF0, 0xAB, 0xCD, 0xFA, 0xAB, 0xCD, 0xFA, 0xAB, 0xCD, 0xFA, 0xAB, 0xCD, 0xFA,
                0xAB, 0xCD, 0xFA, 0xAB, 0xCD, 0xFA, 0xAB, 0xCD, 0xFA,
            ];

            let gts = Gts::from_bytes(PanId::broadcast(), &payload).unwrap();

            assert_eq!(gts.permit, true);
            assert_eq!(gts.descriptors.len(), 7);

            for i in 0..7 {
                let desc = &gts.descriptors[i];
                assert_eq!(desc.address, ShortAddress::new(PanId::broadcast(), 0xCDAB));

                let direction = match i > 7 / 2 {
                    true => GtsDirection::Receive,
                    false => GtsDirection::Transmit,
                };

                assert_eq!(desc.direction, direction);
                assert_eq!(desc.starting_slot, 0xA);
                assert_eq!(desc.length, 0xF);
            }
        }
    }
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct PendingAddress {}

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
