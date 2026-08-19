use std::{
    ffi::c_void,
    mem::MaybeUninit,
    ops::Deref,
    sync::{Arc, atomic::Ordering},
};

use crate::{
    BassError, Result,
    callback::{self, DspContext},
    engine::{Channel, ChannelKind, EngineInner},
    error::{api_error, check_bool},
    raw,
};

/// A handle to the optional BASS_FX library.
#[derive(Clone)]
pub struct FxLibrary {
    pub(crate) inner: Arc<EngineInner>,
}

impl FxLibrary {
    pub fn version(&self) -> u32 {
        self.inner
            .fx
            .lock()
            .ok()
            .and_then(|fx| fx.as_ref().map(|fx| unsafe { (fx.get_version)() }))
            .unwrap_or(0)
    }
    pub fn tempo(&self, source: Channel, flags: u32) -> Result<TempoChannel> {
        source.into_tempo(flags)
    }
    pub fn reverse(&self, source: Channel, dec_block: f32, flags: u32) -> Result<ReverseChannel> {
        source.into_reverse(dec_block, flags)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BassFxEffect {
    Rotate,
    Echo,
    Flanger,
    Volume,
    PeakEq,
    Reverb,
    LowPassFilter,
    Mix,
    Damp,
    AutoWah,
    Echo2,
    Phaser,
    Echo3,
    Chorus,
    AllPassFilter,
    Compressor,
    Distortion,
    Compressor2,
    VolumeEnvelope,
    BiquadFilter,
    Echo4,
    PitchShift,
    Freeverb,
}
impl BassFxEffect {
    pub(crate) const fn raw(self) -> u32 {
        match self {
            Self::Rotate => raw::BASS_FX_BFX_ROTATE,
            Self::Echo => raw::BASS_FX_BFX_ECHO,
            Self::Flanger => raw::BASS_FX_BFX_FLANGER,
            Self::Volume => raw::BASS_FX_BFX_VOLUME,
            Self::PeakEq => raw::BASS_FX_BFX_PEAKEQ,
            Self::Reverb => raw::BASS_FX_BFX_REVERB,
            Self::LowPassFilter => raw::BASS_FX_BFX_LPF,
            Self::Mix => raw::BASS_FX_BFX_MIX,
            Self::Damp => raw::BASS_FX_BFX_DAMP,
            Self::AutoWah => raw::BASS_FX_BFX_AUTOWAH,
            Self::Echo2 => raw::BASS_FX_BFX_ECHO2,
            Self::Phaser => raw::BASS_FX_BFX_PHASER,
            Self::Echo3 => raw::BASS_FX_BFX_ECHO3,
            Self::Chorus => raw::BASS_FX_BFX_CHORUS,
            Self::AllPassFilter => raw::BASS_FX_BFX_APF,
            Self::Compressor => raw::BASS_FX_BFX_COMPRESSOR,
            Self::Distortion => raw::BASS_FX_BFX_DISTORTION,
            Self::Compressor2 => raw::BASS_FX_BFX_COMPRESSOR2,
            Self::VolumeEnvelope => raw::BASS_FX_BFX_VOLUME_ENV,
            Self::BiquadFilter => raw::BASS_FX_BFX_BQF,
            Self::Echo4 => raw::BASS_FX_BFX_ECHO4,
            Self::PitchShift => raw::BASS_FX_BFX_PITCHSHIFT,
            Self::Freeverb => raw::BASS_FX_BFX_FREEVERB,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectKind {
    Dx8(u32),
    BassFx(BassFxEffect),
    Volume,
}

/// A practical loudness-balancing chain built from official BASS_FX effects.
#[derive(Debug, Clone, Copy)]
pub struct LoudnessOptions {
    pub gain_db: f32,
    pub threshold_db: f32,
    pub ratio: f32,
    pub attack_ms: f32,
    pub release_ms: f32,
}

impl Default for LoudnessOptions {
    fn default() -> Self {
        Self {
            gain_db: 0.0,
            threshold_db: -18.0,
            ratio: 3.0,
            attack_ms: 10.0,
            release_ms: 100.0,
        }
    }
}

pub struct LoudnessChain {
    pub compressor: Effect,
}
impl EffectKind {
    pub(crate) const fn raw(self) -> u32 {
        match self {
            Self::Dx8(value) => value,
            Self::BassFx(value) => value.raw(),
            Self::Volume => raw::BASS_FX_VOLUME,
        }
    }
    pub(crate) const fn needs_fx(self) -> bool {
        matches!(self, Self::BassFx(_))
    }
}

/// A type marker for one of the official BASS/BASS_FX parameter structures.
pub trait EffectParameters: Copy + 'static {
    const KIND: EffectKind;
}

macro_rules! params {
    ($($ty:ty => $kind:expr),* $(,)?) => { $(impl EffectParameters for $ty { const KIND: EffectKind = $kind; })* };
}
params! {
    raw::BASS_DX8_CHORUS => EffectKind::Dx8(raw::BASS_FX_DX8_CHORUS),
    raw::BASS_DX8_COMPRESSOR => EffectKind::Dx8(raw::BASS_FX_DX8_COMPRESSOR),
    raw::BASS_DX8_DISTORTION => EffectKind::Dx8(raw::BASS_FX_DX8_DISTORTION),
    raw::BASS_DX8_ECHO => EffectKind::Dx8(raw::BASS_FX_DX8_ECHO),
    raw::BASS_DX8_FLANGER => EffectKind::Dx8(raw::BASS_FX_DX8_FLANGER),
    raw::BASS_DX8_GARGLE => EffectKind::Dx8(raw::BASS_FX_DX8_GARGLE),
    raw::BASS_DX8_I3DL2REVERB => EffectKind::Dx8(raw::BASS_FX_DX8_I3DL2REVERB),
    raw::BASS_DX8_PARAMEQ => EffectKind::Dx8(raw::BASS_FX_DX8_PARAMEQ),
    raw::BASS_DX8_REVERB => EffectKind::Dx8(raw::BASS_FX_DX8_REVERB),
    raw::BASS_FX_VOLUME_PARAM => EffectKind::Volume,
    raw::BASS_BFX_ROTATE => EffectKind::BassFx(BassFxEffect::Rotate),
    raw::BASS_BFX_ECHO => EffectKind::BassFx(BassFxEffect::Echo),
    raw::BASS_BFX_FLANGER => EffectKind::BassFx(BassFxEffect::Flanger),
    raw::BASS_BFX_VOLUME => EffectKind::BassFx(BassFxEffect::Volume),
    raw::BASS_BFX_PEAKEQ => EffectKind::BassFx(BassFxEffect::PeakEq),
    raw::BASS_BFX_REVERB => EffectKind::BassFx(BassFxEffect::Reverb),
    raw::BASS_BFX_LPF => EffectKind::BassFx(BassFxEffect::LowPassFilter),
    raw::BASS_BFX_MIX => EffectKind::BassFx(BassFxEffect::Mix),
    raw::BASS_BFX_DAMP => EffectKind::BassFx(BassFxEffect::Damp),
    raw::BASS_BFX_AUTOWAH => EffectKind::BassFx(BassFxEffect::AutoWah),
    raw::BASS_BFX_ECHO2 => EffectKind::BassFx(BassFxEffect::Echo2),
    raw::BASS_BFX_PHASER => EffectKind::BassFx(BassFxEffect::Phaser),
    raw::BASS_BFX_ECHO3 => EffectKind::BassFx(BassFxEffect::Echo3),
    raw::BASS_BFX_CHORUS => EffectKind::BassFx(BassFxEffect::Chorus),
    raw::BASS_BFX_APF => EffectKind::BassFx(BassFxEffect::AllPassFilter),
    raw::BASS_BFX_COMPRESSOR => EffectKind::BassFx(BassFxEffect::Compressor),
    raw::BASS_BFX_DISTORTION => EffectKind::BassFx(BassFxEffect::Distortion),
    raw::BASS_BFX_COMPRESSOR2 => EffectKind::BassFx(BassFxEffect::Compressor2),
    raw::BASS_BFX_VOLUME_ENV => EffectKind::BassFx(BassFxEffect::VolumeEnvelope),
    raw::BASS_BFX_BQF => EffectKind::BassFx(BassFxEffect::BiquadFilter),
    raw::BASS_BFX_ECHO4 => EffectKind::BassFx(BassFxEffect::Echo4),
    raw::BASS_BFX_PITCHSHIFT => EffectKind::BassFx(BassFxEffect::PitchShift),
    raw::BASS_BFX_FREEVERB => EffectKind::BassFx(BassFxEffect::Freeverb),
}

pub struct Effect {
    pub(crate) inner: Arc<EngineInner>,
    pub(crate) channel: u32,
    pub(crate) handle: u32,
    pub(crate) kind: EffectKind,
}
impl Effect {
    pub fn raw_handle(&self) -> u32 {
        self.handle
    }
    pub fn kind(&self) -> EffectKind {
        self.kind
    }
    pub fn set_parameters<P: EffectParameters>(&self, parameters: &P) -> Result<()> {
        if P::KIND != self.kind {
            return Err(BassError::InvalidInput {
                kind: "effect parameters",
                message: format!(
                    "parameters are for {:?}, effect is {:?}",
                    P::KIND,
                    self.kind
                ),
            });
        }
        let ok = unsafe {
            (self.inner.bass.fx_set_parameters)(self.handle, (parameters as *const P).cast())
        };
        check_bool("BASS_FXSetParameters", ok, self.error())
    }
    pub fn get_parameters<P: EffectParameters>(&self) -> Result<P> {
        if P::KIND != self.kind {
            return Err(BassError::InvalidInput {
                kind: "effect parameters",
                message: format!(
                    "parameters are for {:?}, effect is {:?}",
                    P::KIND,
                    self.kind
                ),
            });
        }
        let mut value = MaybeUninit::<P>::zeroed();
        let ok =
            unsafe { (self.inner.bass.fx_get_parameters)(self.handle, value.as_mut_ptr().cast()) };
        check_bool("BASS_FXGetParameters", ok, self.error())?;
        // SAFETY: the native function reported success and filled the C
        // parameter structure.
        Ok(unsafe { value.assume_init() })
    }
    pub fn set_priority(&self, priority: i32) -> Result<()> {
        let ok = unsafe { (self.inner.bass.fx_set_priority)(self.handle, priority) };
        check_bool("BASS_FXSetPriority", ok, self.error())
    }
    pub fn set_bypass(&self, bypass: bool) -> Result<()> {
        let ok = unsafe { (self.inner.bass.fx_set_bypass)(self.handle, bypass as i32) };
        check_bool("BASS_FXSetBypass", ok, self.error())
    }
    pub fn reset(&self) -> Result<()> {
        let ok = unsafe { (self.inner.bass.fx_reset)(self.handle) };
        check_bool("BASS_FXReset", ok, self.error())
    }
    fn error(&self) -> i32 {
        unsafe { (self.inner.bass.error_get_code)() }
    }
}
impl Drop for Effect {
    fn drop(&mut self) {
        unsafe {
            (self.inner.bass.channel_remove_fx)(self.channel, self.handle);
        }
    }
}

impl Channel {
    pub fn add_loudness(&self, options: LoudnessOptions, priority: i32) -> Result<LoudnessChain> {
        let compressor =
            self.add_effect(EffectKind::BassFx(BassFxEffect::Compressor2), priority)?;
        compressor.set_parameters(&raw::BASS_BFX_COMPRESSOR2 {
            fGain: options.gain_db,
            fThreshold: options.threshold_db,
            fRatio: options.ratio,
            fAttack: options.attack_ms,
            fRelease: options.release_ms,
            lChannel: raw::BASS_BFX_CHANALL,
        })?;
        Ok(LoudnessChain { compressor })
    }

    pub fn add_effect(&self, kind: EffectKind, priority: i32) -> Result<Effect> {
        if kind.needs_fx()
            && self
                .inner
                .fx
                .lock()
                .map_err(|_| BassError::FxUnavailable)?
                .is_none()
        {
            return Err(BassError::FxUnavailable);
        }
        let handle = unsafe { (self.inner.bass.channel_set_fx)(self.handle, kind.raw(), priority) };
        if handle == 0 {
            return Err(api_error("BASS_ChannelSetFX", unsafe {
                (self.inner.bass.error_get_code)()
            }));
        }
        Ok(Effect {
            inner: self.inner.clone(),
            channel: self.handle,
            handle,
            kind,
        })
    }

    pub fn add_dsp(&self, callback: DspCallback, priority: i32) -> Result<DspRegistration> {
        self.add_dsp_ex(callback, priority, 0)
    }
    pub fn add_dsp_ex(
        &self,
        callback: DspCallback,
        priority: i32,
        flags: u32,
    ) -> Result<DspRegistration> {
        let context = DspContext::new(callback);
        let user = Arc::as_ptr(&context) as *mut c_void;
        let handle = unsafe {
            (self.inner.bass.channel_set_dsp_ex)(
                self.handle,
                Some(callback::dsp_trampoline),
                user,
                priority,
                flags,
            )
        };
        if handle == 0 {
            context.alive.store(false, Ordering::Release);
            return Err(api_error("BASS_ChannelSetDSPEx", unsafe {
                (self.inner.bass.error_get_code)()
            }));
        }
        Ok(DspRegistration {
            inner: self.inner.clone(),
            channel: self.handle,
            handle,
            context,
        })
    }

    pub fn into_tempo(self, flags: u32) -> Result<TempoChannel> {
        let inner = self.inner.clone();
        let source = self.handle;
        let handle = {
            let fx = inner.fx.lock().map_err(|_| BassError::FxUnavailable)?;
            let Some(fx) = fx.as_ref() else {
                return Err(BassError::FxUnavailable);
            };
            unsafe { (fx.tempo_create)(source, flags | raw::BASS_FX_FREESOURCE) }
        };
        if handle == 0 {
            return Err(api_error("BASS_FX_TempoCreate", unsafe {
                (self.inner.bass.error_get_code)()
            }));
        }
        std::mem::forget(self);
        Ok(TempoChannel {
            channel: Channel::new(inner, handle, ChannelKind::Derived, None),
        })
    }

    pub fn into_reverse(self, dec_block: f32, flags: u32) -> Result<ReverseChannel> {
        let inner = self.inner.clone();
        let source = self.handle;
        let handle = {
            let fx = inner.fx.lock().map_err(|_| BassError::FxUnavailable)?;
            let Some(fx) = fx.as_ref() else {
                return Err(BassError::FxUnavailable);
            };
            unsafe { (fx.reverse_create)(source, dec_block, flags | raw::BASS_FX_FREESOURCE) }
        };
        if handle == 0 {
            return Err(api_error("BASS_FX_ReverseCreate", unsafe {
                (self.inner.bass.error_get_code)()
            }));
        }
        std::mem::forget(self);
        Ok(ReverseChannel {
            channel: Channel::new(inner, handle, ChannelKind::Derived, None),
        })
    }
}

pub struct TempoChannel {
    pub(crate) channel: Channel,
}
impl TempoChannel {
    pub fn tempo(&self) -> Result<f32> {
        self.channel.attribute(raw::BASS_ATTRIB_TEMPO)
    }
    pub fn set_tempo(&self, value: f32) -> Result<()> {
        self.channel.set_attribute(raw::BASS_ATTRIB_TEMPO, value)
    }
    pub fn pitch(&self) -> Result<f32> {
        self.channel.attribute(raw::BASS_ATTRIB_TEMPO_PITCH)
    }
    pub fn set_pitch(&self, value: f32) -> Result<()> {
        self.channel
            .set_attribute(raw::BASS_ATTRIB_TEMPO_PITCH, value)
    }
    pub fn tempo_frequency(&self) -> Result<f32> {
        self.channel.attribute(raw::BASS_ATTRIB_TEMPO_FREQ)
    }
    pub fn set_tempo_frequency(&self, value: f32) -> Result<()> {
        self.channel
            .set_attribute(raw::BASS_ATTRIB_TEMPO_FREQ, value)
    }
    pub fn rate_ratio(&self) -> f32 {
        self.channel
            .inner
            .fx
            .lock()
            .ok()
            .and_then(|fx| {
                fx.as_ref()
                    .map(|fx| unsafe { (fx.tempo_get_rate_ratio)(self.channel.handle) })
            })
            .unwrap_or(0.0)
    }
    pub fn source_handle(&self) -> u32 {
        self.channel
            .inner
            .fx
            .lock()
            .ok()
            .and_then(|fx| {
                fx.as_ref()
                    .map(|fx| unsafe { (fx.tempo_get_source)(self.channel.handle) })
            })
            .unwrap_or(0)
    }
}
impl Deref for TempoChannel {
    type Target = Channel;
    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}

pub struct ReverseChannel {
    pub(crate) channel: Channel,
}
impl ReverseChannel {
    pub fn direction(&self) -> Result<f32> {
        self.channel
            .inner
            .fx
            .lock()
            .ok()
            .and_then(|fx| fx.as_ref().map(|fx| self.channel_from_source(fx)))
            .ok_or(BassError::FxUnavailable)?
    }
    fn channel_from_source(&self, fx: &raw::FxApi) -> Result<f32> {
        let source = unsafe { (fx.reverse_get_source)(self.channel.handle) };
        self.channel
            .attribute_on(source, raw::BASS_ATTRIB_REVERSE_DIR)
    }
    pub fn set_direction(&self, direction: f32) -> Result<()> {
        let source = self.source_handle();
        self.channel
            .set_attribute_on(source, raw::BASS_ATTRIB_REVERSE_DIR, direction)
    }
    pub fn source_handle(&self) -> u32 {
        self.channel
            .inner
            .fx
            .lock()
            .ok()
            .and_then(|fx| {
                fx.as_ref()
                    .map(|fx| unsafe { (fx.reverse_get_source)(self.channel.handle) })
            })
            .unwrap_or(0)
    }
}
impl Deref for ReverseChannel {
    type Target = Channel;
    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}

impl Channel {
    fn attribute_on(&self, handle: u32, attribute: u32) -> Result<f32> {
        let mut value = 0.0;
        let ok = unsafe { (self.inner.bass.channel_get_attribute)(handle, attribute, &mut value) };
        check_bool("BASS_ChannelGetAttribute", ok, unsafe {
            (self.inner.bass.error_get_code)()
        })?;
        Ok(value)
    }
    fn set_attribute_on(&self, handle: u32, attribute: u32, value: f32) -> Result<()> {
        let ok = unsafe { (self.inner.bass.channel_set_attribute)(handle, attribute, value) };
        check_bool("BASS_ChannelSetAttribute", ok, unsafe {
            (self.inner.bass.error_get_code)()
        })
    }
}

pub type DspCallback = Box<dyn FnMut(&mut [u8], DspInfo) + Send + 'static>;
#[derive(Debug, Clone, Copy)]
pub struct DspInfo {
    pub dsp_handle: u32,
    pub channel: u32,
    pub byte_length: usize,
}

pub struct DspRegistration {
    pub(crate) inner: Arc<EngineInner>,
    pub(crate) channel: u32,
    pub(crate) handle: u32,
    pub(crate) context: Arc<DspContext>,
}
impl Drop for DspRegistration {
    fn drop(&mut self) {
        self.context.alive.store(false, Ordering::Release);
        unsafe {
            (self.inner.bass.channel_remove_dsp)(self.channel, self.handle);
        }
    }
}
