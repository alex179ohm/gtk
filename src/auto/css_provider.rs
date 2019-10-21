// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use CssSection;
use Error;
use StyleProvider;

glib_wrapper! {
    pub struct CssProvider(Object<gtk_sys::GtkCssProvider, gtk_sys::GtkCssProviderClass, CssProviderClass>) @implements StyleProvider;

    match fn {
        get_type => || gtk_sys::gtk_css_provider_get_type(),
    }
}

impl CssProvider {
    pub fn new() -> CssProvider {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_css_provider_new()) }
    }

    #[cfg_attr(feature = "v3_24", deprecated)]
    pub fn get_default() -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gtk_sys::gtk_css_provider_get_default()) }
    }

    pub fn get_named(name: &str, variant: Option<&str>) -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gtk_sys::gtk_css_provider_get_named(
                name.to_glib_none().0,
                variant.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_CSS_PROVIDER: Option<&CssProvider> = None;

pub trait CssProviderExt: 'static {
    fn load_from_data(&self, data: &[u8]) -> Result<(), Error>;

    fn load_from_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error>;

    fn load_from_path(&self, path: &str) -> Result<(), Error>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn load_from_resource(&self, resource_path: &str);

    fn to_string(&self) -> GString;

    fn connect_parsing_error<F: Fn(&Self, &CssSection, &Error) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<CssProvider>> CssProviderExt for O {
    fn load_from_data(&self, data: &[u8]) -> Result<(), Error> {
        let length = data.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_css_provider_load_from_data(
                self.as_ref().to_glib_none().0,
                data.to_glib_none().0,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_from_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_css_provider_load_from_file(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_from_path(&self, path: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_css_provider_load_from_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn load_from_resource(&self, resource_path: &str) {
        unsafe {
            gtk_sys::gtk_css_provider_load_from_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
            );
        }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gtk_sys::gtk_css_provider_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_parsing_error<F: Fn(&Self, &CssSection, &Error) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn parsing_error_trampoline<P, F: Fn(&P, &CssSection, &Error) + 'static>(
            this: *mut gtk_sys::GtkCssProvider,
            section: *mut gtk_sys::GtkCssSection,
            error: *mut glib_sys::GError,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CssProvider>,
        {
            let f: &F = &*(f as *const F);
            f(
                &CssProvider::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(section),
                &from_glib_borrow(error),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"parsing-error\0".as_ptr() as *const _,
                Some(transmute(parsing_error_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CssProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CssProvider")
    }
}
