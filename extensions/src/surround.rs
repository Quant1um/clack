use crate::audio_ports::AudioPortType;
use clack_common::extensions::{Extension, HostExtensionSide, PluginExtensionSide, RawExtension};
use clap_sys::ext::surround::*;
use std::ffi::CStr;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PluginSurround(RawExtension<PluginExtensionSide, clap_plugin_surround>);
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HostSurround(RawExtension<HostExtensionSide, clap_host_surround>);

// SAFETY: The type is repr(C) and ABI-compatible with the matching extension type.
unsafe impl Extension for PluginSurround {
    const IDENTIFIERS: &[&CStr] = &[CLAP_EXT_SURROUND, CLAP_EXT_SURROUND_COMPAT];
    type ExtensionSide = PluginExtensionSide;

    unsafe fn from_raw(raw: RawExtension<Self::ExtensionSide>) -> Self {
        // SAFETY: This type is expected to contain a type that is ABI-compatible with the matching extension type.
        Self(unsafe { raw.cast() })
    }
}

// SAFETY: The type is repr(C) and ABI-compatible with the matching extension type.
unsafe impl Extension for HostSurround {
    const IDENTIFIERS: &[&CStr] = &[CLAP_EXT_SURROUND, CLAP_EXT_SURROUND_COMPAT];
    type ExtensionSide = HostExtensionSide;

    unsafe fn from_raw(raw: RawExtension<Self::ExtensionSide>) -> Self {
        // SAFETY: This type is expected to contain a type that is ABI-compatible with the matching extension type.
        Self(unsafe { raw.cast() })
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SurroundChannel {
    FrontLeft = CLAP_SURROUND_FL as u8,
    FrontRight = CLAP_SURROUND_FR as u8,
    FrontCenter = CLAP_SURROUND_FC as u8,
    LowFrequency = CLAP_SURROUND_LFE as u8,
    BackLeft = CLAP_SURROUND_BL as u8,
    BackRight = CLAP_SURROUND_BR as u8,
    FrontLeftCenter = CLAP_SURROUND_FLC as u8,
    FrontRightCenter = CLAP_SURROUND_FRC as u8,
    BackCenter = CLAP_SURROUND_BC as u8,
    SideLeft = CLAP_SURROUND_SL as u8,
    SideRight = CLAP_SURROUND_SR as u8,
    TopCenter = CLAP_SURROUND_TC as u8,
    TopFrontLeft = CLAP_SURROUND_TFL as u8,
    TopFrontCenter = CLAP_SURROUND_TFC as u8,
    TopFrontRight = CLAP_SURROUND_TFR as u8,
    TopBackLeft = CLAP_SURROUND_TBL as u8,
    TopBackCenter = CLAP_SURROUND_TBC as u8,
    TopBackRight = CLAP_SURROUND_TBR as u8,
    // TopSideLeft = CLAP_SURROUND_TSL as u8,
    // TopSideRight = CLAP_SURROUND_TSR as u8,
}

bitflags::bitflags! {
    /// Flags for surround layouts.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct SurroundChannels: u64 {
        const FRONT_LEFT = 1u64 << CLAP_SURROUND_FL;
        const FRONT_RIGHT = 1u64 << CLAP_SURROUND_FR;
        const FRONT_CENTER = 1u64 << CLAP_SURROUND_FC;
        const LOW_FREQUENCY = 1u64 << CLAP_SURROUND_LFE;
        const BACK_LEFT = 1u64 << CLAP_SURROUND_BL;
        const BACK_RIGHT = 1u64 << CLAP_SURROUND_BR;
        const FRONT_LEFT_CENTER = 1u64 << CLAP_SURROUND_FLC;
        const FRONT_RIGHT_CENTER = 1u64 << CLAP_SURROUND_FRC;
        const BACK_CENTER = 1u64 << CLAP_SURROUND_BC;
        const SIDE_LEFT = 1u64 << CLAP_SURROUND_SL;
        const SIDE_RIGHT = 1u64 << CLAP_SURROUND_SR;
        const TOP_CENTER = 1u64 << CLAP_SURROUND_TC;
        const TOP_FRONT_LEFT = 1u64 << CLAP_SURROUND_TFL;
        const TOP_FRONT_CENTER = 1u64 << CLAP_SURROUND_TFC;
        const TOP_FRONT_RIGHT = 1u64 << CLAP_SURROUND_TFR;
        const TOP_BACK_LEFT = 1u64 << CLAP_SURROUND_TBL;
        const TOP_BACK_CENTER = 1u64 << CLAP_SURROUND_TBC;
        const TOP_BACK_RIGHT = 1u64 << CLAP_SURROUND_TBR;
        // const TOP_SIDE_LEFT = 1u64 << CLAP_SURROUND_TSL;
        // const TOP_SIDE_RIGHT = 1u64 << CLAP_SURROUND_TSR;
    }
}

impl AudioPortType<'static> {
    pub const SURROUND: Self = AudioPortType(CLAP_PORT_SURROUND);
}

