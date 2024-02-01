// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Surface;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GdkGrabBrokenEvent")]
    pub struct GrabBrokenEvent(Shared<ffi::GdkGrabBrokenEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl StaticType for GrabBrokenEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_grab_broken_event_get_type()) }
    }
}

impl GrabBrokenEvent {
    #[doc(alias = "gdk_grab_broken_event_get_grab_surface")]
    #[doc(alias = "get_grab_surface")]
    pub fn grab_surface(&self) -> Surface {
        unsafe {
            from_glib_none(ffi::gdk_grab_broken_event_get_grab_surface(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_grab_broken_event_get_implicit")]
    #[doc(alias = "get_implicit")]
    pub fn is_implicit(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_grab_broken_event_get_implicit(
                self.to_glib_none().0,
            ))
        }
    }
}
