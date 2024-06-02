// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GDatagramBased")]
    pub struct DatagramBased(Interface<ffi::GDatagramBased, ffi::GDatagramBasedInterface>);

    match fn {
        type_ => || ffi::g_datagram_based_get_type(),
    }
}

impl DatagramBased {
    pub const NONE: Option<&'static DatagramBased> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DatagramBased>> Sealed for T {}
}

pub trait DatagramBasedExt: IsA<DatagramBased> + sealed::Sealed + 'static {
    #[doc(alias = "g_datagram_based_condition_check")]
    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition {
        unsafe {
            from_glib(ffi::g_datagram_based_condition_check(
                self.as_ref().to_glib_none().0,
                condition.into_glib(),
            ))
        }
    }
}

impl<O: IsA<DatagramBased>> DatagramBasedExt for O {}
