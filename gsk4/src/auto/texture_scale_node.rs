// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ScalingFilter;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GskTextureScaleNode")]
    pub struct TextureScaleNode(Shared<ffi::GskTextureScaleNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl StaticType for TextureScaleNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_texture_scale_node_get_type()) }
    }
}

impl TextureScaleNode {
    #[doc(alias = "gsk_texture_scale_node_new")]
    pub fn new(
        texture: &impl IsA<gdk::Texture>,
        bounds: &graphene::Rect,
        filter: ScalingFilter,
    ) -> TextureScaleNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_texture_scale_node_new(
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
                filter.into_glib(),
            ))
        }
    }

    #[doc(alias = "gsk_texture_scale_node_get_filter")]
    #[doc(alias = "get_filter")]
    pub fn filter(&self) -> ScalingFilter {
        unsafe {
            from_glib(ffi::gsk_texture_scale_node_get_filter(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_texture_scale_node_get_texture")]
    #[doc(alias = "get_texture")]
    pub fn texture(&self) -> gdk::Texture {
        unsafe {
            from_glib_none(ffi::gsk_texture_scale_node_get_texture(
                self.to_glib_none().0,
            ))
        }
    }
}
