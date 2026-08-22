//! Safe, dynamically-loaded Rust bindings for BASS 2.4 and BASS_FX.
//!
//! The official BASS DLLs are intentionally not linked at build time. Use
//! [`BassEngine::load`] with an explicit path, or
//! [`BassEngine::load_with_options`] to provide an explicit BASS_FX path.

#![deny(unsafe_op_in_unsafe_fn)]

mod callback;
mod engine;
mod error;
mod fx;
pub mod midi;
mod path;

pub mod raw;

pub use engine::{
    ActiveState, BassEngine, BassEngineOptions, Channel, ChannelInfo, ChannelKind, DeviceInfo,
    DeviceType, DownloadCallback, DownloadEvent, InitOptions, OutputBackend, OutputInfo, Plugin,
    PluginFormat, PluginInfo, RemoteProgress, SourceOptions, SyncCallback, SyncEvent,
    SyncKind, SyncRegistration, TagKind,
    UrlOptions,
};
pub use error::{BassError, Result};
pub use fx::{
    BassFxEffect, DspCallback, DspInfo, DspRegistration, Effect, EffectKind, EffectParameters,
    FxLibrary, LoudnessChain, LoudnessOptions, ReverseChannel, TempoChannel,
};

/// Returns the BASS API version encoded in the official headers.
pub const BASS_API_VERSION: u16 = 0x204;

/// Returns the BASS_FX API version encoded in the official headers.
pub const BASS_FX_API_VERSION: u16 = 0x204;

#[cfg(test)]
mod tests {
    use std::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn defaults_enable_float_decode_and_prescan() {
        let options = SourceOptions::default();
        assert!(options.float);
        assert!(options.prescan);
        assert_eq!(options.stream_flags & raw::BASS_SAMPLE_FLOAT, 0);
    }

    #[test]
    fn backend_flags_match_official_constants() {
        assert_eq!(OutputBackend::Wasapi.flags(), 0);
        assert_eq!(OutputBackend::DirectSound.flags(), raw::BASS_DEVICE_DSOUND);
        assert_eq!(raw::BASS_DEVICE_MONO, 2);
    }

    #[test]
    fn fx_parameter_layout_matches_msvc_header_expectations() {
        assert_eq!(align_of::<raw::BASS_BFX_ENV_NODE>(), 4);
        assert_eq!(size_of::<raw::BASS_BFX_ENV_NODE>(), 12);
        assert_eq!(size_of::<raw::BASS_DX8_PARAMEQ>(), 12);
        assert_eq!(size_of::<raw::BASS_BFX_FREEVERB>(), 28);
    }

    #[test]
    fn midi_boundary_reports_unsupported_without_guessing_abi() {
        let error = midi::MidiAddon::load("definitely-missing-bassmidi.dll").unwrap_err();
        assert!(matches!(
            error,
            BassError::AddonUnavailable { name: "BASSMIDI" }
        ));
    }
}
