// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Context;
use Font;
use FontDescription;
use FontFamily;
use Fontset;
use Language;
use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FontMap(Object<ffi::PangoFontMap, ffi::PangoFontMapClass, FontMapClass>);

    match fn {
        get_type => || ffi::pango_font_map_get_type(),
    }
}

pub const NONE_FONT_MAP: Option<&FontMap> = None;

pub trait FontMapExt: 'static {
    #[cfg(any(feature = "v1_34", feature = "dox"))]
    fn changed(&self);

    fn create_context(&self) -> Option<Context>;

    #[cfg(any(feature = "v1_32_4", feature = "dox"))]
    fn get_serial(&self) -> u32;

    #[cfg_attr(feature = "v1_38", deprecated)]
    fn get_shape_engine_type(&self) -> Option<GString>;

    fn list_families(&self) -> Vec<FontFamily>;

    fn load_font<P: IsA<Context>>(&self, context: &P, desc: &FontDescription) -> Option<Font>;

    fn load_fontset<P: IsA<Context>>(&self, context: &P, desc: &FontDescription, language: &Language) -> Option<Fontset>;
}

impl<O: IsA<FontMap>> FontMapExt for O {
    #[cfg(any(feature = "v1_34", feature = "dox"))]
    fn changed(&self) {
        unsafe {
            ffi::pango_font_map_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn create_context(&self) -> Option<Context> {
        unsafe {
            from_glib_full(ffi::pango_font_map_create_context(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_32_4", feature = "dox"))]
    fn get_serial(&self) -> u32 {
        unsafe {
            ffi::pango_font_map_get_serial(self.as_ref().to_glib_none().0)
        }
    }

    fn get_shape_engine_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::pango_font_map_get_shape_engine_type(self.as_ref().to_glib_none().0))
        }
    }

    fn list_families(&self) -> Vec<FontFamily> {
        unsafe {
            let mut families = ptr::null_mut();
            let mut n_families = mem::uninitialized();
            ffi::pango_font_map_list_families(self.as_ref().to_glib_none().0, &mut families, &mut n_families);
            FromGlibContainer::from_glib_container_num(families, n_families as usize)
        }
    }

    fn load_font<P: IsA<Context>>(&self, context: &P, desc: &FontDescription) -> Option<Font> {
        unsafe {
            from_glib_full(ffi::pango_font_map_load_font(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, desc.to_glib_none().0))
        }
    }

    fn load_fontset<P: IsA<Context>>(&self, context: &P, desc: &FontDescription, language: &Language) -> Option<Fontset> {
        unsafe {
            from_glib_full(ffi::pango_font_map_load_fontset(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, desc.to_glib_none().0, mut_override(language.to_glib_none().0)))
        }
    }
}

impl fmt::Display for FontMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontMap")
    }
}
