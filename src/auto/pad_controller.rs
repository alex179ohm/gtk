// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use gobject_sys;
use gtk_sys;
use std::fmt;
use EventController;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use PadActionType;
use PropagationPhase;
use Widget;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use Window;

glib_wrapper! {
    pub struct PadController(Object<gtk_sys::GtkPadController, gtk_sys::GtkPadControllerClass, PadControllerClass>) @extends EventController;

    match fn {
        get_type => || gtk_sys::gtk_pad_controller_get_type(),
    }
}

impl PadController {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn new<P: IsA<Window>, Q: IsA<gio::ActionGroup>>(
        window: &P,
        group: &Q,
        pad: Option<&gdk::Device>,
    ) -> PadController {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_sys::gtk_pad_controller_new(
                window.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                pad.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn set_action(
        &self,
        type_: PadActionType,
        index: i32,
        mode: i32,
        label: &str,
        action_name: &str,
    ) {
        unsafe {
            gtk_sys::gtk_pad_controller_set_action(
                self.to_glib_none().0,
                type_.to_glib(),
                index,
                mode,
                label.to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    pub fn get_property_action_group(&self) -> Option<gio::ActionGroup> {
        unsafe {
            let mut value = Value::from_type(<gio::ActionGroup as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"action-group\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `action-group` getter")
        }
    }

    pub fn get_property_pad(&self) -> Option<gdk::Device> {
        unsafe {
            let mut value = Value::from_type(<gdk::Device as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"pad\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `pad` getter")
        }
    }
}

pub struct PadControllerBuilder {
    action_group: Option<gio::ActionGroup>,
    pad: Option<gdk::Device>,
    propagation_phase: Option<PropagationPhase>,
    widget: Option<Widget>,
}

impl PadControllerBuilder {
    pub fn new() -> Self {
        Self {
            action_group: None,
            pad: None,
            propagation_phase: None,
            widget: None,
        }
    }

    pub fn build(self) -> PadController {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref action_group) = self.action_group {
            properties.push(("action-group", action_group));
        }
        if let Some(ref pad) = self.pad {
            properties.push(("pad", pad));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        glib::Object::new(PadController::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn action_group<P: IsA<gio::ActionGroup>>(mut self, action_group: &P) -> Self {
        self.action_group = Some(action_group.clone().upcast());
        self
    }

    pub fn pad(mut self, pad: &gdk::Device) -> Self {
        self.pad = Some(pad.clone());
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }

    pub fn widget<P: IsA<Widget>>(mut self, widget: &P) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }
}

impl fmt::Display for PadController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PadController")
    }
}
