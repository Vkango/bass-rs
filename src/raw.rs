#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::{
    ffi::{c_char, c_void},
    mem::MaybeUninit,
    path::PathBuf,
};

use libloading::Library;

pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type QWORD = u64;
pub type BOOL = i32;
pub type HMUSIC = DWORD;
pub type HSAMPLE = DWORD;
pub type HCHANNEL = DWORD;
pub type HSTREAM = DWORD;
pub type HRECORD = DWORD;
pub type HSYNC = DWORD;
pub type HDSP = DWORD;
pub type HFX = DWORD;
pub type HPLUGIN = DWORD;

pub const BASSVERSION: WORD = 0x204;

pub const BASS_OK: i32 = 0;
pub const BASS_ERROR_MEM: i32 = 1;
pub const BASS_ERROR_FILEOPEN: i32 = 2;
pub const BASS_ERROR_DRIVER: i32 = 3;
pub const BASS_ERROR_BUFLOST: i32 = 4;
pub const BASS_ERROR_HANDLE: i32 = 5;
pub const BASS_ERROR_FORMAT: i32 = 6;
pub const BASS_ERROR_POSITION: i32 = 7;
pub const BASS_ERROR_INIT: i32 = 8;
pub const BASS_ERROR_START: i32 = 9;
pub const BASS_ERROR_SSL: i32 = 10;
pub const BASS_ERROR_REINIT: i32 = 11;
pub const BASS_ERROR_TRACK: i32 = 13;
pub const BASS_ERROR_ALREADY: i32 = 14;
pub const BASS_ERROR_NOTAUDIO: i32 = 17;
pub const BASS_ERROR_NOCHAN: i32 = 18;
pub const BASS_ERROR_ILLTYPE: i32 = 19;
pub const BASS_ERROR_ILLPARAM: i32 = 20;
pub const BASS_ERROR_NO3D: i32 = 21;
pub const BASS_ERROR_NOEAX: i32 = 22;
pub const BASS_ERROR_DEVICE: i32 = 23;
pub const BASS_ERROR_NOPLAY: i32 = 24;
pub const BASS_ERROR_FREQ: i32 = 25;
pub const BASS_ERROR_NOTFILE: i32 = 27;
pub const BASS_ERROR_NOHW: i32 = 29;
pub const BASS_ERROR_EMPTY: i32 = 31;
pub const BASS_ERROR_NONET: i32 = 32;
pub const BASS_ERROR_CREATE: i32 = 33;
pub const BASS_ERROR_NOFX: i32 = 34;
pub const BASS_ERROR_NOTAVAIL: i32 = 37;
pub const BASS_ERROR_DECODE: i32 = 38;
pub const BASS_ERROR_DX: i32 = 39;
pub const BASS_ERROR_TIMEOUT: i32 = 40;
pub const BASS_ERROR_FILEFORM: i32 = 41;
pub const BASS_ERROR_SPEAKER: i32 = 42;
pub const BASS_ERROR_VERSION: i32 = 43;
pub const BASS_ERROR_CODEC: i32 = 44;
pub const BASS_ERROR_ENDED: i32 = 45;
pub const BASS_ERROR_BUSY: i32 = 46;
pub const BASS_ERROR_UNSTREAMABLE: i32 = 47;
pub const BASS_ERROR_PROTOCOL: i32 = 48;
pub const BASS_ERROR_DENIED: i32 = 49;
pub const BASS_ERROR_FREEING: i32 = 50;
pub const BASS_ERROR_CANCEL: i32 = 51;

pub const BASS_CONFIG_BUFFER: DWORD = 0;
pub const BASS_CONFIG_UPDATEPERIOD: DWORD = 1;
pub const BASS_CONFIG_GVOL_SAMPLE: DWORD = 4;
pub const BASS_CONFIG_GVOL_STREAM: DWORD = 5;
pub const BASS_CONFIG_GVOL_MUSIC: DWORD = 6;
pub const BASS_CONFIG_CURVE_VOL: DWORD = 7;
pub const BASS_CONFIG_CURVE_PAN: DWORD = 8;
pub const BASS_CONFIG_FLOATDSP: DWORD = 9;
pub const BASS_CONFIG_3DALGORITHM: DWORD = 10;
pub const BASS_CONFIG_NET_TIMEOUT: DWORD = 11;
pub const BASS_CONFIG_NET_BUFFER: DWORD = 12;
pub const BASS_CONFIG_PAUSE_NOPLAY: DWORD = 13;
pub const BASS_CONFIG_NET_PREBUF: DWORD = 15;
pub const BASS_CONFIG_NET_PASSIVE: DWORD = 18;
pub const BASS_CONFIG_REC_BUFFER: DWORD = 19;
pub const BASS_CONFIG_NET_PLAYLIST: DWORD = 21;
pub const BASS_CONFIG_MUSIC_VIRTUAL: DWORD = 22;
pub const BASS_CONFIG_VERIFY: DWORD = 23;
pub const BASS_CONFIG_UPDATETHREADS: DWORD = 24;
pub const BASS_CONFIG_DEV_BUFFER: DWORD = 27;
pub const BASS_CONFIG_REC_LOOPBACK: DWORD = 28;
pub const BASS_CONFIG_DEV_DEFAULT: DWORD = 36;
pub const BASS_CONFIG_NET_READTIMEOUT: DWORD = 37;
pub const BASS_CONFIG_VISTA_SPEAKERS: DWORD = 38;
pub const BASS_CONFIG_MF_DISABLE: DWORD = 40;
pub const BASS_CONFIG_HANDLES: DWORD = 41;
pub const BASS_CONFIG_UNICODE: DWORD = 42;
pub const BASS_CONFIG_SRC: DWORD = 43;
pub const BASS_CONFIG_SRC_SAMPLE: DWORD = 44;
pub const BASS_CONFIG_ASYNCFILE_BUFFER: DWORD = 45;
pub const BASS_CONFIG_OGG_PRESCAN: DWORD = 47;
pub const BASS_CONFIG_VIDEO: DWORD = 48;
pub const BASS_CONFIG_DEV_NONSTOP: DWORD = 50;
pub const BASS_CONFIG_VERIFY_NET: DWORD = 52;
pub const BASS_CONFIG_DEV_PERIOD: DWORD = 53;
pub const BASS_CONFIG_FLOAT: DWORD = 54;
pub const BASS_CONFIG_NET_SEEK: DWORD = 56;
pub const BASS_CONFIG_NET_PLAYLIST_DEPTH: DWORD = 59;
pub const BASS_CONFIG_NET_PREBUF_WAIT: DWORD = 60;
pub const BASS_CONFIG_WASAPI_PERSIST: DWORD = 65;
pub const BASS_CONFIG_REC_WASAPI: DWORD = 66;
pub const BASS_CONFIG_SAMPLE_ONEHANDLE: DWORD = 69;
pub const BASS_CONFIG_NET_META: DWORD = 71;
pub const BASS_CONFIG_NET_RESTRATE: DWORD = 72;
pub const BASS_CONFIG_REC_DEFAULT: DWORD = 73;
pub const BASS_CONFIG_NORAMP: DWORD = 74;
pub const BASS_CONFIG_NOSOUND_MAXDELAY: DWORD = 76;
pub const BASS_CONFIG_DOWNMIX: DWORD = 80;
pub const BASS_CONFIG_NET_AGENT: DWORD = 16;
pub const BASS_CONFIG_NET_PROXY: DWORD = 17;
pub const BASS_CONFIG_DEV_NOTIFY: DWORD = 33;
pub const BASS_CONFIG_FILENAME: DWORD = 75;
pub const BASS_CONFIG_THREAD: DWORD = 0x4000_0000;

pub const BASS_DEVICE_MONO: DWORD = 2;
pub const BASS_DEVICE_REINIT: DWORD = 0x80;
pub const BASS_DEVICE_SPEAKERS: DWORD = 0x800;
pub const BASS_DEVICE_NOSPEAKER: DWORD = 0x1000;
pub const BASS_DEVICE_FREQ: DWORD = 0x4000;
pub const BASS_DEVICE_STEREO: DWORD = 0x8000;
pub const BASS_DEVICE_HOG: DWORD = 0x10000;
pub const BASS_DEVICE_DSOUND: DWORD = 0x40000;
pub const BASS_DEVICE_SOFTWARE: DWORD = 0x80000;

