use std::{
    ffi::{OsStr, c_char},
    path::Path,
};

#[cfg(not(windows))]
use std::ffi::CString;

use crate::{BassError, Result};

pub(crate) enum NativePath {
    #[cfg(windows)]
    Wide(Vec<u16>),
    #[cfg(not(windows))]
    Narrow(CString),
}

impl NativePath {
    pub(crate) fn new(path: impl AsRef<Path>, kind: &'static str) -> Result<Self> {
        let path = path.as_ref();
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStrExt;
            let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
            if wide.contains(&0) {
                return Err(BassError::InvalidInput {
                    kind,
                    message: "path contains an embedded NUL".into(),
                });
            }
            wide.push(0);
            Ok(Self::Wide(wide))
        }
        #[cfg(not(windows))]
        {
            let value = path.as_os_str().to_string_lossy();
            let c = CString::new(value.as_bytes()).map_err(|_| BassError::InvalidInput {
                kind,
                message: "path contains an embedded NUL".into(),
            })?;
            Ok(Self::Narrow(c))
        }
    }

    pub(crate) fn as_ptr(&self) -> *const c_char {
        match self {
            #[cfg(windows)]
            Self::Wide(value) => value.as_ptr() as *const c_char,
            #[cfg(not(windows))]
            Self::Narrow(value) => value.as_ptr(),
        }
    }

    pub(crate) fn flags(&self, flags: u32) -> u32 {
        #[cfg(windows)]
        {
            if matches!(self, Self::Wide(_)) {
                return flags | crate::raw::BASS_UNICODE;
            }
        }
        flags
    }
}

#[allow(dead_code)]
pub(crate) fn os_str_to_string(value: &OsStr) -> String {
    value.to_string_lossy().into_owned()
}
