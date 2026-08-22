use std::{
    ffi::{CStr, c_void},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::{
    BassError, Result,
    callback::{self, DownloadContext, SyncContext},
    error::{api_error, check_bool},
    path::NativePath,
    raw::{self, BASS_CHANNELINFO, BASS_DEVICEINFO, BASS_INFO, BassApi, DWORD, QWORD},
};

pub(crate) struct EngineInner {
    pub(crate) bass: BassApi,
    pub(crate) fx: Mutex<Option<raw::FxApi>>,
    pub(crate) state: Mutex<EngineState>,
}

#[derive(Debug, Default)]
pub(crate) struct EngineState {
    pub(crate) initialized: bool,
}

impl Drop for EngineInner {
    fn drop(&mut self) {
        if let Ok(mut state) = self.state.lock()
            && state.initialized
        {
            // SAFETY: the function pointer belongs to the live BASS DLL;
            // EngineInner is the last owner of all channels by design.
            unsafe {
                (self.bass.free)();
            }
            state.initialized = false;
        }
    }
}

/// Options controlling optional BASS_FX loading.
#[derive(Debug, Clone, Default)]
pub struct BassEngineOptions {
    pub fx_path: Option<PathBuf>,
    pub require_fx: bool,
}

/// A dynamically loaded BASS instance.
#[derive(Clone)]
pub struct BassEngine {
    pub(crate) inner: Arc<EngineInner>,
}

impl BassEngine {
    /// Load BASS from an explicit native library path.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        Self::load_with_options(path, BassEngineOptions::default())
    }

    /// Load BASS from a directory containing the standard library names.
    ///
    /// The directory must contain `bass.dll` on Windows. If it also contains
    /// `bass_fx.dll`, BASS_FX is loaded automatically. On other platforms the
    /// corresponding native library names are used.
    pub fn load_from_directory(directory: impl AsRef<Path>) -> Result<Self> {
        Self::load_from_directory_with_options(directory, BassEngineOptions::default())
    }

    /// Load BASS from a directory while applying explicit loading options.
    /// An explicit [`BassEngineOptions::fx_path`] takes precedence over the
    /// standard BASS_FX file in the directory.
    pub fn load_from_directory_with_options(
        directory: impl AsRef<Path>,
        options: BassEngineOptions,
    ) -> Result<Self> {
        let directory = directory.as_ref();
        if !directory.is_dir() {
            return Err(BassError::InvalidInput {
                kind: "DLL directory",
                message: format!("{} is not a directory", directory.display()),
            });
        }
        let fx_path = options.fx_path.or_else(|| {
            let path = directory.join(platform_library_name("bass_fx"));
            path.is_file().then_some(path)
        });
        Self::load_with_options(
            directory.join(platform_library_name("bass")),
            BassEngineOptions { fx_path, ..options },
        )
    }

    /// Load BASS from an explicit path and optionally load BASS_FX from the
    /// explicit path in [`BassEngineOptions::fx_path`].
    pub fn load_with_options(path: impl AsRef<Path>, options: BassEngineOptions) -> Result<Self> {
        let bass = BassApi::load(path.as_ref().to_path_buf())?;
        check_api_version((unsafe { (bass.get_version)() } >> 16) as u16)?;

        let fx = match options.fx_path {
            Some(path) => Some(raw::FxApi::load(path)?),
            None if options.require_fx => return Err(BassError::FxUnavailable),
            None => None,
        };
        if let Some(ref fx) = fx {
            check_api_version((unsafe { (fx.get_version)() } >> 16) as u16)?;
        }
        Ok(Self {
            inner: Arc::new(EngineInner {
                bass,
                fx: Mutex::new(fx),
                state: Mutex::new(EngineState::default()),
            }),
        })
    }

    /// Load or replace BASS_FX after the base library has been loaded.
    pub fn load_fx(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        let fx = raw::FxApi::load(path)?;
        check_api_version((unsafe { (fx.get_version)() } >> 16) as u16)?;
        *self.inner.fx.lock().map_err(|_| BassError::InvalidInput {
            kind: "engine state",
            message: "engine lock is poisoned".into(),
        })? = Some(fx);
        Ok(())
    }

    pub fn has_fx(&self) -> bool {
        self.inner.fx.lock().map(|fx| fx.is_some()).unwrap_or(false)
    }

    pub fn fx_library(&self) -> Result<crate::fx::FxLibrary> {
        if self.has_fx() {
            Ok(crate::fx::FxLibrary {
                inner: self.inner.clone(),
            })
        } else {
            Err(BassError::FxUnavailable)
        }
    }

    pub fn bass_version(&self) -> u32 {
        // SAFETY: function pointer is valid for the lifetime of self.inner.
        unsafe { (self.inner.bass.get_version)() }
    }

    pub fn fx_version(&self) -> Option<u32> {
        self.inner
            .fx
            .lock()
            .ok()
            .and_then(|fx| fx.as_ref().map(|fx| unsafe { (fx.get_version)() }))
    }

    /// Enumerate output devices without initializing one.
    pub fn devices(&self) -> Result<Vec<DeviceInfo>> {
        let mut result = Vec::new();
        for index in 0..4096u32 {
            let mut info = BASS_DEVICEINFO::default();
            // SAFETY: info points to valid writable storage and the DLL is live.
            let ok = unsafe { (self.inner.bass.get_device_info)(index, &mut info) };
            if ok == 0 {
                break;
            }
            result.push(DeviceInfo {
                index,
                name: c_string(info.name),
                driver: c_string(info.driver),
                flags: info.flags,
                device_type: DeviceType::from_flags(info.flags),
            });
        }
        Ok(result)
    }

    pub fn device_info(&self, index: u32) -> Result<Option<DeviceInfo>> {
        let mut info = BASS_DEVICEINFO::default();
        // SAFETY: info points to valid writable storage and the DLL is live.
        if unsafe { (self.inner.bass.get_device_info)(index, &mut info) } == 0 {
            return Ok(None);
        }
        Ok(Some(DeviceInfo {
            index,
            name: c_string(info.name),
            driver: c_string(info.driver),
            flags: info.flags,
            device_type: DeviceType::from_flags(info.flags),
        }))
    }

    /// Initialize an output device.  WASAPI is represented by no backend flag;
    /// this is the official BASS behavior on Vista and newer Windows systems.
    pub fn initialize(&self, options: InitOptions) -> Result<()> {
        if options.float_processing {
            self.set_config(raw::BASS_CONFIG_FLOATDSP, 1)?;
        }
        let mut flags = options.backend.flags();
        if options.mono {
            flags |= raw::BASS_DEVICE_MONO;
        }
        if options.exclusive {
            flags |= raw::BASS_DEVICE_HOG;
        }
        if options.force_frequency {
            flags |= raw::BASS_DEVICE_FREQ;
        }
        // SAFETY: null window and GUID are accepted by BASS for normal output.
        let ok = unsafe {
            (self.inner.bass.init)(
                options.device,
                options.sample_rate,
                flags,
                std::ptr::null_mut(),
                std::ptr::null(),
            )
        };
        check_bool("BASS_Init", ok, self.last_error())?;
        self.inner
            .state
            .lock()
            .map_err(|_| BassError::InvalidInput {
                kind: "engine state",
                message: "engine lock is poisoned".into(),
            })?
            .initialized = true;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.inner
            .state
            .lock()
            .map(|state| state.initialized)
            .unwrap_or(false)
    }

    pub fn free(&self) -> Result<()> {
        let ok = unsafe { (self.inner.bass.free)() };
        check_bool("BASS_Free", ok, self.last_error())?;
        self.inner
            .state
            .lock()
            .map_err(|_| BassError::InvalidInput {
                kind: "engine state",
                message: "engine lock is poisoned".into(),
            })?
            .initialized = false;
        Ok(())
    }

    pub fn set_config(&self, option: DWORD, value: DWORD) -> Result<()> {
        let ok = unsafe { (self.inner.bass.set_config)(option, value) };
        check_bool("BASS_SetConfig", ok, self.last_error())
    }

    pub fn get_config(&self, option: DWORD) -> DWORD {
        unsafe { (self.inner.bass.get_config)(option) }
    }

    pub fn set_global_volume(&self, volume: f32) -> Result<()> {
        let ok = unsafe { (self.inner.bass.set_volume)(volume) };
        check_bool("BASS_SetVolume", ok, self.last_error())
    }

    pub fn global_volume(&self) -> f32 {
        unsafe { (self.inner.bass.get_volume)() }
    }

    pub fn cpu_usage(&self) -> f32 {
        unsafe { (self.inner.bass.get_cpu)() }
    }

    pub fn output_info(&self) -> Result<OutputInfo> {
        let mut info = BASS_INFO::default();
        let ok = unsafe { (self.inner.bass.get_info)(&mut info) };
        check_bool("BASS_GetInfo", ok, self.last_error())?;
        Ok(OutputInfo {
            flags: info.flags,
            min_buffer_ms: info.minbuf,
            latency_ms: info.latency,
            sample_rate: info.freq,
            speakers: info.speakers,
            direct_sound_version: info.dsver,
        })
    }

    pub fn start(&self) -> Result<()> {
        self.bool_call("BASS_Start", unsafe { (self.inner.bass.start)() })
    }
    pub fn stop(&self) -> Result<()> {
        self.bool_call("BASS_Stop", unsafe { (self.inner.bass.stop)() })
    }
    pub fn pause(&self) -> Result<()> {
        self.bool_call("BASS_Pause", unsafe { (self.inner.bass.pause)() })
    }
    pub fn is_started(&self) -> bool {
        unsafe { (self.inner.bass.is_started)() != 0 }
    }

    pub fn load_plugin(&self, path: impl AsRef<Path>) -> Result<Plugin> {
        self.load_plugin_with_flags(path, 0)
    }

    pub fn load_plugin_with_flags(&self, path: impl AsRef<Path>, flags: DWORD) -> Result<Plugin> {
        let native = NativePath::new(path, "plugin path")?;
        let flags = native.flags(flags);
        let handle = unsafe { (self.inner.bass.plugin_load)(native.as_ptr(), flags) };
        if handle == 0 {
            return Err(api_error("BASS_PluginLoad", self.last_error()));
        }
        let info = match self.plugin_info(handle) {
            Ok(info) => info,
            Err(error) => {
                unsafe {
                    (self.inner.bass.plugin_free)(handle);
                }
                return Err(error);
            }
        };
        Ok(Plugin {
            inner: self.inner.clone(),
            handle,
            info,
        })
    }

    /// Load plugins from caller-provided native library paths.
    pub fn load_plugins<I, P>(&self, paths: I) -> Result<Vec<Plugin>>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        paths
            .into_iter()
            .map(|path| self.load_plugin(path))
            .collect()
    }

    fn plugin_info(&self, handle: u32) -> Result<PluginInfo> {
        let ptr = unsafe { (self.inner.bass.plugin_get_info)(handle) };
        if ptr.is_null() {
            return Err(api_error("BASS_PluginGetInfo", self.last_error()));
        }
        // SAFETY: BASS returns a valid immutable structure for a live plugin.
        let info = unsafe { &*ptr };
        let mut formats = Vec::new();
        if !info.formats.is_null() {
            for index in 0..info.formatc as usize {
                // SAFETY: formats points to formatc contiguous entries.
                let format = unsafe { &*info.formats.add(index) };
                formats.push(PluginFormat {
                    channel_type: format.ctype,
                    name: c_string(format.name),
                    extensions: c_string(format.exts),
                });
            }
        }
        Ok(PluginInfo {
            version: info.version,
            formats,
        })
    }

    pub fn load_file(&self, path: impl AsRef<Path>, options: SourceOptions) -> Result<Channel> {
        let native = NativePath::new(path, "audio path")?;
        let stream_flags = options.stream_flags();
        let stream = unsafe {
            (self.inner.bass.stream_create_file)(
                raw::BASS_FILE_NAME,
                native.as_ptr().cast(),
                0,
                0,
                native.flags(stream_flags),
            )
        };
        if stream != 0 {
            return Ok(Channel::new(
                self.inner.clone(),
                stream,
                ChannelKind::Stream,
                None,
            ));
        }
        let music_flags = options.music_flags();
        let music = unsafe {
            (self.inner.bass.music_load)(
                raw::BASS_FILE_NAME,
                native.as_ptr().cast(),
                0,
                0,
                native.flags(music_flags),
                options.music_frequency,
            )
        };
        if music != 0 {
            return Ok(Channel::new(
                self.inner.clone(),
                music,
                ChannelKind::Music,
                None,
            ));
        }
        Err(api_error(
            "BASS_StreamCreateFile/BASS_MusicLoad",
            self.last_error(),
        ))
    }

    pub fn load_url(&self, url: &str, options: UrlOptions) -> Result<Channel> {
        let native = NativePath::new(url, "URL")?;
        let callback_context = options.callback.map(DownloadContext::new);
        let native_context = callback_context
            .as_ref()
            .map(|context| Arc::into_raw(context.clone()) as *mut c_void);
        let user = native_context.unwrap_or(std::ptr::null_mut());
        let flags = native.flags(
            options.flags
                | if options.float {
                    raw::BASS_SAMPLE_FLOAT
                } else {
                    0
                },
        );
        let download_proc = callback_context
            .as_ref()
            .map(|_| callback::download_trampoline as raw::DOWNLOADPROC);
        let stream = unsafe {
            (self.inner.bass.stream_create_url)(
                native.as_ptr(),
                options.offset,
                flags,
                download_proc,
                user,
            )
        };
        if stream == 0 {
            if let Some(context) = callback_context.as_ref() {
                context
                    .alive
                    .store(false, std::sync::atomic::Ordering::Release);
            }
            if let Some(user) = native_context {
                unsafe {
                    drop(Arc::from_raw(user as *const DownloadContext));
                }
            }
            return Err(api_error("BASS_StreamCreateURL", self.last_error()));
        }
        Ok(Channel::new_with_user(
            self.inner.clone(),
            stream,
            ChannelKind::Url,
            callback_context,
            native_context,
        ))
    }

    pub(crate) fn last_error(&self) -> i32 {
        self.inner.bass.error()
    }

    pub(crate) fn bool_call(&self, operation: &'static str, value: i32) -> Result<()> {
        check_bool(operation, value, self.last_error())
    }
}

