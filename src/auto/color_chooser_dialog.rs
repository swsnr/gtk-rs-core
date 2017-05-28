// This file was generated by gir (5c017c9) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use ColorChooser;
use Container;
use Dialog;
use Widget;
use Window;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct ColorChooserDialog(Object<ffi::GtkColorChooserDialog>): Dialog, Window, Bin, Container, Widget, ColorChooser;

    match fn {
        get_type => || ffi::gtk_color_chooser_dialog_get_type(),
    }
}

impl ColorChooserDialog {
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Window> + 'b, R: Into<Option<&'b Q>>>(title: P, parent: R) -> ColorChooserDialog {
        assert_initialized_main_thread!();
        let title = title.into();
        let title = title.to_glib_none();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_chooser_dialog_new(title.0, parent.0)).downcast_unchecked()
        }
    }
}

pub trait ColorChooserDialogExt {
    fn get_property_show_editor(&self) -> bool;

    fn set_property_show_editor(&self, show_editor: bool);
}

impl<O: IsA<ColorChooserDialog> + IsA<glib::object::Object>> ColorChooserDialogExt for O {
    fn get_property_show_editor(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-editor".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_editor(&self, show_editor: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-editor".to_glib_none().0, Value::from(&show_editor).to_glib_none().0);
        }
    }
}
