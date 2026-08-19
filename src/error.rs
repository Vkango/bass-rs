use std::path::PathBuf;

/// Errors returned by the safe BASS wrapper.
#[derive(Debug, thiserror::Error)]
pub enum BassError {
    #[error("could not load {library}: {message}")]
    LibraryLoad { library: PathBuf, message: String },

    #[error("missing symbol {symbol} in {library}")]
    MissingSymbol { library: PathBuf, symbol: String },

    #[error("BASS operation {operation} failed with error {code:?}")]
    Api {
        operation: &'static str,
        code: raw_code::ErrorCode,
    },

    #[error("BASS version mismatch: expected API {expected:#x}, got {actual:#x}")]
    VersionMismatch { expected: u16, actual: u16 },

    #[error("BASS_FX is not loaded")]
    FxUnavailable,

    #[error("add-on {name} is unavailable; provide its official DLL through the documented path")]
    AddonUnavailable { name: &'static str },

    #[error("invalid {kind}: {message}")]
    InvalidInput { kind: &'static str, message: String },

    #[error("operation {operation} is not supported on this platform")]
    Unsupported { operation: &'static str },

    #[error("a callback panicked while handling a BASS event")]
    CallbackPanicked,
}

/// Result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, BassError>;

mod raw_code {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ErrorCode {
        Ok,
        Mem,
        FileOpen,
        Driver,
        BufLost,
        Handle,
        Format,
        Position,
        Init,
        Start,
        Ssl,
        Reinit,
        Track,
        Already,
        NotAudio,
        NoChan,
        IllType,
        IllParam,
        No3d,
        NoEax,
        Device,
        NoPlay,
        Freq,
        NotFile,
        NoHw,
        Empty,
        NoNet,
        Create,
        NoFx,
        NotAvail,
        Decode,
        Dx,
        Timeout,
        FileForm,
        Speaker,
        Version,
        Codec,
        Ended,
        Busy,
        Unstreamable,
        Protocol,
        Denied,
        Freeing,
        Cancel,
        Unknown(i32),
    }

    impl From<i32> for ErrorCode {
        fn from(value: i32) -> Self {
            match value {
                0 => Self::Ok,
                1 => Self::Mem,
                2 => Self::FileOpen,
                3 => Self::Driver,
                4 => Self::BufLost,
                5 => Self::Handle,
                6 => Self::Format,
                7 => Self::Position,
                8 => Self::Init,
                9 => Self::Start,
                10 => Self::Ssl,
                11 => Self::Reinit,
                13 => Self::Track,
                14 => Self::Already,
                17 => Self::NotAudio,
                18 => Self::NoChan,
                19 => Self::IllType,
                20 => Self::IllParam,
                21 => Self::No3d,
                22 => Self::NoEax,
                23 => Self::Device,
                24 => Self::NoPlay,
                25 => Self::Freq,
                27 => Self::NotFile,
                29 => Self::NoHw,
                31 => Self::Empty,
                32 => Self::NoNet,
                33 => Self::Create,
                34 => Self::NoFx,
                37 => Self::NotAvail,
                38 => Self::Decode,
                39 => Self::Dx,
                40 => Self::Timeout,
                41 => Self::FileForm,
                42 => Self::Speaker,
                43 => Self::Version,
                44 => Self::Codec,
                45 => Self::Ended,
                46 => Self::Busy,
                47 => Self::Unstreamable,
                48 => Self::Protocol,
                49 => Self::Denied,
                50 => Self::Freeing,
                51 => Self::Cancel,
                other => Self::Unknown(other),
            }
        }
    }
}

pub(crate) fn api_error(operation: &'static str, code: i32) -> BassError {
    BassError::Api {
        operation,
        code: code.into(),
    }
}

pub(crate) fn check_bool(operation: &'static str, value: i32, error_code: i32) -> Result<()> {
    if value != 0 {
        Ok(())
    } else {
        Err(api_error(operation, error_code))
    }
}
