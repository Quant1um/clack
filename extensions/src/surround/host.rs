use crate::surround::{HostSurround, PluginSurround, SurroundChannel, SurroundChannels};
use clack_host::{
    extensions::{ExtensionImplementation, RawExtensionImplementation, wrapper::HostWrapper},
    host::HostHandlers,
    plugin::PluginMainThreadHandle,
};
use clap_sys::{ext::surround::clap_host_surround, host::clap_host};
use std::mem::MaybeUninit;

impl PluginSurround {
    /// Check if the plugin supports the a surround configuration mask.
    pub fn is_channel_mask_supported(
        &self,
        handle: &mut PluginMainThreadHandle,
        mask: SurroundChannels,
    ) -> bool {
        match handle.use_extension(&self.0).is_channel_mask_supported {
            // SAFETY: This type ensures the function pointer is valid.
            Some(is_channel_mask_supported) => unsafe {
                (is_channel_mask_supported)(handle.as_raw_ptr(), mask.bits())
            },
            None => false,
        }
    }

    // Fills the given writer with the surround channel map for the given port, if applicable.
    //
    // You should write exactly `channel_count` channels to the writer. This function should only be
    // called if the port its called for has `port_type` set to `SURROUND`.
    pub fn get_channel_map<'a>(
        &self,
        handle: &mut PluginMainThreadHandle,
        is_input: bool,
        port_index: u32,
        buffer: &'a mut [MaybeUninit<SurroundChannel>],
    ) -> &'a [SurroundChannel] {
        todo!()
    }
}

pub trait HostSurroundImpl {
    /// Notify the host that the surround configuration for one or more ports has changed.
    fn changed(&mut self);
}

// SAFETY: The given struct is the CLAP extension struct for the matching side of this extension.
unsafe impl<H> ExtensionImplementation<H> for HostSurround
where
    for<'a> H: HostHandlers<MainThread<'a>: HostSurroundImpl>,
{
    const IMPLEMENTATION: RawExtensionImplementation =
        RawExtensionImplementation::new(&clap_host_surround {
            changed: Some(changed::<H>),
        });
}

#[allow(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
unsafe extern "C" fn changed<H>(host: *const clap_host)
where
    for<'a> H: HostHandlers<MainThread<'a>: HostSurroundImpl>,
{
    unsafe {
        HostWrapper::<H>::handle(host, |host| {
            host.main_thread().as_mut().changed();
            Ok(())
        });
    }
}