fn platform_library_name(stem: &str) -> String {
    #[cfg(windows)]
    {
        format!("{stem}.dll")
    }
    #[cfg(target_os = "macos")]
    {
        format!("lib{stem}.dylib")
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        format!("lib{stem}.so")
    }
    #[cfg(not(any(windows, unix)))]
    {
        format!("{stem}.dll")
    }
}

fn check_api_version(actual: u16) -> Result<()> {
    if actual == crate::BASS_API_VERSION {
        Ok(())
    } else {
        Err(BassError::VersionMismatch {
            expected: crate::BASS_API_VERSION,
            actual,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OutputBackend {
    #[default]
    Wasapi,
    DirectSound,
}
impl OutputBackend {
    pub(crate) fn flags(self) -> DWORD {
        match self {
            Self::Wasapi => 0,
            Self::DirectSound => raw::BASS_DEVICE_DSOUND,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InitOptions {
    pub device: i32,
    pub sample_rate: u32,
    pub backend: OutputBackend,
    pub mono: bool,
    pub exclusive: bool,
    pub force_frequency: bool,
    pub float_processing: bool,
}

impl Default for InitOptions {
    fn default() -> Self {
        Self {
            device: -1,
            sample_rate: 44100,
            backend: OutputBackend::default(),
            mono: false,
            exclusive: false,
            force_frequency: false,
            float_processing: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputInfo {
    pub flags: DWORD,
    pub min_buffer_ms: u32,
    pub latency_ms: u32,
    pub sample_rate: u32,
    pub speakers: u32,
    pub direct_sound_version: u32,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub index: u32,
    pub name: String,
    pub driver: String,
    pub flags: DWORD,
    pub device_type: DeviceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Network,
    Speakers,
    Line,
    Headphones,
    Microphone,
    Headset,
    Handset,
    Digital,
    Spdif,
    Hdmi,
    DisplayPort,
    Unknown,
}
impl DeviceType {
    fn from_flags(flags: DWORD) -> Self {
        match flags & raw::BASS_DEVICE_TYPE_MASK {
            raw::BASS_DEVICE_TYPE_NETWORK => Self::Network,
            raw::BASS_DEVICE_TYPE_SPEAKERS => Self::Speakers,
            raw::BASS_DEVICE_TYPE_LINE => Self::Line,
            raw::BASS_DEVICE_TYPE_HEADPHONES => Self::Headphones,
            raw::BASS_DEVICE_TYPE_MICROPHONE => Self::Microphone,
            raw::BASS_DEVICE_TYPE_HEADSET => Self::Headset,
            raw::BASS_DEVICE_TYPE_HANDSET => Self::Handset,
            raw::BASS_DEVICE_TYPE_DIGITAL => Self::Digital,
            raw::BASS_DEVICE_TYPE_SPDIF => Self::Spdif,
            raw::BASS_DEVICE_TYPE_HDMI => Self::Hdmi,
            raw::BASS_DEVICE_TYPE_DISPLAYPORT => Self::DisplayPort,
            _ => Self::Unknown,
        }
    }
}

impl DeviceInfo {
    pub fn is_default(&self) -> bool {
        self.flags & raw::BASS_DEVICE_DEFAULT != 0
    }
    pub fn is_enabled(&self) -> bool {
        self.flags & raw::BASS_DEVICE_ENABLED != 0
    }
    pub fn is_initialized(&self) -> bool {
        self.flags & raw::BASS_DEVICE_INIT != 0
    }
    pub fn is_loopback(&self) -> bool {
        self.flags & raw::BASS_DEVICE_LOOPBACK != 0
    }
}

#[derive(Debug, Clone)]
pub struct SourceOptions {
    pub float: bool,
    pub mono: bool,
    pub looped: bool,
    pub decode_only: bool,
    pub prescan: bool,
    pub stream_flags: DWORD,
    pub music_flags: DWORD,
    pub music_frequency: DWORD,
}

impl Default for SourceOptions {
    fn default() -> Self {
        Self {
            float: true,
            mono: false,
            looped: false,
            decode_only: false,
            prescan: true,
            stream_flags: 0,
            music_flags: raw::BASS_MUSIC_RAMP,
            music_frequency: 0,
        }
    }
}
impl SourceOptions {
    fn stream_flags(&self) -> DWORD {
        self.stream_flags
            | if self.float {
                raw::BASS_SAMPLE_FLOAT
            } else {
                0
            }
            | if self.mono { raw::BASS_SAMPLE_MONO } else { 0 }
            | if self.looped {
                raw::BASS_SAMPLE_LOOP
            } else {
                0
            }
            | if self.decode_only {
                raw::BASS_STREAM_DECODE
            } else {
                0
            }
            | if self.prescan {
                raw::BASS_STREAM_PRESCAN
            } else {
                0
            }
    }
    fn music_flags(&self) -> DWORD {
        self.music_flags
            | if self.float { raw::BASS_MUSIC_FLOAT } else { 0 }
            | if self.mono { raw::BASS_MUSIC_MONO } else { 0 }
            | if self.looped { raw::BASS_MUSIC_LOOP } else { 0 }
            | if self.decode_only {
                raw::BASS_MUSIC_DECODE
            } else {
                0
            }
            | if self.prescan {
                raw::BASS_MUSIC_PRESCAN
            } else {
                0
            }
    }
}

pub type DownloadCallback = Box<dyn FnMut(DownloadEvent) + Send + 'static>;
#[derive(Debug, Clone)]
pub enum DownloadEvent {
    Data { length: usize },
    Status(String),
    Finished,
}

pub struct UrlOptions {
    pub offset: DWORD,
    pub float: bool,
    pub flags: DWORD,
    pub callback: Option<DownloadCallback>,
}
impl Default for UrlOptions {
    fn default() -> Self {
        Self {
            offset: 0,
            float: true,
            flags: raw::BASS_STREAM_BLOCK | raw::BASS_STREAM_STATUS,
            callback: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelKind {
    Stream,
    Music,
    Url,
    Derived,
}

pub struct Channel {
    pub(crate) inner: Arc<EngineInner>,
    pub(crate) handle: u32,
    pub(crate) kind: ChannelKind,
    pub(crate) download_context: Option<Arc<DownloadContext>>,
    pub(crate) download_user: Option<*mut c_void>,
    pub(crate) progress: Mutex<ProgressState>,
}

#[derive(Debug)]
pub(crate) struct ProgressState {
    pub(crate) last_download: Option<(QWORD, Instant)>,
}

impl Channel {
    pub(crate) fn new(
        inner: Arc<EngineInner>,
        handle: u32,
        kind: ChannelKind,
        download_context: Option<Arc<DownloadContext>>,
    ) -> Self {
        Self::new_with_user(inner, handle, kind, download_context, None)
    }
    pub(crate) fn new_with_user(
        inner: Arc<EngineInner>,
        handle: u32,
        kind: ChannelKind,
        download_context: Option<Arc<DownloadContext>>,
        download_user: Option<*mut c_void>,
    ) -> Self {
        Self {
            inner,
            handle,
            kind,
            download_context,
            download_user,
            progress: Mutex::new(ProgressState {
                last_download: None,
            }),
        }
    }
    pub fn raw_handle(&self) -> u32 {
        self.handle
    }
    pub fn kind(&self) -> ChannelKind {
        self.kind
    }
    pub fn play(&self, restart: bool) -> Result<()> {
        self.bool_call("BASS_ChannelPlay", unsafe {
            (self.inner.bass.channel_play)(self.handle, restart as i32)
        })
    }
    pub fn pause(&self) -> Result<()> {
        self.bool_call("BASS_ChannelPause", unsafe {
            (self.inner.bass.channel_pause)(self.handle)
        })
    }
    pub fn stop(&self) -> Result<()> {
        self.bool_call("BASS_ChannelStop", unsafe {
            (self.inner.bass.channel_stop)(self.handle)
        })
    }
    pub fn active_state(&self) -> ActiveState {
        match unsafe { (self.inner.bass.channel_is_active)(self.handle) } {
            raw::BASS_ACTIVE_PLAYING => ActiveState::Playing,
            raw::BASS_ACTIVE_STALLED => ActiveState::Stalled,
            raw::BASS_ACTIVE_PAUSED => ActiveState::Paused,
            raw::BASS_ACTIVE_PAUSED_DEVICE => ActiveState::PausedDevice,
            _ => ActiveState::Stopped,
        }
    }
    pub fn set_volume(&self, volume: f32) -> Result<()> {
        self.set_attribute(raw::BASS_ATTRIB_VOL, volume)
    }
    pub fn volume(&self) -> Result<f32> {
        self.attribute(raw::BASS_ATTRIB_VOL)
    }
    pub fn set_pan(&self, pan: f32) -> Result<()> {
        self.set_attribute(raw::BASS_ATTRIB_PAN, pan)
    }
    pub fn pan(&self) -> Result<f32> {
        self.attribute(raw::BASS_ATTRIB_PAN)
    }
    pub fn set_frequency(&self, frequency: f32) -> Result<()> {
        self.set_attribute(raw::BASS_ATTRIB_FREQ, frequency)
    }
    pub fn frequency(&self) -> Result<f32> {
        self.attribute(raw::BASS_ATTRIB_FREQ)
    }
    pub fn set_attribute(&self, attribute: DWORD, value: f32) -> Result<()> {
        self.bool_call("BASS_ChannelSetAttribute", unsafe {
            (self.inner.bass.channel_set_attribute)(self.handle, attribute, value)
        })
    }
    pub fn attribute(&self, attribute: DWORD) -> Result<f32> {
        let mut value = 0.0;
        self.bool_call("BASS_ChannelGetAttribute", unsafe {
            (self.inner.bass.channel_get_attribute)(self.handle, attribute, &mut value)
        })?;
        Ok(value)
    }
    pub fn position(&self) -> Result<Duration> {
        let bytes =
            unsafe { (self.inner.bass.channel_get_position)(self.handle, raw::BASS_POS_BYTE) };
        self.bytes_to_duration(bytes)
    }
    pub fn length(&self) -> Result<Option<Duration>> {
        let bytes =
            unsafe { (self.inner.bass.channel_get_length)(self.handle, raw::BASS_POS_BYTE) };
        if bytes == QWORD::MAX {
            Ok(None)
        } else {
            Ok(Some(self.bytes_to_duration(bytes)?))
        }
    }
    pub fn seek(&self, position: Duration) -> Result<()> {
        let bytes =
            unsafe { (self.inner.bass.channel_seconds2bytes)(self.handle, position.as_secs_f64()) };
        self.bool_call("BASS_ChannelSetPosition", unsafe {
            (self.inner.bass.channel_set_position)(self.handle, bytes, raw::BASS_POS_BYTE)
        })
    }
    pub fn seek_bytes(&self, position: QWORD, mode: DWORD) -> Result<()> {
        self.bool_call("BASS_ChannelSetPosition", unsafe {
            (self.inner.bass.channel_set_position)(self.handle, position, mode)
        })
    }
    pub fn set_device(&self, device: u32) -> Result<()> {
        self.bool_call("BASS_ChannelSetDevice", unsafe {
            (self.inner.bass.channel_set_device)(self.handle, device)
        })
    }
    pub fn device(&self) -> u32 {
        unsafe { (self.inner.bass.channel_get_device)(self.handle) }
    }
    pub fn info(&self) -> Result<ChannelInfo> {
        let mut info = BASS_CHANNELINFO::default();
        self.bool_call("BASS_ChannelGetInfo", unsafe {
            (self.inner.bass.channel_get_info)(self.handle, &mut info)
        })?;
        Ok(ChannelInfo {
            frequency: info.freq,
            channels: info.chans,
            flags: info.flags,
            channel_type: info.ctype,
            original_resolution: info.origres,
            plugin: (info.plugin != 0).then_some(info.plugin),
            filename: c_string_opt(info.filename),
        })
    }
    pub fn get_level(&self) -> u32 {
        unsafe { (self.inner.bass.channel_get_level)(self.handle) }
    }
    pub fn get_level_ex(&self, seconds: f32, flags: DWORD) -> Result<Vec<f32>> {
        let channels = self.info()?.channels.max(1) as usize;
        let mut values = vec![0.0; channels];
        self.bool_call("BASS_ChannelGetLevelEx", unsafe {
            (self.inner.bass.channel_get_level_ex)(self.handle, values.as_mut_ptr(), seconds, flags)
        })?;
        Ok(values)
    }
    pub fn read_data(&self, bytes: usize, flags: DWORD) -> Result<Vec<u8>> {
        let mut data = vec![0u8; bytes];
        let read = unsafe {
            (self.inner.bass.channel_get_data)(
                self.handle,
                data.as_mut_ptr().cast(),
                bytes as DWORD | flags,
            )
        };
        if read == DWORD::MAX {
            return Err(api_error("BASS_ChannelGetData", self.last_error()));
        }
        data.truncate(read as usize);
        Ok(data)
    }
    pub fn read_float_data(&self, samples: usize, flags: DWORD) -> Result<Vec<f32>> {
        let bytes = self.read_data(
            samples * std::mem::size_of::<f32>(),
            flags | raw::BASS_DATA_FLOAT,
        )?;
        let mut result = Vec::with_capacity(bytes.len() / 4);
        for chunk in bytes.chunks_exact(4) {
            result.push(f32::from_ne_bytes(chunk.try_into().unwrap()));
        }
        Ok(result)
    }
    pub fn read_fft_data(&self, fft_size: usize, flags: DWORD) -> Result<Vec<f32>> {
        let fft_flag = match fft_size {
            256 => raw::BASS_DATA_FFT256,
            512 => raw::BASS_DATA_FFT512,
            1024 => raw::BASS_DATA_FFT1024,
            2048 => raw::BASS_DATA_FFT2048,
            4096 => raw::BASS_DATA_FFT4096,
            8192 => raw::BASS_DATA_FFT8192,
            _ => {
                return Err(BassError::InvalidInput {
                    kind: "FFT size",
                    message: "expected one of 256, 512, 1024, 2048, 4096, or 8192".into(),
                });
            }
        };
        let mut data = vec![0.0f32; fft_size / 2 + 1];
        let read = unsafe {
            (self.inner.bass.channel_get_data)(
                self.handle,
                data.as_mut_ptr().cast(),
                fft_flag | flags,
            )
        };
        if read == DWORD::MAX {
            return Err(api_error("BASS_ChannelGetData", self.last_error()));
        }
        data.truncate((read as usize / std::mem::size_of::<f32>()).min(data.len()));
        Ok(data)
    }
    pub fn tags(&self, tag: TagKind) -> Vec<String> {
        let ptr = unsafe { (self.inner.bass.channel_get_tags)(self.handle, tag.raw()) };
        parse_tag_strings(ptr)
    }
    pub fn remote_progress(&self) -> Result<RemoteProgress> {
        let downloaded = file_position(&self.inner, self.handle, raw::BASS_FILEPOS_DOWNLOAD);
        let buffered = file_position(&self.inner, self.handle, raw::BASS_FILEPOS_BUFFER);
        let available = file_position(&self.inner, self.handle, raw::BASS_FILEPOS_AVAILABLE);
        let buffering = file_position(&self.inner, self.handle, raw::BASS_FILEPOS_BUFFERING)
            .map(|v| 100u8.saturating_sub(v.min(100) as u8));
        let now = Instant::now();
        let speed = if let (Some(current), Ok(mut state)) = (downloaded, self.progress.lock()) {
            let value = state.last_download.and_then(|(old, when)| {
                let seconds = now.duration_since(when).as_secs_f64();
                if seconds > 0.0 && current >= old {
                    Some((current - old) as f64 / seconds)
                } else {
                    None
                }
            });
            state.last_download = Some((current, now));
            value
        } else {
            None
        };
        Ok(RemoteProgress {
            state: self.active_state(),
            buffering_percent: buffering,
            downloaded_bytes: downloaded,
            buffered_bytes: buffered,
            available_bytes: available,
            bytes_per_second: speed,
        })
    }
    pub fn set_sync(
        &self,
        kind: SyncKind,
        callback: Box<dyn FnMut(SyncEvent) + Send + 'static>,
    ) -> Result<SyncRegistration> {
        let context = SyncContext::new(callback);
        let user = Arc::as_ptr(&context) as *mut c_void;
        let (sync_type, parameter) = kind.raw();
        let handle = unsafe {
            (self.inner.bass.channel_set_sync)(
                self.handle,
                sync_type,
                parameter,
                Some(callback::sync_trampoline),
                user,
            )
        };
        if handle == 0 {
            context
                .alive
                .store(false, std::sync::atomic::Ordering::Release);
            return Err(api_error("BASS_ChannelSetSync", self.last_error()));
        }
        Ok(SyncRegistration {
            inner: self.inner.clone(),
            channel: self.handle,
            handle,
            context,
        })
    }
    fn bytes_to_duration(&self, bytes: QWORD) -> Result<Duration> {
        let seconds = unsafe { (self.inner.bass.channel_bytes2seconds)(self.handle, bytes) };
        if !seconds.is_finite() || seconds < 0.0 {
            Err(api_error("BASS_ChannelBytes2Seconds", self.last_error()))
        } else {
            Ok(Duration::from_secs_f64(seconds))
        }
    }
    fn bool_call(&self, operation: &'static str, value: i32) -> Result<()> {
        check_bool(operation, value, self.last_error())
    }
    fn last_error(&self) -> i32 {
        unsafe { (self.inner.bass.error_get_code)() }
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        if let Some(context) = &self.download_context {
            context
                .alive
                .store(false, std::sync::atomic::Ordering::Release);
            if self.handle != 0 {
                let mut callback: Option<raw::DOWNLOADPROC> = None;
                unsafe {
                    (self.inner.bass.channel_set_attribute_ex)(
                        self.handle,
                        raw::BASS_ATTRIB_DOWNLOADPROC,
                        &mut callback as *mut Option<raw::DOWNLOADPROC> as *mut c_void,
                        std::mem::size_of::<Option<raw::DOWNLOADPROC>>() as u32,
                    );
                }
            }
        }
        if self.handle != 0 {
            unsafe {
                (self.inner.bass.channel_free)(self.handle);
            }
        }
        if let Some(user) = self.download_user.take() {
            unsafe {
                drop(Arc::from_raw(user as *const DownloadContext));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveState {
    Stopped,
    Playing,
    Stalled,
    Paused,
    PausedDevice,
}

#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub frequency: u32,
    pub channels: u32,
    pub flags: DWORD,
    pub channel_type: DWORD,
    pub original_resolution: DWORD,
    pub plugin: Option<u32>,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum TagKind {
    Id3,
    Id3v2,
    Ogg,
    Http,
    Icy,
    Meta,
    Ape,
    Mp4,
    Wma,
    Vendor,
    MediaFoundation,
}
impl TagKind {
    fn raw(self) -> DWORD {
        match self {
            Self::Id3 => raw::BASS_TAG_ID3,
            Self::Id3v2 => raw::BASS_TAG_ID3V2,
            Self::Ogg => raw::BASS_TAG_OGG,
            Self::Http => raw::BASS_TAG_HTTP,
            Self::Icy => raw::BASS_TAG_ICY,
            Self::Meta => raw::BASS_TAG_META,
            Self::Ape => raw::BASS_TAG_APE,
            Self::Mp4 => raw::BASS_TAG_MP4,
            Self::Wma => raw::BASS_TAG_WMA,
            Self::Vendor => raw::BASS_TAG_VENDOR,
            Self::MediaFoundation => raw::BASS_TAG_MF,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemoteProgress {
    pub state: ActiveState,
    pub buffering_percent: Option<u8>,
    pub downloaded_bytes: Option<QWORD>,
    pub buffered_bytes: Option<QWORD>,
    pub available_bytes: Option<QWORD>,
    pub bytes_per_second: Option<f64>,
}

pub struct SyncRegistration {
    pub(crate) inner: Arc<EngineInner>,
    pub(crate) channel: u32,
    pub(crate) handle: u32,
    pub(crate) context: Arc<SyncContext>,
}
impl Drop for SyncRegistration {
    fn drop(&mut self) {
        self.context
            .alive
            .store(false, std::sync::atomic::Ordering::Release);
        unsafe {
            (self.inner.bass.channel_remove_sync)(self.channel, self.handle);
        }
    }
}

pub enum SyncKind {
    Position(QWORD),
    End,
    Meta,
    Stall,
    Download,
    Free,
    OggChange,
    Other { kind: DWORD, parameter: QWORD },
}
impl SyncKind {
    fn raw(&self) -> (DWORD, QWORD) {
        match *self {
            Self::Position(p) => (raw::BASS_SYNC_POS, p),
            Self::End => (raw::BASS_SYNC_END, 0),
            Self::Meta => (raw::BASS_SYNC_META, 0),
            Self::Stall => (raw::BASS_SYNC_STALL, 0),
            Self::Download => (raw::BASS_SYNC_DOWNLOAD, 0),
            Self::Free => (raw::BASS_SYNC_FREE, 0),
            Self::OggChange => (raw::BASS_SYNC_OGG_CHANGE, 0),
            Self::Other { kind, parameter } => (kind, parameter),
        }
    }
}
pub type SyncCallback = Box<dyn FnMut(SyncEvent) + Send + 'static>;
#[derive(Debug, Clone, Copy)]
pub struct SyncEvent {
    pub sync_handle: u32,
    pub channel: u32,
    pub data: u32,
}

pub struct Plugin {
    pub(crate) inner: Arc<EngineInner>,
    pub(crate) handle: u32,
    pub(crate) info: PluginInfo,
}
impl Plugin {
    pub fn raw_handle(&self) -> u32 {
        self.handle
    }
    pub fn info(&self) -> &PluginInfo {
        &self.info
    }
    pub fn enable(&self, enabled: bool) -> Result<()> {
        let ok = unsafe { (self.inner.bass.plugin_enable)(self.handle, enabled as i32) };
        check_bool("BASS_PluginEnable", ok, unsafe {
            (self.inner.bass.error_get_code)()
        })
    }
}
impl Drop for Plugin {
    fn drop(&mut self) {
        unsafe {
            (self.inner.bass.plugin_free)(self.handle);
        }
    }
}
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub version: DWORD,
    pub formats: Vec<PluginFormat>,
}
#[derive(Debug, Clone)]
pub struct PluginFormat {
    pub channel_type: DWORD,
    pub name: String,
    pub extensions: String,
}

fn file_position(inner: &Arc<EngineInner>, handle: u32, mode: DWORD) -> Option<QWORD> {
    let value = unsafe { (inner.bass.stream_get_file_position)(handle, mode) };
    (value != QWORD::MAX).then_some(value)
}
fn c_string(ptr: *const std::ffi::c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
        if let Ok(value) = std::str::from_utf8(bytes) {
            return value.to_owned();
        }
        #[cfg(windows)]
        {
            use windows_sys::Win32::Globalization::{CP_ACP, MultiByteToWideChar};
            let length = unsafe {
                MultiByteToWideChar(
                    CP_ACP,
                    0,
                    bytes.as_ptr(),
                    bytes.len() as i32,
                    std::ptr::null_mut(),
                    0,
                )
            };
            if length > 0 {
                let mut wide = vec![0u16; length as usize];
                let written = unsafe {
                    MultiByteToWideChar(
                        CP_ACP,
                        0,
                        bytes.as_ptr(),
                        bytes.len() as i32,
                        wide.as_mut_ptr(),
                        length,
                    )
                };
                if written > 0 {
                    return String::from_utf16_lossy(&wide[..written as usize]);
                }
            }
        }
        String::from_utf8_lossy(bytes).into_owned()
    }
}
fn c_string_opt(ptr: *const std::ffi::c_char) -> Option<String> {
    (!ptr.is_null()).then(|| c_string(ptr))
}
fn parse_tag_strings(ptr: *const std::ffi::c_char) -> Vec<String> {
    if ptr.is_null() {
        return Vec::new();
    }
    let mut result = Vec::new();
    let mut current = ptr;
    for _ in 0..4096 {
        let text = unsafe { CStr::from_ptr(current) };
        if text.to_bytes().is_empty() {
            break;
        }
        result.push(text.to_string_lossy().into_owned());
        current = unsafe { current.add(text.to_bytes_with_nul().len()) };
    }
    result
}