pub const BASS_DEVICE_ENABLED: DWORD = 1;
pub const BASS_DEVICE_DEFAULT: DWORD = 2;
pub const BASS_DEVICE_INIT: DWORD = 4;
pub const BASS_DEVICE_LOOPBACK: DWORD = 8;
pub const BASS_DEVICE_DEFAULTCOM: DWORD = 0x80;
pub const BASS_DEVICE_TYPE_MASK: DWORD = 0xff00_0000;
pub const BASS_DEVICE_TYPE_NETWORK: DWORD = 0x0100_0000;
pub const BASS_DEVICE_TYPE_SPEAKERS: DWORD = 0x0200_0000;
pub const BASS_DEVICE_TYPE_LINE: DWORD = 0x0300_0000;
pub const BASS_DEVICE_TYPE_HEADPHONES: DWORD = 0x0400_0000;
pub const BASS_DEVICE_TYPE_MICROPHONE: DWORD = 0x0500_0000;
pub const BASS_DEVICE_TYPE_HEADSET: DWORD = 0x0600_0000;
pub const BASS_DEVICE_TYPE_HANDSET: DWORD = 0x0700_0000;
pub const BASS_DEVICE_TYPE_DIGITAL: DWORD = 0x0800_0000;
pub const BASS_DEVICE_TYPE_SPDIF: DWORD = 0x0900_0000;
pub const BASS_DEVICE_TYPE_HDMI: DWORD = 0x0a00_0000;
pub const BASS_DEVICE_TYPE_DISPLAYPORT: DWORD = 0x4000_0000;

pub const BASS_FILE_NAME: DWORD = 0;
pub const BASS_FILE_MEM: DWORD = 1;
pub const BASS_FILE_MEMCOPY: DWORD = 3;
pub const BASS_FILE_HANDLE: DWORD = 4;

pub const BASS_SAMPLE_MONO: DWORD = 2;
pub const BASS_SAMPLE_LOOP: DWORD = 4;
pub const BASS_SAMPLE_FLOAT: DWORD = 0x100;
pub const BASS_SAMPLE_FX: DWORD = 0x80;
pub const BASS_STREAM_PRESCAN: DWORD = 0x20000;
pub const BASS_STREAM_AUTOFREE: DWORD = 0x40000;
pub const BASS_STREAM_RESTRATE: DWORD = 0x80000;
pub const BASS_STREAM_BLOCK: DWORD = 0x100000;
pub const BASS_STREAM_DECODE: DWORD = 0x200000;
pub const BASS_STREAM_STATUS: DWORD = 0x800000;
pub const BASS_MUSIC_FLOAT: DWORD = BASS_SAMPLE_FLOAT;
pub const BASS_MUSIC_MONO: DWORD = BASS_SAMPLE_MONO;
pub const BASS_MUSIC_LOOP: DWORD = BASS_SAMPLE_LOOP;
pub const BASS_MUSIC_DECODE: DWORD = BASS_STREAM_DECODE;
pub const BASS_MUSIC_PRESCAN: DWORD = BASS_STREAM_PRESCAN;
pub const BASS_MUSIC_RAMP: DWORD = 0x200;
pub const BASS_MUSIC_RAMPS: DWORD = 0x400;
pub const BASS_MUSIC_SURROUND: DWORD = 0x800;
pub const BASS_MUSIC_SURROUND2: DWORD = 0x1000;
pub const BASS_MUSIC_SINCINTER: DWORD = 0x800000;
pub const BASS_MUSIC_AUTOFREE: DWORD = BASS_STREAM_AUTOFREE;

pub const BASS_UNICODE: DWORD = 0x8000_0000;
pub const BASS_FX_FREESOURCE: DWORD = 0x10000;

pub const BASS_FILEPOS_CURRENT: DWORD = 0;
pub const BASS_FILEPOS_DOWNLOAD: DWORD = 1;
pub const BASS_FILEPOS_END: DWORD = 2;
pub const BASS_FILEPOS_START: DWORD = 3;
pub const BASS_FILEPOS_CONNECTED: DWORD = 4;
pub const BASS_FILEPOS_BUFFER: DWORD = 5;
pub const BASS_FILEPOS_SOCKET: DWORD = 6;
pub const BASS_FILEPOS_ASYNCBUF: DWORD = 7;
pub const BASS_FILEPOS_SIZE: DWORD = 8;
pub const BASS_FILEPOS_BUFFERING: DWORD = 9;
pub const BASS_FILEPOS_AVAILABLE: DWORD = 10;

pub const BASS_SYNC_POS: DWORD = 0;
pub const BASS_SYNC_END: DWORD = 2;
pub const BASS_SYNC_META: DWORD = 4;
pub const BASS_SYNC_STALL: DWORD = 6;
pub const BASS_SYNC_DOWNLOAD: DWORD = 7;
pub const BASS_SYNC_FREE: DWORD = 8;
pub const BASS_SYNC_OGG_CHANGE: DWORD = 12;
pub const BASS_SYNC_THREAD: DWORD = 0x2000_0000;
pub const BASS_SYNC_MIXTIME: DWORD = 0x4000_0000;
pub const BASS_SYNC_ONETIME: DWORD = 0x8000_0000;

pub const BASS_ACTIVE_STOPPED: DWORD = 0;
pub const BASS_ACTIVE_PLAYING: DWORD = 1;
pub const BASS_ACTIVE_STALLED: DWORD = 2;
pub const BASS_ACTIVE_PAUSED: DWORD = 3;
pub const BASS_ACTIVE_PAUSED_DEVICE: DWORD = 4;

pub const BASS_ATTRIB_FREQ: DWORD = 1;
pub const BASS_ATTRIB_VOL: DWORD = 2;
pub const BASS_ATTRIB_PAN: DWORD = 3;
pub const BASS_ATTRIB_BUFFER: DWORD = 13;
pub const BASS_ATTRIB_DOWNLOADPROC: DWORD = 18;
pub const BASS_ATTRIB_DOWNMIX: DWORD = 21;
pub const BASS_ATTRIB_TEMPO: DWORD = 0x10000;
pub const BASS_ATTRIB_TEMPO_PITCH: DWORD = 0x10001;
pub const BASS_ATTRIB_TEMPO_FREQ: DWORD = 0x10002;
pub const BASS_ATTRIB_REVERSE_DIR: DWORD = 0x11000;

pub const BASS_POS_BYTE: DWORD = 0;
pub const BASS_POS_MUSIC_ORDER: DWORD = 1;
pub const BASS_POS_DSP: DWORD = 0x800000;
pub const BASS_POS_FLUSH: DWORD = 0x1000000;
pub const BASS_POS_RELATIVE: DWORD = 0x4000000;
pub const BASS_POS_INEXACT: DWORD = 0x8000000;
pub const BASS_POS_DECODE: DWORD = 0x10000000;

pub const BASS_DATA_AVAILABLE: DWORD = 0;
pub const BASS_DATA_FLOAT: DWORD = 0x4000_0000;
pub const BASS_DATA_FFT256: DWORD = 0x8000_0000;
pub const BASS_DATA_FFT512: DWORD = 0x8000_0001;
pub const BASS_DATA_FFT1024: DWORD = 0x8000_0002;
pub const BASS_DATA_FFT2048: DWORD = 0x8000_0003;
pub const BASS_DATA_FFT4096: DWORD = 0x8000_0004;
pub const BASS_DATA_FFT8192: DWORD = 0x8000_0005;
pub const BASS_DATA_FFT_INDIVIDUAL: DWORD = 0x10;
pub const BASS_DATA_FFT_NOWINDOW: DWORD = 0x20;
pub const BASS_DATA_FFT_REMOVEDC: DWORD = 0x40;
pub const BASS_DATA_FFT_COMPLEX: DWORD = 0x80;
pub const BASS_DATA_FFT_NYQUIST: DWORD = 0x100;

