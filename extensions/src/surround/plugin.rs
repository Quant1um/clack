use crate::surround::{HostSurround, PluginSurround, SurroundChannel, SurroundChannels};
use clack_plugin::{
    extensions::{ExtensionImplementation, RawExtensionImplementation, wrapper::PluginWrapper},
    host::HostMainThreadHandle,
    plugin::Plugin,
};
use clap_sys::{ext::surround::clap_plugin_surround, plugin::clap_plugin};
use std::mem::MaybeUninit;

/// A writer for surround channel maps.
pub struct SurroundMapWriter<'a> {
    buf: &'a mut [MaybeUninit<SurroundChannel>],
    len: usize,
}

impl SurroundMapWriter<'_> {
    /// Returns the capacity of the writer (i.e., the maximum number of channels that can be written).
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Returns the number of channels currently written to the writer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if no channels have been written to the writer.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears the writer, removing all written channels.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Sets the channels in the writer to the given iterator, replacing any existing channels.
    pub fn set(&mut self, iter: impl IntoIterator<Item = SurroundChannel>) {
        self.clear();
        self.extend(iter);
    }
}

impl Extend<SurroundChannel> for SurroundMapWriter<'_> {
    fn extend<T: IntoIterator<Item = SurroundChannel>>(&mut self, iter: T) {
        for (slot, channel) in self.buf.iter_mut().skip(self.len).zip(iter) {
            slot.write(channel);
            self.len += 1;
        }
    }
}

impl HostSurround {
    /// Notify the host that the surround configuration for one or more ports has changed.
    pub fn changed(&self, handle: &mut HostMainThreadHandle) {
        if let Some(changed) = handle.use_extension(&self.0).changed {
            // SAFETY: This type ensures the function pointer is valid.
            unsafe { (changed)(handle.as_raw()) }
        }
    }
}

/// The plugin-side implementation of the Surround extension.
pub trait PluginSurroundImpl {
    /// Returns true if the given surround channel mask is supported.
    fn is_channel_mask_supported(&mut self, mask: SurroundChannels) -> bool;

    /// Fills the given writer with the surround channel map for the given port, if applicable.
    ///
    /// You should write exactly `channel_count` channels to the writer. This function should only be
    /// called if the port it is called for has a [`SURROUND`](crate::audio_ports::AudioPortType::SURROUND) type.
    fn get_channel_map(&mut self, is_input: bool, port_index: u32, writer: &mut SurroundMapWriter);
}

// SAFETY: The given struct is the CLAP extension struct for the matching side of this extension.
unsafe impl<P> ExtensionImplementation<P> for PluginSurround
where
    for<'a> P: Plugin<MainThread<'a>: PluginSurroundImpl>,
{
    const IMPLEMENTATION: RawExtensionImplementation =
        RawExtensionImplementation::new(&clap_plugin_surround {
            is_channel_mask_supported: Some(is_channel_mask_supported::<P>),
            get_channel_map: Some(get_channel_map::<P>),
        });
}

#[allow(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
unsafe extern "C" fn is_channel_mask_supported<P>(plugin: *const clap_plugin, mask: u64) -> bool
where
    for<'a> P: Plugin<MainThread<'a>: PluginSurroundImpl>,
{
    unsafe {
        PluginWrapper::<P>::handle(plugin, |plugin| {
            Ok(plugin
                .main_thread()
                .as_mut()
                .is_channel_mask_supported(SurroundChannels::from_bits_retain(mask)))
        })
        .unwrap_or(false)
    }
}

#[allow(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
unsafe extern "C" fn get_channel_map<P>(
    plugin: *const clap_plugin,
    is_input: bool,
    port_index: u32,
    out_channels: *mut u8,
    out_capacity: u32,
) -> u32
where
    for<'a> P: Plugin<MainThread<'a>: PluginSurroundImpl>,
{
    unsafe {
        PluginWrapper::<P>::handle(plugin, |plugin| {
            let mut writer = SurroundMapWriter {
                len: 0,
                buf: std::slice::from_raw_parts_mut(
                    out_channels as *mut MaybeUninit<SurroundChannel>,
                    out_capacity as usize,
                ),
            };

            plugin
                .main_thread()
                .as_mut()
                .get_channel_map(is_input, port_index, &mut writer);

            Ok(writer.len as u32)
        })
        .unwrap_or(0)
    }
}
