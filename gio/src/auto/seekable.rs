// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Cancellable;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct Seekable(Interface<ffi::GSeekable, ffi::GSeekableIface>);

    match fn {
        type_ => || ffi::g_seekable_get_type(),
    }
}

pub const NONE_SEEKABLE: Option<&Seekable> = None;

pub trait SeekableExt: 'static {
    #[doc(alias = "g_seekable_can_seek")]
    fn can_seek(&self) -> bool;

    #[doc(alias = "g_seekable_can_truncate")]
    fn can_truncate(&self) -> bool;

    #[doc(alias = "g_seekable_seek")]
    fn seek<P: IsA<Cancellable>>(
        &self,
        offset: i64,
        type_: glib::SeekType,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_seekable_tell")]
    fn tell(&self) -> i64;

    #[doc(alias = "g_seekable_truncate")]
    fn truncate<P: IsA<Cancellable>>(
        &self,
        offset: i64,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;
}

impl<O: IsA<Seekable>> SeekableExt for O {
    fn can_seek(&self) -> bool {
        unsafe { from_glib(ffi::g_seekable_can_seek(self.as_ref().to_glib_none().0)) }
    }

    fn can_truncate(&self) -> bool {
        unsafe { from_glib(ffi::g_seekable_can_truncate(self.as_ref().to_glib_none().0)) }
    }

    fn seek<P: IsA<Cancellable>>(
        &self,
        offset: i64,
        type_: glib::SeekType,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_seekable_seek(
                self.as_ref().to_glib_none().0,
                offset,
                type_.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn tell(&self) -> i64 {
        unsafe { ffi::g_seekable_tell(self.as_ref().to_glib_none().0) }
    }

    fn truncate<P: IsA<Cancellable>>(
        &self,
        offset: i64,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_seekable_truncate(
                self.as_ref().to_glib_none().0,
                offset,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for Seekable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Seekable")
    }
}