pub const BASS_LEVEL_MONO: DWORD = 1;
pub const BASS_LEVEL_STEREO: DWORD = 2;
pub const BASS_LEVEL_RMS: DWORD = 4;
pub const BASS_LEVEL_VOLPAN: DWORD = 8;

pub const BASS_TAG_ID3: DWORD = 0;
pub const BASS_TAG_ID3V2: DWORD = 1;
pub const BASS_TAG_OGG: DWORD = 2;
pub const BASS_TAG_HTTP: DWORD = 3;
pub const BASS_TAG_ICY: DWORD = 4;
pub const BASS_TAG_META: DWORD = 5;
pub const BASS_TAG_APE: DWORD = 6;
pub const BASS_TAG_MP4: DWORD = 7;
pub const BASS_TAG_WMA: DWORD = 8;
pub const BASS_TAG_VENDOR: DWORD = 9;
pub const BASS_TAG_MF: DWORD = 13;

pub const BASS_DSP_READONLY: DWORD = 1;
pub const BASS_DSP_FLOAT: DWORD = 2;
pub const BASS_DSP_FREECALL: DWORD = 4;
pub const BASS_DSP_BYPASS: DWORD = 0x400000;

pub const BASS_FX_DX8_CHORUS: DWORD = 0;
pub const BASS_FX_DX8_COMPRESSOR: DWORD = 1;
pub const BASS_FX_DX8_DISTORTION: DWORD = 2;
pub const BASS_FX_DX8_ECHO: DWORD = 3;
pub const BASS_FX_DX8_FLANGER: DWORD = 4;
pub const BASS_FX_DX8_GARGLE: DWORD = 5;
pub const BASS_FX_DX8_I3DL2REVERB: DWORD = 6;
pub const BASS_FX_DX8_PARAMEQ: DWORD = 7;
pub const BASS_FX_DX8_REVERB: DWORD = 8;
pub const BASS_FX_VOLUME: DWORD = 9;

pub const BASS_FX_BFX_ROTATE: DWORD = 0x10000;
pub const BASS_FX_BFX_ECHO: DWORD = 0x10001;
pub const BASS_FX_BFX_FLANGER: DWORD = 0x10002;
pub const BASS_FX_BFX_VOLUME: DWORD = 0x10003;
pub const BASS_FX_BFX_PEAKEQ: DWORD = 0x10004;
pub const BASS_FX_BFX_REVERB: DWORD = 0x10005;
pub const BASS_FX_BFX_LPF: DWORD = 0x10006;
pub const BASS_FX_BFX_MIX: DWORD = 0x10007;
pub const BASS_FX_BFX_DAMP: DWORD = 0x10008;
pub const BASS_FX_BFX_AUTOWAH: DWORD = 0x10009;
pub const BASS_FX_BFX_ECHO2: DWORD = 0x1000a;
pub const BASS_FX_BFX_PHASER: DWORD = 0x1000b;
pub const BASS_FX_BFX_ECHO3: DWORD = 0x1000c;
pub const BASS_FX_BFX_CHORUS: DWORD = 0x1000d;
pub const BASS_FX_BFX_APF: DWORD = 0x1000e;
pub const BASS_FX_BFX_COMPRESSOR: DWORD = 0x1000f;
pub const BASS_FX_BFX_DISTORTION: DWORD = 0x10010;
pub const BASS_FX_BFX_COMPRESSOR2: DWORD = 0x10011;
pub const BASS_FX_BFX_VOLUME_ENV: DWORD = 0x10012;
pub const BASS_FX_BFX_BQF: DWORD = 0x10013;
pub const BASS_FX_BFX_ECHO4: DWORD = 0x10014;
pub const BASS_FX_BFX_PITCHSHIFT: DWORD = 0x10015;
pub const BASS_FX_BFX_FREEVERB: DWORD = 0x10016;

pub const BASS_BFX_CHANALL: i32 = -1;
pub const BASS_BFX_CHANNONE: i32 = 0;
pub const BASS_BFX_CHAN1: i32 = 1;
pub const BASS_BFX_CHAN2: i32 = 2;
pub const BASS_BFX_CHAN3: i32 = 4;
pub const BASS_BFX_CHAN4: i32 = 8;
pub const BASS_BFX_CHAN5: i32 = 16;
pub const BASS_BFX_CHAN6: i32 = 32;
pub const BASS_BFX_CHAN7: i32 = 64;
pub const BASS_BFX_CHAN8: i32 = 128;
pub const BASS_BFX_BQF_LOWPASS: i32 = 0;
pub const BASS_BFX_BQF_HIGHPASS: i32 = 1;
pub const BASS_BFX_BQF_BANDPASS: i32 = 2;
pub const BASS_BFX_BQF_BANDPASS_Q: i32 = 3;
pub const BASS_BFX_BQF_NOTCH: i32 = 4;
pub const BASS_BFX_BQF_ALLPASS: i32 = 5;
pub const BASS_BFX_BQF_PEAKINGEQ: i32 = 6;
pub const BASS_BFX_BQF_LOWSHELF: i32 = 7;
pub const BASS_BFX_BQF_HIGHSHELF: i32 = 8;

