use self::beacon::BeaconFrame;

pub mod beacon;

/// General kind of frames
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[derive(Debug, Clone)]
pub enum Frame<'a> {
    Beacon(BeaconFrame<'a>),
    EnhBeacon,
    Data,
    Acknowledgment,
    EnhAcknowledgment,
    MacCommand,
    MultiPurpose,
    Frak,
    Extended,
}
