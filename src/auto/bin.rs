// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Bin(Object<ffi::GtkBin>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_bin_get_type(),
    }
}

pub trait BinExt {
    fn get_child(&self) -> Option<Widget>;
}

impl<O: IsA<Bin>> BinExt for O {
    fn get_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_bin_get_child(self.to_glib_none().0))
        }
    }
}
