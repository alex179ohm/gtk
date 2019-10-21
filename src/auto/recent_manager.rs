// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use Error;
use RecentData;
use RecentInfo;

glib_wrapper! {
    pub struct RecentManager(Object<gtk_sys::GtkRecentManager, gtk_sys::GtkRecentManagerClass, RecentManagerClass>);

    match fn {
        get_type => || gtk_sys::gtk_recent_manager_get_type(),
    }
}

impl RecentManager {
    pub fn new() -> RecentManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_recent_manager_new()) }
    }

    pub fn get_default() -> Option<RecentManager> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gtk_sys::gtk_recent_manager_get_default()) }
    }
}

#[derive(Default)]
pub struct RecentManagerBuilder {
    filename: Option<String>,
}

impl RecentManagerBuilder {
    pub fn new() -> Self {
        Self { filename: None }
    }

    pub fn build(self) -> RecentManager {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref filename) = self.filename {
            properties.push(("filename", filename));
        }
        glib::Object::new(RecentManager::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }
}

pub const NONE_RECENT_MANAGER: Option<&RecentManager> = None;

pub trait RecentManagerExt: 'static {
    fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool;

    fn add_item(&self, uri: &str) -> bool;

    fn get_items(&self) -> Vec<RecentInfo>;

    fn has_item(&self, uri: &str) -> bool;

    fn lookup_item(&self, uri: &str) -> Result<Option<RecentInfo>, Error>;

    fn move_item(&self, uri: &str, new_uri: Option<&str>) -> Result<(), Error>;

    fn purge_items(&self) -> Result<i32, Error>;

    fn remove_item(&self, uri: &str) -> Result<(), Error>;

    fn get_property_filename(&self) -> Option<GString>;

    fn get_property_size(&self) -> i32;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RecentManager>> RecentManagerExt for O {
    fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_manager_add_full(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                recent_data.to_glib_none().0,
            ))
        }
    }

    fn add_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_manager_add_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn get_items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_recent_manager_get_items(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_manager_has_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn lookup_item(&self, uri: &str) -> Result<Option<RecentInfo>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_recent_manager_lookup_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn move_item(&self, uri: &str, new_uri: Option<&str>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_recent_manager_move_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                new_uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn purge_items(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                gtk_sys::gtk_recent_manager_purge_items(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_item(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_recent_manager_remove_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_property_filename(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"filename\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `filename` getter")
        }
    }

    fn get_property_size(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `size` getter")
                .unwrap()
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentManager,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentManager>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentManager::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentManager,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentManager>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentManager::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute(notify_size_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RecentManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RecentManager")
    }
}
