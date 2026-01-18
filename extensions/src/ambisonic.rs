use crate::audio_ports::AudioPortType;
use clack_common::extensions::{Extension, HostExtensionSide, PluginExtensionSide, RawExtension};
use clap_sys::ext::ambisonic::*;
use std::ffi::CStr;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PluginAmbisonic(RawExtension<PluginExtensionSide, clap_plugin_ambisonic>);
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HostAmbisonic(RawExtension<HostExtensionSide, clap_host_ambisonic>);

// SAFETY: The type is repr(C) and ABI-compatible with the matching extension type.
unsafe impl Extension for PluginAmbisonic {
    const IDENTIFIERS: &[&CStr] = &[CLAP_EXT_AMBISONIC, CLAP_EXT_AMBISONIC_COMPAT];
    type ExtensionSide = PluginExtensionSide;

    unsafe fn from_raw(raw: RawExtension<Self::ExtensionSide>) -> Self {
        // SAFETY: This type is expected to contain a type that is ABI-compatible with the matching extension type.
        Self(unsafe { raw.cast() })
    }
}

// SAFETY: The type is repr(C) and ABI-compatible with the matching extension type.
unsafe impl Extension for HostAmbisonic {
    const IDENTIFIERS: &[&CStr] = &[CLAP_EXT_AMBISONIC, CLAP_EXT_AMBISONIC_COMPAT];
    type ExtensionSide = HostExtensionSide;

    unsafe fn from_raw(raw: RawExtension<Self::ExtensionSide>) -> Self {
        // SAFETY: This type is expected to contain a type that is ABI-compatible with the matching extension type.
        Self(unsafe { raw.cast() })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AmbisonicConfig {
    pub ordering: AmbisonicOrdering,
    pub normalization: AmbisonicNormalization,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AmbisonicOrdering {
    FuMa = CLAP_AMBISONIC_ORDERING_FUMA,
    ACN = CLAP_AMBISONIC_ORDERING_ACN,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AmbisonicNormalization {
    MaxN = CLAP_AMBISONIC_NORMALIZATION_MAXN,
    SN3D = CLAP_AMBISONIC_NORMALIZATION_SN3D,
    SN2D = CLAP_AMBISONIC_NORMALIZATION_SN2D,
    N3D = CLAP_AMBISONIC_NORMALIZATION_N3D,
    N2D = CLAP_AMBISONIC_NORMALIZATION_N2D,
}

impl AmbisonicConfig {
    pub fn from_raw(raw: clap_ambisonic_config) -> Option<Self> {
        Some(Self {
            ordering: AmbisonicOrdering::from_raw(raw.ordering)?,
            normalization: AmbisonicNormalization::from_raw(raw.normalization)?,
        })
    }

    pub fn to_raw(self) -> clap_ambisonic_config {
        clap_ambisonic_config {
            ordering: self.ordering.to_raw(),
            normalization: self.normalization.to_raw(),
        }
    }
}

impl AmbisonicOrdering {
    pub fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            i if i == CLAP_AMBISONIC_ORDERING_FUMA => Some(AmbisonicOrdering::FuMa),
            i if i == CLAP_AMBISONIC_ORDERING_ACN => Some(AmbisonicOrdering::ACN),
            _ => None,
        }
    }

    pub fn to_raw(self) -> u32 {
        self as _
    }
}

impl AmbisonicNormalization {
    pub fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            i if i == CLAP_AMBISONIC_NORMALIZATION_MAXN => Some(AmbisonicNormalization::MaxN),
            i if i == CLAP_AMBISONIC_NORMALIZATION_SN3D => Some(AmbisonicNormalization::SN3D),
            i if i == CLAP_AMBISONIC_NORMALIZATION_SN2D => Some(AmbisonicNormalization::SN2D),
            i if i == CLAP_AMBISONIC_NORMALIZATION_N3D => Some(AmbisonicNormalization::N3D),
            i if i == CLAP_AMBISONIC_NORMALIZATION_N2D => Some(AmbisonicNormalization::N2D),
            _ => None,
        }
    }

    pub fn to_raw(self) -> u32 {
        self as _
    }
}

impl AudioPortType<'static> {
    pub const AMBISONIC: Self = AudioPortType(CLAP_PORT_AMBISONIC);
}

#[cfg(feature = "clack-plugin")]
mod plugin;
#[cfg(feature = "clack-plugin")]
pub use plugin::*;

#[cfg(feature = "clack-host")]
mod host;
#[cfg(feature = "clack-host")]
pub use host::*;
