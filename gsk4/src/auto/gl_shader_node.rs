// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{GLShader, RenderNode};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GskGLShaderNode")]
    pub struct GLShaderNode(Shared<ffi::GskGLShaderNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl StaticType for GLShaderNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_gl_shader_node_get_type()) }
    }
}

impl GLShaderNode {
    #[doc(alias = "gsk_gl_shader_node_new")]
    pub fn new(
        shader: &GLShader,
        bounds: &graphene::Rect,
        args: &glib::Bytes,
        children: &[RenderNode],
    ) -> GLShaderNode {
        skip_assert_initialized!();
        let n_children = children.len() as _;
        unsafe {
            from_glib_full(ffi::gsk_gl_shader_node_new(
                shader.to_glib_none().0,
                bounds.to_glib_none().0,
                args.to_glib_none().0,
                children.to_glib_none().0,
                n_children,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_node_get_args")]
    #[doc(alias = "get_args")]
    pub fn args(&self) -> glib::Bytes {
        unsafe { from_glib_none(ffi::gsk_gl_shader_node_get_args(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_gl_shader_node_get_n_children")]
    #[doc(alias = "get_n_children")]
    pub fn n_children(&self) -> u32 {
        unsafe { ffi::gsk_gl_shader_node_get_n_children(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_node_get_shader")]
    #[doc(alias = "get_shader")]
    pub fn shader(&self) -> GLShader {
        unsafe { from_glib_none(ffi::gsk_gl_shader_node_get_shader(self.to_glib_none().0)) }
    }
}
