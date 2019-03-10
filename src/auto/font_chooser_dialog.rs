// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Dialog;
use FontChooser;
use Widget;
use Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct FontChooserDialog(Object<gtk_sys::GtkFontChooserDialog, gtk_sys::GtkFontChooserDialogClass, FontChooserDialogClass>) @extends Dialog, Window, Bin, Container, Widget, @implements Buildable, FontChooser;

    match fn {
        get_type => || gtk_sys::gtk_font_chooser_dialog_get_type(),
    }
}

impl FontChooserDialog {
    pub fn new<P: IsA<Window>>(title: Option<&str>, parent: Option<&P>) -> FontChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_font_chooser_dialog_new(title.to_glib_none().0, parent.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_FONT_CHOOSER_DIALOG: Option<&FontChooserDialog> = None;

impl fmt::Display for FontChooserDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontChooserDialog")
    }
}
