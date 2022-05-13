// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Application;
use crate::File;
use glib::prelude::*;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib::GString;
use std::boxed::Box as Box_;
use std::mem::transmute;

pub trait ApplicationExtManual {
    #[doc(alias = "g_application_run")]
    fn run(&self) -> i32;
    #[doc(alias = "g_application_run")]
    fn run_with_args<S: AsRef<str>>(&self, args: &[S]) -> i32;
    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "g_application_hold")]
    fn hold(&self) -> ApplicationHoldGuard;

    #[doc(alias = "g_application_mark_busy")]
    fn mark_busy(&self) -> ApplicationBusyGuard;
}

impl<O: IsA<Application>> ApplicationExtManual for O {
    fn run(&self) -> i32 {
        self.run_with_args(&std::env::args().collect::<Vec<_>>())
    }

    fn run_with_args<S: AsRef<str>>(&self, args: &[S]) -> i32 {
        let argv: Vec<&str> = args.iter().map(|a| a.as_ref()).collect();
        let argc = argv.len() as i32;
        unsafe {
            ffi::g_application_run(self.as_ref().to_glib_none().0, argc, argv.to_glib_none().0)
        }
    }

    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn open_trampoline<P, F: Fn(&P, &[File], &str) + 'static>(
            this: *mut ffi::GApplication,
            files: *const *mut ffi::GFile,
            n_files: libc::c_int,
            hint: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            let files: Vec<File> = FromGlibContainer::from_glib_none_num(files, n_files as usize);
            f(
                Application::from_glib_borrow(this).unsafe_cast_ref(),
                &files,
                &GString::from_glib_borrow(hint),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"open\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    open_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn hold(&self) -> ApplicationHoldGuard {
        unsafe {
            ffi::g_application_hold(self.as_ref().to_glib_none().0);
        }
        ApplicationHoldGuard(self.as_ref().downgrade())
    }

    fn mark_busy(&self) -> ApplicationBusyGuard {
        unsafe {
            ffi::g_application_mark_busy(self.as_ref().to_glib_none().0);
        }
        ApplicationBusyGuard(self.as_ref().downgrade())
    }
}

#[derive(Debug)]
pub struct ApplicationHoldGuard(glib::WeakRef<Application>);

impl Drop for ApplicationHoldGuard {
    fn drop(&mut self) {
        if let Some(application) = self.0.upgrade() {
            unsafe {
                ffi::g_application_release(application.to_glib_none().0);
            }
        }
    }
}

#[derive(Debug)]
pub struct ApplicationBusyGuard(glib::WeakRef<Application>);

impl Drop for ApplicationBusyGuard {
    fn drop(&mut self) {
        if let Some(application) = self.0.upgrade() {
            unsafe {
                ffi::g_application_unmark_busy(application.to_glib_none().0);
            }
        }
    }
}
