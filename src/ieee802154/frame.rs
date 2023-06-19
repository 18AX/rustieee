use super::address::AddressKind;

const BEACON_VALUE: u8 = 0x0;
const DATA_VALUE: u8 = 0x1;
const ACKNOWLEDGMENT_VALUE: u8 = 0x2;
const MAC_COMMAND_VALUE: u8 = 0x3;
const MULTIPURPOSE_VALUE: u8 = 0x5;
const FRAK_VALUE: u8 = 0x6;
const EXTENDED_VALUE: u8 = 0x7;

/// Different frame type for the first 3 bits of the frame control.
/// Chapter 7.2.2.1
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameKind {
    Beacon,
    Data,
    Acknowledgment,
    MacCommand,
    MultiPurpose,
    Frak,
    Extended,
}

impl FrameKind {
    pub const fn bits(&self) -> u8 {
        match self {
            FrameKind::Beacon => BEACON_VALUE,
            FrameKind::Data => DATA_VALUE,
            FrameKind::Acknowledgment => ACKNOWLEDGMENT_VALUE,
            FrameKind::MacCommand => MAC_COMMAND_VALUE,
            FrameKind::MultiPurpose => MULTIPURPOSE_VALUE,
            FrameKind::Frak => FRAK_VALUE,
            FrameKind::Extended => EXTENDED_VALUE,
        }
    }

    pub const fn from_byte(value: u8) -> Result<Self, crate::parser::Error> {
        match value {
            BEACON_VALUE => Ok(FrameKind::Beacon),
            DATA_VALUE => Ok(FrameKind::Data),
            ACKNOWLEDGMENT_VALUE => Ok(FrameKind::Acknowledgment),
            MAC_COMMAND_VALUE => Ok(FrameKind::MacCommand),
            MULTIPURPOSE_VALUE => Ok(FrameKind::MultiPurpose),
            FRAK_VALUE => Ok(FrameKind::Frak),
            EXTENDED_VALUE => Ok(FrameKind::Extended),
            _ => Err(crate::parser::Error::InvalidHeader),
        }
    }
}

impl From<FrameKind> for u8 {
    fn from(value: FrameKind) -> Self {
        value.bits()
    }
}

impl TryFrom<u8> for FrameKind {
    type Error = crate::parser::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        FrameKind::from_byte(value)
    }
}

/// General frame
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct Frame<'a> {
    pub header: MacHeader,
    pub payload: &'a [u8],
}

/// MAC Header (MHR) of a frame
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct MacHeader {
    pub control_field: ControlField,
    pub src_addr: Option<AddressKind>,
    pub dst_addr: Option<AddressKind>,
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct StandardControlField {
    frame_kind: FrameKind,
}

/// Control field. Common for all frame type except multipurpose frame,
/// fragment frame and extended frame.
/// Chapter 7.2.2
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub enum ControlField {
    Standard(StandardControlField),
}

#[cfg(test)]
mod tests {
    use crate::ieee802154::frame::*;

    #[test]
    fn frame_kind_from_byte_valid_values() {
        assert_eq!(
            FrameKind::Beacon,
            FrameKind::from_byte(BEACON_VALUE).unwrap()
        );
        assert_eq!(FrameKind::Data, FrameKind::from_byte(DATA_VALUE).unwrap());
        assert_eq!(
            FrameKind::Acknowledgment,
            FrameKind::from_byte(ACKNOWLEDGMENT_VALUE).unwrap()
        );
        assert_eq!(
            FrameKind::MacCommand,
            FrameKind::from_byte(MAC_COMMAND_VALUE).unwrap()
        );
        assert_eq!(
            FrameKind::MultiPurpose,
            FrameKind::from_byte(MULTIPURPOSE_VALUE).unwrap()
        );
        assert_eq!(FrameKind::Frak, FrameKind::from_byte(FRAK_VALUE).unwrap());
        assert_eq!(
            FrameKind::Extended,
            FrameKind::from_byte(EXTENDED_VALUE).unwrap()
        );
    }

    #[test]
    fn frame_kind_from_byte_invalid_values() {
        assert!(matches!(
            FrameKind::from_byte(0xFF),
            Err(crate::parser::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameKind::from_byte(0x8),
            Err(crate::parser::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameKind::from_byte(0xA),
            Err(crate::parser::Error::InvalidHeader)
        ));
    }
}