impl SurroundChannel {
    pub fn from_raw(raw: u8) -> Option<Self> {
        match raw as u32 {
            i if i == CLAP_SURROUND_FL => Some(SurroundChannel::FrontLeft),
            i if i == CLAP_SURROUND_FR => Some(SurroundChannel::FrontRight),
            i if i == CLAP_SURROUND_FC => Some(SurroundChannel::FrontCenter),
            i if i == CLAP_SURROUND_LFE => Some(SurroundChannel::LowFrequency),
            i if i == CLAP_SURROUND_BL => Some(SurroundChannel::BackLeft),
            i if i == CLAP_SURROUND_BR => Some(SurroundChannel::BackRight),
            i if i == CLAP_SURROUND_FLC => Some(SurroundChannel::FrontLeftCenter),
            i if i == CLAP_SURROUND_FRC => Some(SurroundChannel::FrontRightCenter),
            i if i == CLAP_SURROUND_BC => Some(SurroundChannel::BackCenter),
            i if i == CLAP_SURROUND_SL => Some(SurroundChannel::SideLeft),
            i if i == CLAP_SURROUND_SR => Some(SurroundChannel::SideRight),
            i if i == CLAP_SURROUND_TC => Some(SurroundChannel::TopCenter),
            i if i == CLAP_SURROUND_TFL => Some(SurroundChannel::TopFrontLeft),
            i if i == CLAP_SURROUND_TFC => Some(SurroundChannel::TopFrontCenter),
            i if i == CLAP_SURROUND_TFR => Some(SurroundChannel::TopFrontRight),
            i if i == CLAP_SURROUND_TBL => Some(SurroundChannel::TopBackLeft),
            i if i == CLAP_SURROUND_TBC => Some(SurroundChannel::TopBackCenter),
            i if i == CLAP_SURROUND_TBR => Some(SurroundChannel::TopBackRight),
            _ => None,
        }
    }

    #[inline]
    pub fn to_raw(self) -> u8 {
        self as _
    }
}

impl From<SurroundChannel> for SurroundChannels {
    fn from(channel: SurroundChannel) -> Self {
        match channel {
            SurroundChannel::FrontLeft => SurroundChannels::FRONT_LEFT,
            SurroundChannel::FrontRight => SurroundChannels::FRONT_RIGHT,
            SurroundChannel::FrontCenter => SurroundChannels::FRONT_CENTER,
            SurroundChannel::LowFrequency => SurroundChannels::LOW_FREQUENCY,
            SurroundChannel::BackLeft => SurroundChannels::BACK_LEFT,
            SurroundChannel::BackRight => SurroundChannels::BACK_RIGHT,
            SurroundChannel::FrontLeftCenter => SurroundChannels::FRONT_LEFT_CENTER,
            SurroundChannel::FrontRightCenter => SurroundChannels::FRONT_RIGHT_CENTER,
            SurroundChannel::BackCenter => SurroundChannels::BACK_CENTER,
            SurroundChannel::SideLeft => SurroundChannels::SIDE_LEFT,
            SurroundChannel::SideRight => SurroundChannels::SIDE_RIGHT,
            SurroundChannel::TopCenter => SurroundChannels::TOP_CENTER,
            SurroundChannel::TopFrontLeft => SurroundChannels::TOP_FRONT_LEFT,
            SurroundChannel::TopFrontCenter => SurroundChannels::TOP_FRONT_CENTER,
            SurroundChannel::TopFrontRight => SurroundChannels::TOP_FRONT_RIGHT,
            SurroundChannel::TopBackLeft => SurroundChannels::TOP_BACK_LEFT,
            SurroundChannel::TopBackCenter => SurroundChannels::TOP_BACK_CENTER,
            SurroundChannel::TopBackRight => SurroundChannels::TOP_BACK_RIGHT,
        }
    }
}

impl Extend<SurroundChannel> for SurroundChannels {
    fn extend<T: IntoIterator<Item = SurroundChannel>>(&mut self, iter: T) {
        for channel in iter {
            *self |= SurroundChannels::from(channel);
        }
    }
}

impl FromIterator<SurroundChannel> for SurroundChannels {
    fn from_iter<I: IntoIterator<Item = SurroundChannel>>(iter: I) -> Self {
        let mut mask = SurroundChannels::empty();
        for channel in iter {
            mask |= SurroundChannels::from(channel);
        }
        mask
    }
}

#[cfg(feature = "clack-plugin")]
mod plugin;
#[cfg(feature = "clack-plugin")]
pub use plugin::*;

#[cfg(feature = "clack-host")]
mod host;
#[cfg(feature = "clack-host")]
pub use host::*;
