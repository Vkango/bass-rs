use std::{
    ffi::{CStr, c_void},
    panic::{AssertUnwindSafe, catch_unwind},
    slice,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::{
    engine::{DownloadCallback, DownloadEvent, SyncCallback, SyncEvent},
    fx::{DspCallback, DspInfo},
};

pub(crate) struct DownloadContext {
    pub(crate) alive: AtomicBool,
    pub(crate) callback: Mutex<DownloadCallback>,
}

impl DownloadContext {
    pub(crate) fn new(callback: DownloadCallback) -> Arc<Self> {
        Arc::new(Self {
            alive: AtomicBool::new(true),
            callback: Mutex::new(callback),
        })
    }
}

pub(crate) unsafe extern "system" fn download_trampoline(
    buffer: *const c_void,
    length: u32,
    user: *mut c_void,
) {
    if user.is_null() {
        return;
    }
    // SAFETY: user is an Arc allocation held by Channel until the native
    // stream has been freed and no more callbacks can be issued.
    let context = unsafe { &*(user as *const DownloadContext) };
    if !context.alive.load(Ordering::Acquire) {
        return;
    }
    let event = if buffer.is_null() {
        DownloadEvent::Finished
    } else if length == 0 {
        let status = unsafe { CStr::from_ptr(buffer.cast()) }
            .to_string_lossy()
            .into_owned();
        DownloadEvent::Status(status)
    } else {
        DownloadEvent::Data {
            length: length as usize,
        }
    };
    if let Ok(mut callback) = context.callback.lock() {
        let _ = catch_unwind(AssertUnwindSafe(|| (callback)(event)));
    }
}

pub(crate) struct SyncContext {
    pub(crate) alive: AtomicBool,
    pub(crate) callback: Mutex<SyncCallback>,
}

impl SyncContext {
    pub(crate) fn new(callback: SyncCallback) -> Arc<Self> {
        Arc::new(Self {
            alive: AtomicBool::new(true),
            callback: Mutex::new(callback),
        })
    }
}

pub(crate) unsafe extern "system" fn sync_trampoline(
    handle: u32,
    channel: u32,
    data: u32,
    user: *mut c_void,
) {
    if user.is_null() {
        return;
    }
    // SAFETY: see download_trampoline.
    let context = unsafe { &*(user as *const SyncContext) };
    if !context.alive.load(Ordering::Acquire) {
        return;
    }
    if let Ok(mut callback) = context.callback.lock() {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            (callback)(SyncEvent {
                sync_handle: handle,
                channel,
                data,
            });
        }));
    }
}

pub(crate) struct DspContext {
    pub(crate) alive: AtomicBool,
    pub(crate) callback: Mutex<DspCallback>,
}

impl DspContext {
    pub(crate) fn new(callback: DspCallback) -> Arc<Self> {
        Arc::new(Self {
            alive: AtomicBool::new(true),
            callback: Mutex::new(callback),
        })
    }
}

pub(crate) unsafe extern "system" fn dsp_trampoline(
    handle: u32,
    channel: u32,
    buffer: *mut c_void,
    length: u32,
    user: *mut c_void,
) {
    if user.is_null() {
        return;
    }
    // SAFETY: see download_trampoline.
    let context = unsafe { &*(user as *const DspContext) };
    if !context.alive.load(Ordering::Acquire) {
        return;
    }
    let bytes = if buffer.is_null() {
        &mut []
    } else {
        // SAFETY: BASS owns the buffer for the duration of this callback and
        // provides its valid byte length.
        unsafe { slice::from_raw_parts_mut(buffer.cast::<u8>(), length as usize) }
    };
    if let Ok(mut callback) = context.callback.lock() {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            (callback)(
                bytes,
                DspInfo {
                    dsp_handle: handle,
                    channel,
                    byte_length: length as usize,
                },
            );
        }));
    }
}
