#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub struct StandardControlField {
    frame_kind: FrameKind,
    security_enabled: bool,
    frame_pending: bool,
    ack_required: bool,
    pan_id_compression: bool,
    seq_no_present: bool,
    ie_present: bool,
    version: FrameVersion,
}

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

/// Frame version. field not present for fragment frame and extended frame.
/// Chapter 7.2.2.10
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameVersion {
    Ieee802154_2003,
    Ieee802154_2006,
    Ieee802154,
}

const IEEE_2003_STD_VALUE: u8 = 0x0;
const IEEE_2006_STD_VALUE: u8 = 0x1;
const IEEE_STD_VALUE: u8 = 0x02;
const IEEE_MULTIPURPOSE_VALUE: u8 = 0x0;

impl FrameVersion {
    const fn from_byte_std(value: u8) -> Result<Self, crate::parser::Error> {
        match value {
            IEEE_2003_STD_VALUE => Ok(FrameVersion::Ieee802154_2003),
            IEEE_2006_STD_VALUE => Ok(FrameVersion::Ieee802154_2006),
            IEEE_STD_VALUE => Ok(FrameVersion::Ieee802154),
            _ => Err(crate::parser::Error::InvalidHeader),
        }
    }

    const fn from_byte_multipurpose(value: u8) -> Result<Self, crate::parser::Error> {
        match value {
            IEEE_MULTIPURPOSE_VALUE => Ok(FrameVersion::Ieee802154),
            _ => Err(crate::parser::Error::InvalidHeader),
        }
    }

    /// Returns [`FrameVersion`] depending on the value.
    ///
    /// The frame kind is needed because the frame version bits change
    /// according to the frame type.
    ///
    /// # Error
    ///
    /// If the value is invalid or the frame type does not have a version
    /// field, returns `InvalidHeader`.
    pub const fn from_byte(kind: FrameKind, value: u8) -> Result<Self, crate::parser::Error> {
        match kind {
            FrameKind::Beacon
            | FrameKind::Data
            | FrameKind::Acknowledgment
            | FrameKind::MacCommand => Self::from_byte_std(value),
            FrameKind::MultiPurpose => Self::from_byte_multipurpose(value),
            _ => Err(crate::parser::Error::InvalidHeader),
        }
    }

    const fn bits_std(&self) -> u8 {
        match self {
            FrameVersion::Ieee802154_2003 => IEEE_2003_STD_VALUE,
            FrameVersion::Ieee802154_2006 => IEEE_2006_STD_VALUE,
            FrameVersion::Ieee802154 => IEEE_STD_VALUE,
        }
    }

    const fn bits_multipurpose(&self) -> Result<u8, crate::composer::Error> {
        match self {
            FrameVersion::Ieee802154 => Ok(IEEE_MULTIPURPOSE_VALUE),
            _ => Err(crate::composer::Error::InvalidHeader),
        }
    }

    /// Returns bits associated with the version.
    ///
    /// # Errors
    ///
    /// If the frame kind does not contain the version provided, an error is
    /// returned.
    pub const fn bits(&self, kind: FrameKind) -> Result<u8, crate::composer::Error> {
        match kind {
            FrameKind::Beacon
            | FrameKind::Data
            | FrameKind::Acknowledgment
            | FrameKind::MacCommand => Ok(self.bits_std()),
            FrameKind::MultiPurpose => self.bits_multipurpose(),
            _ => Err(crate::composer::Error::InvalidHeader),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ieee802154::control_field::*;

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

    #[test]
    fn frame_version_from_byte_valid_values() {
        assert_eq!(
            FrameVersion::Ieee802154_2003,
            FrameVersion::from_byte(FrameKind::Beacon, IEEE_2003_STD_VALUE).unwrap()
        );
        assert_eq!(
            FrameVersion::Ieee802154_2006,
            FrameVersion::from_byte(FrameKind::Data, IEEE_2006_STD_VALUE).unwrap()
        );
        assert_eq!(
            FrameVersion::Ieee802154,
            FrameVersion::from_byte(FrameKind::Acknowledgment, IEEE_STD_VALUE).unwrap()
        );
        assert_eq!(
            FrameVersion::Ieee802154,
            FrameVersion::from_byte(FrameKind::MacCommand, IEEE_STD_VALUE).unwrap()
        );
        assert_eq!(
            FrameVersion::Ieee802154,
            FrameVersion::from_byte(FrameKind::MultiPurpose, IEEE_MULTIPURPOSE_VALUE).unwrap()
        );
    }

    #[test]
    fn frame_version_from_byte_invalid_values() {
        assert!(matches!(
            FrameVersion::from_byte(FrameKind::Frak, IEEE_2003_STD_VALUE),
            Err(crate::parser::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameVersion::from_byte(FrameKind::Extended, IEEE_STD_VALUE),
            Err(crate::parser::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameVersion::from_byte(FrameKind::MultiPurpose, IEEE_STD_VALUE),
            Err(crate::parser::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameVersion::from_byte(FrameKind::MultiPurpose, IEEE_2006_STD_VALUE),
            Err(crate::parser::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameVersion::from_byte(FrameKind::Beacon, 0x4),
            Err(crate::parser::Error::InvalidHeader)
        ));
    }

    #[test]
    fn frame_version_bits_valid_values() {
        assert_eq!(
            FrameVersion::Ieee802154.bits(FrameKind::Beacon).unwrap(),
            IEEE_STD_VALUE
        );
        assert_eq!(
            FrameVersion::Ieee802154_2003
                .bits(FrameKind::Acknowledgment)
                .unwrap(),
            IEEE_2003_STD_VALUE
        );
        assert_eq!(
            FrameVersion::Ieee802154_2006.bits(FrameKind::Data).unwrap(),
            IEEE_2006_STD_VALUE
        );
        assert_eq!(
            FrameVersion::Ieee802154
                .bits(FrameKind::MacCommand)
                .unwrap(),
            IEEE_STD_VALUE
        );
        assert_eq!(
            FrameVersion::Ieee802154
                .bits(FrameKind::MultiPurpose)
                .unwrap(),
            IEEE_MULTIPURPOSE_VALUE
        );
    }

    #[test]
    fn frame_version_bits_invalid_values() {
        assert!(matches!(
            FrameVersion::Ieee802154.bits(FrameKind::Frak),
            Err(crate::composer::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameVersion::Ieee802154.bits(FrameKind::Extended),
            Err(crate::composer::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameVersion::Ieee802154_2003.bits(FrameKind::MultiPurpose),
            Err(crate::composer::Error::InvalidHeader)
        ));
        assert!(matches!(
            FrameVersion::Ieee802154_2006.bits(FrameKind::MultiPurpose),
            Err(crate::composer::Error::InvalidHeader)
        ));
    }
}