pub const BASS_FX_TEMPO_ALGO_LINEAR: DWORD = 0x200;
pub const BASS_FX_TEMPO_ALGO_CUBIC: DWORD = 0x400;
pub const BASS_FX_TEMPO_ALGO_SHANNON: DWORD = 0x800;
pub const BASS_FX_RVS_REVERSE: f32 = -1.0;
pub const BASS_FX_RVS_FORWARD: f32 = 1.0;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BASS_DEVICEINFO {
    pub name: *const c_char,
    pub driver: *const c_char,
    pub flags: DWORD,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BASS_INFO {
    pub flags: DWORD,
    pub reserved: [DWORD; 7],
    pub minbuf: DWORD,
    pub dsver: DWORD,
    pub latency: DWORD,
    pub initflags: DWORD,
    pub speakers: DWORD,
    pub freq: DWORD,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BASS_CHANNELINFO {
    pub freq: DWORD,
    pub chans: DWORD,
    pub flags: DWORD,
    pub ctype: DWORD,
    pub origres: DWORD,
    pub plugin: HPLUGIN,
    pub sample: HSAMPLE,
    pub filename: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_PLUGINFORM {
    pub ctype: DWORD,
    pub name: *const c_char,
    pub exts: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_PLUGININFO {
    pub version: DWORD,
    pub formatc: DWORD,
    pub formats: *const BASS_PLUGINFORM,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BASS_3DVECTOR {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type STREAMPROC = unsafe extern "system" fn(HSTREAM, *mut c_void, DWORD, *mut c_void) -> DWORD;
pub type DOWNLOADPROC = unsafe extern "system" fn(*const c_void, DWORD, *mut c_void);
pub type SYNCPROC = unsafe extern "system" fn(HSYNC, DWORD, DWORD, *mut c_void);
pub type DSPPROC = unsafe extern "system" fn(HDSP, DWORD, *mut c_void, DWORD, *mut c_void);
pub type RECORDPROC = unsafe extern "system" fn(HRECORD, *const c_void, DWORD, *mut c_void) -> BOOL;
pub type BPMPROC = unsafe extern "system" fn(DWORD, f32, *mut c_void);
pub type BPMPROGRESSPROC = unsafe extern "system" fn(DWORD, f32, *mut c_void);
pub type BPMBEATPROC = unsafe extern "system" fn(DWORD, f64, *mut c_void);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_FILEPROCS {
    pub close: Option<unsafe extern "system" fn(*mut c_void)>,
    pub length: Option<unsafe extern "system" fn(*mut c_void) -> QWORD>,
    pub read: Option<unsafe extern "system" fn(*mut c_void, DWORD, *mut c_void) -> DWORD>,
    pub seek: Option<unsafe extern "system" fn(QWORD, *mut c_void) -> BOOL>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_CHORUS {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: DWORD,
    pub fDelay: f32,
    pub lPhase: DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_COMPRESSOR {
    pub fGain: f32,
    pub fAttack: f32,
    pub fRelease: f32,
    pub fThreshold: f32,
    pub fRatio: f32,
    pub fPredelay: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_DISTORTION {
    pub fGain: f32,
    pub fEdge: f32,
    pub fPostEQCenterFrequency: f32,
    pub fPostEQBandwidth: f32,
    pub fPreLowpassCutoff: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_ECHO {
    pub fWetDryMix: f32,
    pub fFeedback: f32,
    pub fLeftDelay: f32,
    pub fRightDelay: f32,
    pub lPanDelay: BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_FLANGER {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: DWORD,
    pub fDelay: f32,
    pub lPhase: DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_GARGLE {
    pub dwRateHz: DWORD,
    pub dwWaveShape: DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_I3DL2REVERB {
    pub lRoom: i32,
    pub lRoomHF: i32,
    pub flRoomRolloffFactor: f32,
    pub flDecayTime: f32,
    pub flDecayHFRatio: f32,
    pub lReflections: i32,
    pub flReflectionsDelay: f32,
    pub lReverb: i32,
    pub flReverbDelay: f32,
    pub flDiffusion: f32,
    pub flDensity: f32,
    pub flHFReference: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_PARAMEQ {
    pub fCenter: f32,
    pub fBandwidth: f32,
    pub fGain: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_DX8_REVERB {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_FX_VOLUME_PARAM {
    pub fTarget: f32,
    pub fCurrent: f32,
    pub fTime: f32,
    pub lCurve: DWORD,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_ROTATE {
    pub fRate: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_ECHO {
    pub fLevel: f32,
    pub lDelay: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_FLANGER {
    pub fWetDry: f32,
    pub fSpeed: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_VOLUME {
    pub lChannel: i32,
    pub fVolume: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_PEAKEQ {
    pub lBand: i32,
    pub fBandwidth: f32,
    pub fQ: f32,
    pub fCenter: f32,
    pub fGain: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_REVERB {
    pub fLevel: f32,
    pub lDelay: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_LPF {
    pub fResonance: f32,
    pub fCutOffFreq: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_MIX {
    pub lChannel: *const i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_DAMP {
    pub fTarget: f32,
    pub fQuiet: f32,
    pub fRate: f32,
    pub fGain: f32,
    pub fDelay: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_AUTOWAH {
    pub fDryMix: f32,
    pub fWetMix: f32,
    pub fFeedback: f32,
    pub fRate: f32,
    pub fRange: f32,
    pub fFreq: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_ECHO2 {
    pub fDryMix: f32,
    pub fWetMix: f32,
    pub fFeedback: f32,
    pub fDelay: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_PHASER {
    pub fDryMix: f32,
    pub fWetMix: f32,
    pub fFeedback: f32,
    pub fRate: f32,
    pub fRange: f32,
    pub fFreq: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_ECHO3 {
    pub fDryMix: f32,
    pub fWetMix: f32,
    pub fDelay: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_CHORUS {
    pub fDryMix: f32,
    pub fWetMix: f32,
    pub fFeedback: f32,
    pub fMinSweep: f32,
    pub fMaxSweep: f32,
    pub fRate: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_APF {
    pub fGain: f32,
    pub fDelay: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_COMPRESSOR {
    pub fThreshold: f32,
    pub fAttacktime: f32,
    pub fReleasetime: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_DISTORTION {
    pub fDrive: f32,
    pub fDryMix: f32,
    pub fWetMix: f32,
    pub fFeedback: f32,
    pub fVolume: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_COMPRESSOR2 {
    pub fGain: f32,
    pub fThreshold: f32,
    pub fRatio: f32,
    pub fAttack: f32,
    pub fRelease: f32,
    pub lChannel: i32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_ENV_NODE {
    pub pos: f64,
    pub val: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_VOLUME_ENV {
    pub lChannel: i32,
    pub lNodeCount: i32,
    pub pNodes: *const BASS_BFX_ENV_NODE,
    pub bFollow: BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_BQF {
    pub lFilter: i32,
    pub fCenter: f32,
    pub fGain: f32,
    pub fBandwidth: f32,
    pub fQ: f32,
    pub fS: f32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_ECHO4 {
    pub fDryMix: f32,
    pub fWetMix: f32,
    pub fFeedback: f32,
    pub fDelay: f32,
    pub bStereo: BOOL,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_PITCHSHIFT {
    pub fPitchShift: f32,
    pub fSemitones: f32,
    pub lFFTsize: i32,
    pub lOsamp: i32,
    pub lChannel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BASS_BFX_FREEVERB {
    pub fDryMix: f32,
    pub fWetMix: f32,
    pub fRoomSize: f32,
    pub fDamp: f32,
    pub fWidth: f32,
    pub lMode: DWORD,
    pub lChannel: i32,
}

type FnSetConfig = unsafe extern "system" fn(DWORD, DWORD) -> BOOL;
type FnGetConfig = unsafe extern "system" fn(DWORD) -> DWORD;
type FnSetConfigPtr = unsafe extern "system" fn(DWORD, *const c_void) -> BOOL;
type FnGetConfigPtr = unsafe extern "system" fn(DWORD) -> *const c_void;
type FnGetVersion = unsafe extern "system" fn() -> DWORD;
type FnErrorGetCode = unsafe extern "system" fn() -> i32;
type FnGetDeviceInfo = unsafe extern "system" fn(DWORD, *mut BASS_DEVICEINFO) -> BOOL;
type FnInit = unsafe extern "system" fn(i32, DWORD, DWORD, *mut c_void, *const c_void) -> BOOL;
type FnFree = unsafe extern "system" fn() -> BOOL;
type FnSetDevice = unsafe extern "system" fn(DWORD) -> BOOL;
type FnGetDevice = unsafe extern "system" fn() -> DWORD;
type FnGetInfo = unsafe extern "system" fn(*mut BASS_INFO) -> BOOL;
type FnStart = unsafe extern "system" fn() -> BOOL;
type FnStop = unsafe extern "system" fn() -> BOOL;
type FnPause = unsafe extern "system" fn() -> BOOL;
type FnIsStarted = unsafe extern "system" fn() -> DWORD;
type FnUpdate = unsafe extern "system" fn(DWORD) -> BOOL;
type FnGetCpu = unsafe extern "system" fn() -> f32;
type FnSetVolume = unsafe extern "system" fn(f32) -> BOOL;
type FnGetVolume = unsafe extern "system" fn() -> f32;
type FnPluginLoad = unsafe extern "system" fn(*const c_char, DWORD) -> HPLUGIN;
type FnPluginFree = unsafe extern "system" fn(HPLUGIN) -> BOOL;
type FnPluginEnable = unsafe extern "system" fn(HPLUGIN, BOOL) -> BOOL;
type FnPluginGetInfo = unsafe extern "system" fn(HPLUGIN) -> *const BASS_PLUGININFO;
type FnSampleLoad =
    unsafe extern "system" fn(DWORD, *const c_void, QWORD, DWORD, DWORD, DWORD) -> HSAMPLE;
type FnSampleCreate = unsafe extern "system" fn(DWORD, DWORD, DWORD, DWORD, DWORD) -> HSAMPLE;
type FnSampleFree = unsafe extern "system" fn(HSAMPLE) -> BOOL;
type FnSampleSetData = unsafe extern "system" fn(HSAMPLE, *const c_void) -> BOOL;
type FnSampleGetData = unsafe extern "system" fn(HSAMPLE, *mut c_void) -> BOOL;
type FnSampleGetInfo = unsafe extern "system" fn(HSAMPLE, *mut c_void) -> BOOL;
type FnSampleSetInfo = unsafe extern "system" fn(HSAMPLE, *const c_void) -> BOOL;
type FnSampleGetChannel = unsafe extern "system" fn(HSAMPLE, DWORD) -> HCHANNEL;
type FnSampleGetChannels = unsafe extern "system" fn(HSAMPLE, *mut HCHANNEL) -> DWORD;
type FnSampleStop = unsafe extern "system" fn(HSAMPLE) -> BOOL;
type FnStreamCreate =
    unsafe extern "system" fn(DWORD, DWORD, DWORD, Option<STREAMPROC>, *mut c_void) -> HSTREAM;
type FnStreamCreateFile =
    unsafe extern "system" fn(DWORD, *const c_void, QWORD, QWORD, DWORD) -> HSTREAM;
type FnStreamCreateUrl = unsafe extern "system" fn(
    *const c_char,
    DWORD,
    DWORD,
    Option<DOWNLOADPROC>,
    *mut c_void,
) -> HSTREAM;
type FnStreamCreateFileUser =
    unsafe extern "system" fn(DWORD, DWORD, *const BASS_FILEPROCS, *mut c_void) -> HSTREAM;
type FnStreamCancel = unsafe extern "system" fn(*mut c_void) -> BOOL;
type FnStreamFree = unsafe extern "system" fn(HSTREAM) -> BOOL;
type FnStreamGetFilePosition = unsafe extern "system" fn(HSTREAM, DWORD) -> QWORD;
type FnStreamPutData = unsafe extern "system" fn(HSTREAM, *const c_void, DWORD) -> DWORD;
type FnStreamPutFileData = unsafe extern "system" fn(HSTREAM, *const c_void, DWORD) -> DWORD;
type FnMusicLoad =
    unsafe extern "system" fn(DWORD, *const c_void, QWORD, DWORD, DWORD, DWORD) -> HMUSIC;
type FnMusicFree = unsafe extern "system" fn(HMUSIC) -> BOOL;
type FnRecordGetDeviceInfo = unsafe extern "system" fn(DWORD, *mut BASS_DEVICEINFO) -> BOOL;
type FnRecordInit = unsafe extern "system" fn(i32) -> BOOL;
type FnRecordFree = unsafe extern "system" fn() -> BOOL;
type FnRecordSetDevice = unsafe extern "system" fn(i32) -> BOOL;
type FnRecordGetDevice = unsafe extern "system" fn() -> DWORD;
type FnRecordGetInfo = unsafe extern "system" fn(*mut c_void) -> BOOL;
type FnRecordGetInputName = unsafe extern "system" fn(i32) -> *const c_char;
type FnRecordSetInput = unsafe extern "system" fn(i32, DWORD, f32) -> BOOL;
type FnRecordGetInput = unsafe extern "system" fn(i32, *mut f32) -> DWORD;
type FnRecordStart =
    unsafe extern "system" fn(DWORD, DWORD, DWORD, Option<RECORDPROC>, *mut c_void) -> HRECORD;
type FnChannelBytes2Seconds = unsafe extern "system" fn(DWORD, QWORD) -> f64;
type FnChannelSeconds2Bytes = unsafe extern "system" fn(DWORD, f64) -> QWORD;
type FnChannelGetDevice = unsafe extern "system" fn(DWORD) -> DWORD;
type FnChannelSetDevice = unsafe extern "system" fn(DWORD, DWORD) -> BOOL;
type FnChannelIsActive = unsafe extern "system" fn(DWORD) -> DWORD;
type FnChannelGetInfo = unsafe extern "system" fn(DWORD, *mut BASS_CHANNELINFO) -> BOOL;
type FnChannelGetTags = unsafe extern "system" fn(DWORD, DWORD) -> *const c_char;
type FnChannelFlags = unsafe extern "system" fn(DWORD, DWORD, DWORD) -> DWORD;
type FnChannelLock = unsafe extern "system" fn(DWORD, BOOL) -> BOOL;
type FnChannelRef = unsafe extern "system" fn(DWORD, BOOL) -> BOOL;
type FnChannelFree = unsafe extern "system" fn(DWORD) -> BOOL;
type FnChannelPlay = unsafe extern "system" fn(DWORD, BOOL) -> BOOL;
type FnChannelStart = unsafe extern "system" fn(DWORD) -> BOOL;
type FnChannelStop = unsafe extern "system" fn(DWORD) -> BOOL;
type FnChannelPause = unsafe extern "system" fn(DWORD) -> BOOL;
type FnChannelUpdate = unsafe extern "system" fn(DWORD, DWORD) -> BOOL;
type FnChannelSetAttribute = unsafe extern "system" fn(DWORD, DWORD, f32) -> BOOL;
type FnChannelGetAttribute = unsafe extern "system" fn(DWORD, DWORD, *mut f32) -> BOOL;
type FnChannelSlideAttribute = unsafe extern "system" fn(DWORD, DWORD, f32, DWORD) -> BOOL;
type FnChannelIsSliding = unsafe extern "system" fn(DWORD, DWORD) -> BOOL;
type FnChannelSetAttributeEx = unsafe extern "system" fn(DWORD, DWORD, *mut c_void, DWORD) -> BOOL;
type FnChannelGetAttributeEx = unsafe extern "system" fn(DWORD, DWORD, *mut c_void, DWORD) -> DWORD;
type FnChannelGetLength = unsafe extern "system" fn(DWORD, DWORD) -> QWORD;
type FnChannelSetPosition = unsafe extern "system" fn(DWORD, QWORD, DWORD) -> BOOL;
type FnChannelGetPosition = unsafe extern "system" fn(DWORD, DWORD) -> QWORD;
type FnChannelGetLevel = unsafe extern "system" fn(DWORD) -> DWORD;
type FnChannelGetLevelEx = unsafe extern "system" fn(DWORD, *mut f32, f32, DWORD) -> BOOL;
type FnChannelGetData = unsafe extern "system" fn(DWORD, *mut c_void, DWORD) -> DWORD;
type FnChannelSetSync =
    unsafe extern "system" fn(DWORD, DWORD, QWORD, Option<SYNCPROC>, *mut c_void) -> HSYNC;
type FnChannelRemoveSync = unsafe extern "system" fn(DWORD, HSYNC) -> BOOL;
type FnChannelSetLink = unsafe extern "system" fn(DWORD, DWORD) -> BOOL;
type FnChannelRemoveLink = unsafe extern "system" fn(DWORD, DWORD) -> BOOL;
type FnChannelSetDsp = unsafe extern "system" fn(DWORD, Option<DSPPROC>, *mut c_void, i32) -> HDSP;
type FnChannelSetDspEx =
    unsafe extern "system" fn(DWORD, Option<DSPPROC>, *mut c_void, i32, DWORD) -> HDSP;
type FnChannelRemoveDsp = unsafe extern "system" fn(DWORD, HDSP) -> BOOL;
type FnChannelSetFx = unsafe extern "system" fn(DWORD, DWORD, i32) -> HFX;
type FnChannelRemoveFx = unsafe extern "system" fn(DWORD, HFX) -> BOOL;
type FnFxSetParameters = unsafe extern "system" fn(HFX, *const c_void) -> BOOL;
type FnFxGetParameters = unsafe extern "system" fn(HFX, *mut c_void) -> BOOL;
type FnFxSetPriority = unsafe extern "system" fn(DWORD, i32) -> BOOL;
type FnFxSetBypass = unsafe extern "system" fn(DWORD, BOOL) -> BOOL;
type FnFxReset = unsafe extern "system" fn(DWORD) -> BOOL;
type FnFxFree = unsafe extern "system" fn(DWORD) -> BOOL;

macro_rules! load_symbol {
    ($lib:expr, $path:expr, $name:ident, $ty:ty) => {{
        let bytes = concat!(stringify!($name), "\0").as_bytes();
        // SAFETY: the symbol is resolved from the official BASS DLL and the
        // function type mirrors bass.h exactly.
        unsafe {
            *$lib
                .get::<$ty>(bytes)
                .map_err(|_error| crate::BassError::MissingSymbol {
                    library: $path.clone(),
                    symbol: stringify!($name).into(),
                })?
        }
    }};
}

#[allow(dead_code)]
pub struct BassApi {
    pub(crate) library_path: PathBuf,
    pub(crate) library: Library,
    pub(crate) set_config: FnSetConfig,
    pub(crate) get_config: FnGetConfig,
    pub(crate) set_config_ptr: FnSetConfigPtr,
    pub(crate) get_config_ptr: FnGetConfigPtr,
    pub(crate) get_version: FnGetVersion,
    pub(crate) error_get_code: FnErrorGetCode,
    pub(crate) get_device_info: FnGetDeviceInfo,
    pub(crate) init: FnInit,
    pub(crate) free: FnFree,
    pub(crate) set_device: FnSetDevice,
    pub(crate) get_device: FnGetDevice,
    pub(crate) get_info: FnGetInfo,
    pub(crate) start: FnStart,
    pub(crate) stop: FnStop,
    pub(crate) pause: FnPause,
    pub(crate) is_started: FnIsStarted,
    pub(crate) update: FnUpdate,
    pub(crate) get_cpu: FnGetCpu,
    pub(crate) set_volume: FnSetVolume,
    pub(crate) get_volume: FnGetVolume,
    pub(crate) plugin_load: FnPluginLoad,
    pub(crate) plugin_free: FnPluginFree,
    pub(crate) plugin_enable: FnPluginEnable,
    pub(crate) plugin_get_info: FnPluginGetInfo,
    pub(crate) sample_load: FnSampleLoad,
    pub(crate) sample_create: FnSampleCreate,
    pub(crate) sample_free: FnSampleFree,
    pub(crate) sample_set_data: FnSampleSetData,
    pub(crate) sample_get_data: FnSampleGetData,
    pub(crate) sample_get_info: FnSampleGetInfo,
    pub(crate) sample_set_info: FnSampleSetInfo,
    pub(crate) sample_get_channel: FnSampleGetChannel,
    pub(crate) sample_get_channels: FnSampleGetChannels,
    pub(crate) sample_stop: FnSampleStop,
    pub(crate) stream_create: FnStreamCreate,
    pub(crate) stream_create_file: FnStreamCreateFile,
    pub(crate) stream_create_url: FnStreamCreateUrl,
    pub(crate) stream_create_file_user: FnStreamCreateFileUser,
    pub(crate) stream_cancel: FnStreamCancel,
    pub(crate) stream_free: FnStreamFree,
    pub(crate) stream_get_file_position: FnStreamGetFilePosition,
    pub(crate) stream_put_data: FnStreamPutData,
    pub(crate) stream_put_file_data: FnStreamPutFileData,
    pub(crate) music_load: FnMusicLoad,
    pub(crate) music_free: FnMusicFree,
    pub(crate) record_get_device_info: FnRecordGetDeviceInfo,
    pub(crate) record_init: FnRecordInit,
    pub(crate) record_free: FnRecordFree,
    pub(crate) record_set_device: FnRecordSetDevice,
    pub(crate) record_get_device: FnRecordGetDevice,
    pub(crate) record_get_info: FnRecordGetInfo,
    pub(crate) record_get_input_name: FnRecordGetInputName,
    pub(crate) record_set_input: FnRecordSetInput,
    pub(crate) record_get_input: FnRecordGetInput,
    pub(crate) record_start: FnRecordStart,
    pub(crate) channel_bytes2seconds: FnChannelBytes2Seconds,
    pub(crate) channel_seconds2bytes: FnChannelSeconds2Bytes,
    pub(crate) channel_get_device: FnChannelGetDevice,
    pub(crate) channel_set_device: FnChannelSetDevice,
    pub(crate) channel_is_active: FnChannelIsActive,
    pub(crate) channel_get_info: FnChannelGetInfo,
    pub(crate) channel_get_tags: FnChannelGetTags,
    pub(crate) channel_flags: FnChannelFlags,
    pub(crate) channel_lock: FnChannelLock,
    pub(crate) channel_ref: FnChannelRef,
    pub(crate) channel_free: FnChannelFree,
    pub(crate) channel_play: FnChannelPlay,
    pub(crate) channel_start: FnChannelStart,
    pub(crate) channel_stop: FnChannelStop,
    pub(crate) channel_pause: FnChannelPause,
    pub(crate) channel_update: FnChannelUpdate,
    pub(crate) channel_set_attribute: FnChannelSetAttribute,
    pub(crate) channel_get_attribute: FnChannelGetAttribute,
    pub(crate) channel_slide_attribute: FnChannelSlideAttribute,
    pub(crate) channel_is_sliding: FnChannelIsSliding,
    pub(crate) channel_set_attribute_ex: FnChannelSetAttributeEx,
    pub(crate) channel_get_attribute_ex: FnChannelGetAttributeEx,
    pub(crate) channel_get_length: FnChannelGetLength,
    pub(crate) channel_set_position: FnChannelSetPosition,
    pub(crate) channel_get_position: FnChannelGetPosition,
    pub(crate) channel_get_level: FnChannelGetLevel,
    pub(crate) channel_get_level_ex: FnChannelGetLevelEx,
    pub(crate) channel_get_data: FnChannelGetData,
    pub(crate) channel_set_sync: FnChannelSetSync,
    pub(crate) channel_remove_sync: FnChannelRemoveSync,
    pub(crate) channel_set_link: FnChannelSetLink,
    pub(crate) channel_remove_link: FnChannelRemoveLink,
    pub(crate) channel_set_dsp: FnChannelSetDsp,
    pub(crate) channel_set_dsp_ex: FnChannelSetDspEx,
    pub(crate) channel_remove_dsp: FnChannelRemoveDsp,
    pub(crate) channel_set_fx: FnChannelSetFx,
    pub(crate) channel_remove_fx: FnChannelRemoveFx,
    pub(crate) fx_set_parameters: FnFxSetParameters,
    pub(crate) fx_get_parameters: FnFxGetParameters,
    pub(crate) fx_set_priority: FnFxSetPriority,
    pub(crate) fx_set_bypass: FnFxSetBypass,
    pub(crate) fx_reset: FnFxReset,
    pub(crate) fx_free: FnFxFree,
}

impl BassApi {
    pub(crate) fn load(path: PathBuf) -> crate::Result<Self> {
        // SAFETY: loading the user-selected official native library is the
        // explicit purpose of this module.  All calls are isolated behind the
        // function table and safe wrappers.
        let library =
            unsafe { Library::new(&path) }.map_err(|error| crate::BassError::LibraryLoad {
                library: path.clone(),
                message: error.to_string(),
            })?;
        Ok(Self {
            set_config: load_symbol!(library, path, BASS_SetConfig, FnSetConfig),
            get_config: load_symbol!(library, path, BASS_GetConfig, FnGetConfig),
            set_config_ptr: load_symbol!(library, path, BASS_SetConfigPtr, FnSetConfigPtr),
            get_config_ptr: load_symbol!(library, path, BASS_GetConfigPtr, FnGetConfigPtr),
            get_version: load_symbol!(library, path, BASS_GetVersion, FnGetVersion),
            error_get_code: load_symbol!(library, path, BASS_ErrorGetCode, FnErrorGetCode),
            get_device_info: load_symbol!(library, path, BASS_GetDeviceInfo, FnGetDeviceInfo),
            init: load_symbol!(library, path, BASS_Init, FnInit),
            free: load_symbol!(library, path, BASS_Free, FnFree),
            set_device: load_symbol!(library, path, BASS_SetDevice, FnSetDevice),
            get_device: load_symbol!(library, path, BASS_GetDevice, FnGetDevice),
            get_info: load_symbol!(library, path, BASS_GetInfo, FnGetInfo),
            start: load_symbol!(library, path, BASS_Start, FnStart),
            stop: load_symbol!(library, path, BASS_Stop, FnStop),
            pause: load_symbol!(library, path, BASS_Pause, FnPause),
            is_started: load_symbol!(library, path, BASS_IsStarted, FnIsStarted),
            update: load_symbol!(library, path, BASS_Update, FnUpdate),
            get_cpu: load_symbol!(library, path, BASS_GetCPU, FnGetCpu),
            set_volume: load_symbol!(library, path, BASS_SetVolume, FnSetVolume),
            get_volume: load_symbol!(library, path, BASS_GetVolume, FnGetVolume),
            plugin_load: load_symbol!(library, path, BASS_PluginLoad, FnPluginLoad),
            plugin_free: load_symbol!(library, path, BASS_PluginFree, FnPluginFree),
            plugin_enable: load_symbol!(library, path, BASS_PluginEnable, FnPluginEnable),
            plugin_get_info: load_symbol!(library, path, BASS_PluginGetInfo, FnPluginGetInfo),
            sample_load: load_symbol!(library, path, BASS_SampleLoad, FnSampleLoad),
            sample_create: load_symbol!(library, path, BASS_SampleCreate, FnSampleCreate),
            sample_free: load_symbol!(library, path, BASS_SampleFree, FnSampleFree),
            sample_set_data: load_symbol!(library, path, BASS_SampleSetData, FnSampleSetData),
            sample_get_data: load_symbol!(library, path, BASS_SampleGetData, FnSampleGetData),
            sample_get_info: load_symbol!(library, path, BASS_SampleGetInfo, FnSampleGetInfo),
            sample_set_info: load_symbol!(library, path, BASS_SampleSetInfo, FnSampleSetInfo),
            sample_get_channel: load_symbol!(
                library,
                path,
                BASS_SampleGetChannel,
                FnSampleGetChannel
            ),
            sample_get_channels: load_symbol!(
                library,
                path,
                BASS_SampleGetChannels,
                FnSampleGetChannels
            ),
            sample_stop: load_symbol!(library, path, BASS_SampleStop, FnSampleStop),
            stream_create: load_symbol!(library, path, BASS_StreamCreate, FnStreamCreate),
            stream_create_file: load_symbol!(
                library,
                path,
                BASS_StreamCreateFile,
                FnStreamCreateFile
            ),
            stream_create_url: load_symbol!(library, path, BASS_StreamCreateURL, FnStreamCreateUrl),
            stream_create_file_user: load_symbol!(
                library,
                path,
                BASS_StreamCreateFileUser,
                FnStreamCreateFileUser
            ),
            stream_cancel: load_symbol!(library, path, BASS_StreamCancel, FnStreamCancel),
            stream_free: load_symbol!(library, path, BASS_StreamFree, FnStreamFree),
            stream_get_file_position: load_symbol!(
                library,
                path,
                BASS_StreamGetFilePosition,
                FnStreamGetFilePosition
            ),
            stream_put_data: load_symbol!(library, path, BASS_StreamPutData, FnStreamPutData),
            stream_put_file_data: load_symbol!(
                library,
                path,
                BASS_StreamPutFileData,
                FnStreamPutFileData
            ),
            music_load: load_symbol!(library, path, BASS_MusicLoad, FnMusicLoad),
            music_free: load_symbol!(library, path, BASS_MusicFree, FnMusicFree),
            record_get_device_info: load_symbol!(
                library,
                path,
                BASS_RecordGetDeviceInfo,
                FnRecordGetDeviceInfo
            ),
            record_init: load_symbol!(library, path, BASS_RecordInit, FnRecordInit),
            record_free: load_symbol!(library, path, BASS_RecordFree, FnRecordFree),
            record_set_device: load_symbol!(library, path, BASS_RecordSetDevice, FnRecordSetDevice),
            record_get_device: load_symbol!(library, path, BASS_RecordGetDevice, FnRecordGetDevice),
            record_get_info: load_symbol!(library, path, BASS_RecordGetInfo, FnRecordGetInfo),
            record_get_input_name: load_symbol!(
                library,
                path,
                BASS_RecordGetInputName,
                FnRecordGetInputName
            ),
            record_set_input: load_symbol!(library, path, BASS_RecordSetInput, FnRecordSetInput),
            record_get_input: load_symbol!(library, path, BASS_RecordGetInput, FnRecordGetInput),
            record_start: load_symbol!(library, path, BASS_RecordStart, FnRecordStart),
            channel_bytes2seconds: load_symbol!(
                library,
                path,
                BASS_ChannelBytes2Seconds,
                FnChannelBytes2Seconds
            ),
            channel_seconds2bytes: load_symbol!(
                library,
                path,
                BASS_ChannelSeconds2Bytes,
                FnChannelSeconds2Bytes
            ),
            channel_get_device: load_symbol!(
                library,
                path,
                BASS_ChannelGetDevice,
                FnChannelGetDevice
            ),
            channel_set_device: load_symbol!(
                library,
                path,
                BASS_ChannelSetDevice,
                FnChannelSetDevice
            ),
            channel_is_active: load_symbol!(library, path, BASS_ChannelIsActive, FnChannelIsActive),
            channel_get_info: load_symbol!(library, path, BASS_ChannelGetInfo, FnChannelGetInfo),
            channel_get_tags: load_symbol!(library, path, BASS_ChannelGetTags, FnChannelGetTags),
            channel_flags: load_symbol!(library, path, BASS_ChannelFlags, FnChannelFlags),
            channel_lock: load_symbol!(library, path, BASS_ChannelLock, FnChannelLock),
            channel_ref: load_symbol!(library, path, BASS_ChannelRef, FnChannelRef),
            channel_free: load_symbol!(library, path, BASS_ChannelFree, FnChannelFree),
            channel_play: load_symbol!(library, path, BASS_ChannelPlay, FnChannelPlay),
            channel_start: load_symbol!(library, path, BASS_ChannelStart, FnChannelStart),
            channel_stop: load_symbol!(library, path, BASS_ChannelStop, FnChannelStop),
            channel_pause: load_symbol!(library, path, BASS_ChannelPause, FnChannelPause),
            channel_update: load_symbol!(library, path, BASS_ChannelUpdate, FnChannelUpdate),
            channel_set_attribute: load_symbol!(
                library,
                path,
                BASS_ChannelSetAttribute,
                FnChannelSetAttribute
            ),
            channel_get_attribute: load_symbol!(
                library,
                path,
                BASS_ChannelGetAttribute,
                FnChannelGetAttribute
            ),
            channel_slide_attribute: load_symbol!(
                library,
                path,
                BASS_ChannelSlideAttribute,
                FnChannelSlideAttribute
            ),
            channel_is_sliding: load_symbol!(
                library,
                path,
                BASS_ChannelIsSliding,
                FnChannelIsSliding
            ),
            channel_set_attribute_ex: load_symbol!(
                library,
                path,
                BASS_ChannelSetAttributeEx,
                FnChannelSetAttributeEx
            ),
            channel_get_attribute_ex: load_symbol!(
                library,
                path,
                BASS_ChannelGetAttributeEx,
                FnChannelGetAttributeEx
            ),
            channel_get_length: load_symbol!(
                library,
                path,
                BASS_ChannelGetLength,
                FnChannelGetLength
            ),
            channel_set_position: load_symbol!(
                library,
                path,
                BASS_ChannelSetPosition,
                FnChannelSetPosition
            ),
            channel_get_position: load_symbol!(
                library,
                path,
                BASS_ChannelGetPosition,
                FnChannelGetPosition
            ),
            channel_get_level: load_symbol!(library, path, BASS_ChannelGetLevel, FnChannelGetLevel),
            channel_get_level_ex: load_symbol!(
                library,
                path,
                BASS_ChannelGetLevelEx,
                FnChannelGetLevelEx
            ),
            channel_get_data: load_symbol!(library, path, BASS_ChannelGetData, FnChannelGetData),
            channel_set_sync: load_symbol!(library, path, BASS_ChannelSetSync, FnChannelSetSync),
            channel_remove_sync: load_symbol!(
                library,
                path,
                BASS_ChannelRemoveSync,
                FnChannelRemoveSync
            ),
            channel_set_link: load_symbol!(library, path, BASS_ChannelSetLink, FnChannelSetLink),
            channel_remove_link: load_symbol!(
                library,
                path,
                BASS_ChannelRemoveLink,
                FnChannelRemoveLink
            ),
            channel_set_dsp: load_symbol!(library, path, BASS_ChannelSetDSP, FnChannelSetDsp),
            channel_set_dsp_ex: load_symbol!(
                library,
                path,
                BASS_ChannelSetDSPEx,
                FnChannelSetDspEx
            ),
            channel_remove_dsp: load_symbol!(
                library,
                path,
                BASS_ChannelRemoveDSP,
                FnChannelRemoveDsp
            ),
            channel_set_fx: load_symbol!(library, path, BASS_ChannelSetFX, FnChannelSetFx),
            channel_remove_fx: load_symbol!(library, path, BASS_ChannelRemoveFX, FnChannelRemoveFx),
            fx_set_parameters: load_symbol!(library, path, BASS_FXSetParameters, FnFxSetParameters),
            fx_get_parameters: load_symbol!(library, path, BASS_FXGetParameters, FnFxGetParameters),
            fx_set_priority: load_symbol!(library, path, BASS_FXSetPriority, FnFxSetPriority),
            fx_set_bypass: load_symbol!(library, path, BASS_FXSetBypass, FnFxSetBypass),
            fx_reset: load_symbol!(library, path, BASS_FXReset, FnFxReset),
            fx_free: load_symbol!(library, path, BASS_FXFree, FnFxFree),
            library_path: path,
            library,
        })
    }

    pub(crate) fn error(&self) -> i32 {
        // SAFETY: function pointer was loaded from the live library.
        unsafe { (self.error_get_code)() }
    }
}

type FnFxGetVersion = unsafe extern "system" fn() -> DWORD;
type FnFxTempoCreate = unsafe extern "system" fn(DWORD, DWORD) -> HSTREAM;
type FnFxTempoGetSource = unsafe extern "system" fn(HSTREAM) -> DWORD;
type FnFxTempoGetRateRatio = unsafe extern "system" fn(HSTREAM) -> f32;
type FnFxReverseCreate = unsafe extern "system" fn(DWORD, f32, DWORD) -> HSTREAM;
type FnFxReverseGetSource = unsafe extern "system" fn(HSTREAM) -> DWORD;
type FnFxBpmDecodeGet = unsafe extern "system" fn(
    DWORD,
    f64,
    f64,
    DWORD,
    DWORD,
    Option<BPMPROGRESSPROC>,
    *mut c_void,
) -> f32;
type FnFxBpmCallbackSet =
    unsafe extern "system" fn(DWORD, Option<BPMPROC>, f64, DWORD, DWORD, *mut c_void) -> BOOL;
type FnFxBpmCallbackReset = unsafe extern "system" fn(DWORD) -> BOOL;
type FnFxBpmTranslate = unsafe extern "system" fn(DWORD, f32, DWORD) -> f32;
type FnFxBpmFree = unsafe extern "system" fn(DWORD) -> BOOL;
type FnFxBeatCallbackSet =
    unsafe extern "system" fn(DWORD, Option<BPMBEATPROC>, *mut c_void) -> BOOL;
type FnFxBeatCallbackReset = unsafe extern "system" fn(DWORD) -> BOOL;
type FnFxBeatDecodeGet =
    unsafe extern "system" fn(DWORD, f64, f64, DWORD, Option<BPMBEATPROC>, *mut c_void) -> BOOL;
type FnFxBeatSetParameters = unsafe extern "system" fn(DWORD, f32, f32, f32) -> BOOL;
type FnFxBeatGetParameters = unsafe extern "system" fn(DWORD, *mut f32, *mut f32, *mut f32) -> BOOL;
type FnFxBeatFree = unsafe extern "system" fn(DWORD) -> BOOL;

#[allow(dead_code)]
pub struct FxApi {
    pub(crate) library_path: PathBuf,
    pub(crate) library: Library,
    pub(crate) get_version: FnFxGetVersion,
    pub(crate) tempo_create: FnFxTempoCreate,
    pub(crate) tempo_get_source: FnFxTempoGetSource,
    pub(crate) tempo_get_rate_ratio: FnFxTempoGetRateRatio,
    pub(crate) reverse_create: FnFxReverseCreate,
    pub(crate) reverse_get_source: FnFxReverseGetSource,
    pub(crate) bpm_decode_get: FnFxBpmDecodeGet,
    pub(crate) bpm_callback_set: FnFxBpmCallbackSet,
    pub(crate) bpm_callback_reset: FnFxBpmCallbackReset,
    pub(crate) bpm_translate: FnFxBpmTranslate,
    pub(crate) bpm_free: FnFxBpmFree,
    pub(crate) beat_callback_set: FnFxBeatCallbackSet,
    pub(crate) beat_callback_reset: FnFxBeatCallbackReset,
    pub(crate) beat_decode_get: FnFxBeatDecodeGet,
    pub(crate) beat_set_parameters: FnFxBeatSetParameters,
    pub(crate) beat_get_parameters: FnFxBeatGetParameters,
    pub(crate) beat_free: FnFxBeatFree,
}

impl FxApi {
    pub(crate) fn load(path: PathBuf) -> crate::Result<Self> {
        // SAFETY: see BassApi::load.
        let library =
            unsafe { Library::new(&path) }.map_err(|error| crate::BassError::LibraryLoad {
                library: path.clone(),
                message: error.to_string(),
            })?;
        Ok(Self {
            get_version: load_symbol!(library, path, BASS_FX_GetVersion, FnFxGetVersion),
            tempo_create: load_symbol!(library, path, BASS_FX_TempoCreate, FnFxTempoCreate),
            tempo_get_source: load_symbol!(
                library,
                path,
                BASS_FX_TempoGetSource,
                FnFxTempoGetSource
            ),
            tempo_get_rate_ratio: load_symbol!(
                library,
                path,
                BASS_FX_TempoGetRateRatio,
                FnFxTempoGetRateRatio
            ),
            reverse_create: load_symbol!(library, path, BASS_FX_ReverseCreate, FnFxReverseCreate),
            reverse_get_source: load_symbol!(
                library,
                path,
                BASS_FX_ReverseGetSource,
                FnFxReverseGetSource
            ),
            bpm_decode_get: load_symbol!(library, path, BASS_FX_BPM_DecodeGet, FnFxBpmDecodeGet),
            bpm_callback_set: load_symbol!(
                library,
                path,
                BASS_FX_BPM_CallbackSet,
                FnFxBpmCallbackSet
            ),
            bpm_callback_reset: load_symbol!(
                library,
                path,
                BASS_FX_BPM_CallbackReset,
                FnFxBpmCallbackReset
            ),
            bpm_translate: load_symbol!(library, path, BASS_FX_BPM_Translate, FnFxBpmTranslate),
            bpm_free: load_symbol!(library, path, BASS_FX_BPM_Free, FnFxBpmFree),
            beat_callback_set: load_symbol!(
                library,
                path,
                BASS_FX_BPM_BeatCallbackSet,
                FnFxBeatCallbackSet
            ),
            beat_callback_reset: load_symbol!(
                library,
                path,
                BASS_FX_BPM_BeatCallbackReset,
                FnFxBeatCallbackReset
            ),
            beat_decode_get: load_symbol!(
                library,
                path,
                BASS_FX_BPM_BeatDecodeGet,
                FnFxBeatDecodeGet
            ),
            beat_set_parameters: load_symbol!(
                library,
                path,
                BASS_FX_BPM_BeatSetParameters,
                FnFxBeatSetParameters
            ),
            beat_get_parameters: load_symbol!(
                library,
                path,
                BASS_FX_BPM_BeatGetParameters,
                FnFxBeatGetParameters
            ),
            beat_free: load_symbol!(library, path, BASS_FX_BPM_BeatFree, FnFxBeatFree),
            library_path: path,
            library,
        })
    }
}

/// A zeroed C structure used by callers of raw functions.
pub fn zeroed<T>() -> T {
    // SAFETY: this helper is intentionally equivalent to MaybeUninit::zeroed
    // and is only useful for C-compatible output structures.
    unsafe { MaybeUninit::zeroed().assume_init() }
}
