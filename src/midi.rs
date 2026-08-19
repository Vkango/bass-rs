use std::path::{Path, PathBuf};

use crate::{BassError, Result};

#[derive(Debug, Clone, Default)]
pub struct MidiOptions {
    pub max_polyphony: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct MidiAddon {
    pub path: PathBuf,
}

impl MidiAddon {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !path.is_file() {
            return Err(BassError::AddonUnavailable { name: "BASSMIDI" });
        }
        Err(BassError::Unsupported {
            operation: "BASSMIDI ABI (official bassmidi.h is not present)",
        })
    }

    /// Locate `bassmidi.dll` (or the platform equivalent) in a caller-
    /// supplied directory and load it.
    pub fn load_from_directory(directory: impl AsRef<Path>) -> Result<Self> {
        let directory = directory.as_ref();
        if !directory.is_dir() {
            return Err(BassError::InvalidInput {
                kind: "DLL directory",
                message: format!("{} is not a directory", directory.display()),
            });
        }
        Self::load(directory.join(platform_library_name("bassmidi")))
    }

    pub fn set_max_polyphony(&self, _options: MidiOptions) -> Result<()> {
        Err(BassError::Unsupported {
            operation: "BASSMIDI ABI (official bassmidi.h is not present)",
        })
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
