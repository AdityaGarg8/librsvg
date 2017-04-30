// This file was generated by gir (2184662) from gir-files (ec4c204)
// DO NOT EDIT

use ffi;
use glib::translate::*;

bitflags! {
    pub flags HandleFlags: u32 {
        const HANDLE_FLAGS_NONE = 0,
        const HANDLE_FLAG_UNLIMITED = 1,
        const HANDLE_FLAG_KEEP_IMAGE_DATA = 2,
    }
}

#[doc(hidden)]
impl ToGlib for HandleFlags {
    type GlibType = ffi::RsvgHandleFlags;

    fn to_glib(&self) -> ffi::RsvgHandleFlags {
        ffi::RsvgHandleFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::RsvgHandleFlags> for HandleFlags {
    fn from_glib(value: ffi::RsvgHandleFlags) -> HandleFlags {
        HandleFlags::from_bits_truncate(value.bits())
    }
}

