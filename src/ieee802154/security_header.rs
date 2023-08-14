use byte::{BytesExt, TryRead, LE};

// TODO: chapter 9.4
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Default)]
pub struct AuxiliarySecurityHeader {
    pub security_level: Option<SecurityLevel>,
    pub key_identifier_mode: KeyIdentifierMode,
    pub frame_counter: Option<u32>,
}

impl<'a> TryRead<'a> for AuxiliarySecurityHeader {
    fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
        let mut hdr = AuxiliarySecurityHeader::default();
        let offset = &mut 0;

        let security_control: u8 = bytes.read(offset)?;

        let security_level: u8 = security_control & 0x7;
        let key_identifier_mode: u8 = (security_control >> 3) & 0x3;
        let frame_counter_present: bool = security_control & (1 << 5) == 0;

        if security_level != 0 {
            hdr.security_level = Some(SecurityLevel {
                encrypted: security_level & 0x4 != 0,
                mic: match security_level & 0x3 {
                    0x1 => Mic::Mic32,
                    0x2 => Mic::Mic64,
                    0x3 => Mic::Mic128,
                    _ => return Err(byte::Error::BadInput { err: "Invalid MIC" }),
                },
            });
        }

        if frame_counter_present {
            hdr.frame_counter = Some(bytes.read_with(offset, LE)?);
        }

        hdr.key_identifier_mode = match key_identifier_mode {
            0x1 => KeyIdentifierMode::KeyIndex(KeyIndex(bytes.read(offset)?)),
            0x2 => KeyIdentifierMode::Key4(
                ShortKey(bytes.read_with(offset, LE)?),
                KeyIndex(bytes.read(offset)?),
            ),
            0x3 => KeyIdentifierMode::Key8(
                LongKey(bytes.read_with(offset, LE)?),
                KeyIndex(bytes.read(offset)?),
            ),
            _ => KeyIdentifierMode::Implicit,
        };

        Ok((hdr, *offset))
    }
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mic {
    Mic32,
    Mic64,
    Mic128,
}

impl Mic {
    pub fn size(&self) -> usize {
        match self {
            Mic::Mic32 => 4,
            Mic::Mic64 => 8,
            Mic::Mic128 => 16,
        }
    }
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityLevel {
    pub mic: Mic,
    pub encrypted: bool,
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyIndex(u8);

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortKey(u32);

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LongKey(u64);

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum KeyIdentifierMode {
    /// Key is determined implicitly from the originator and recipient(s) of the frame, as indicated in the frame header.
    #[default]
    Implicit,
    /// Key is determined from the Key Index field.
    KeyIndex(KeyIndex),
    /// Key is determined explicitly from the 4-octet Key Source field and the Key Index field.
    Key4(ShortKey, KeyIndex),
    /// Key is determined explicitly from the 8-octet Key Source field and the Key Index field.
    Key8(LongKey, KeyIndex),
}

impl KeyIdentifierMode {
    pub fn key_identifier_length(&self) -> usize {
        match self {
            KeyIdentifierMode::Implicit => 0,
            KeyIdentifierMode::KeyIndex(_) => 1,
            KeyIdentifierMode::Key4(_, _) => 5,
            KeyIdentifierMode::Key8(_, _) => 9,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use byte::BytesExt;

    use crate::ieee802154::security_header::{
        KeyIdentifierMode, KeyIndex, LongKey, Mic, SecurityLevel, ShortKey,
    };

    use super::AuxiliarySecurityHeader;

    #[test]
    fn with_frame_counter() {
        let input: [u8; 5] = [0x00, 0xFE, 0xDC, 0xBA, 0x98];

        let mut offset = 0;
        let hdr: AuxiliarySecurityHeader = input.read(&mut offset).unwrap();

        assert_eq!(hdr.security_level, None);
        assert_eq!(hdr.frame_counter, Some(0x98BADCFE));
        assert_eq!(hdr.key_identifier_mode, KeyIdentifierMode::Implicit);
    }

    #[test]
    fn without_frame_counter() {
        let input: [u8; 1] = [0x1 << 5];

        let mut offset = 0;
        let hdr: AuxiliarySecurityHeader = input.read(&mut offset).unwrap();

        assert_eq!(hdr.frame_counter, None);
        assert_eq!(hdr.key_identifier_mode, KeyIdentifierMode::Implicit);
        assert_eq!(hdr.security_level, None);
    }

    #[test]
    fn with_enc_mic_128_and_key8() {
        let key: u64 = 0xABCD12345678ABCD;
        let key_index: u8 = 0xCD;

        let mut input: Vec<u8> = Vec::new();
        input.push(0x3F);
        input.extend_from_slice(&key.to_le_bytes());
        input.push(key_index);

        let mut offset = 0;
        let hdr: AuxiliarySecurityHeader = input.read(&mut offset).unwrap();

        assert_eq!(hdr.frame_counter, None);
        assert_eq!(
            hdr.security_level,
            Some(SecurityLevel {
                encrypted: true,
                mic: Mic::Mic128,
            })
        );
        assert_eq!(
            hdr.key_identifier_mode,
            KeyIdentifierMode::Key8(LongKey(key), KeyIndex(key_index))
        );
    }

    #[test]
    fn with_frame_counter_and_no_enc_mic_64_and_key4() {
        let key: u32 = 0xDEADB33F;
        let key_index: u8 = 0x42;
        let frame_counter: u32 = 0x12345678;

        let mut input: Vec<u8> = Vec::new();
        input.push(0b00010010);
        input.extend_from_slice(&frame_counter.to_le_bytes());
        input.extend_from_slice(&key.to_le_bytes());
        input.extend_from_slice(&key_index.to_be_bytes());

        let mut offset: usize = 0;
        let hdr: AuxiliarySecurityHeader = input.read(&mut offset).unwrap();

        assert_eq!(hdr.frame_counter, Some(frame_counter));
        assert_eq!(
            hdr.security_level,
            Some(SecurityLevel {
                mic: Mic::Mic64,
                encrypted: false
            })
        );
        assert_eq!(
            hdr.key_identifier_mode,
            KeyIdentifierMode::Key4(ShortKey(key), KeyIndex(key_index))
        );
    }
}
