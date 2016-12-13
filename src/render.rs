//! Contains definitions for wlc render functions (wlc-render.h)

use libc::{c_void, uint32_t, uintptr_t};
use super::handle::{WlcOutput};
use super::types::{Geometry};

#[cfg_attr(feature = "static-wlc", link(name = "wlc", kind = "static"))]
#[cfg_attr(not(feature = "static-wlc"), link(name = "wlc"))]

#[repr(C)]
enum wlc_pixel_format {
    WLC_RGBA8888
}

#[repr(C)]
enum wlc_renderer {
    WLC_RENDERER_GLES2,
    WLC_NO_RENDERER
}

#[repr(C)]
enum wlc_surface_format {
    SURFACE_RGB,
    SURFACE_RGBA,
    SURFACE_EGL,
    SURFACE_Y_UV,
    SURFACE_Y_U_V,
    SURFACE_Y_XUXV,
}

extern "C" {
    /// Write pixel data with the specific format to output's framebuffer.
    /// If the geometry is out of bounds, it will be automaticall clamped.
    fn wlc_pixels_write(format: wlc_pixel_format, geometry: *const Geometry, data: *const c_void);

    /// Read pixel data from output's framebuffer.
    /// If the geometry is out of bounds, it will be automatically clamped.
    /// Potentially clamped geometry will be stored in out_geometry,
    /// to indicate width / height of the returned data.
    fn wlc_pixels_read(format: wlc_pixel_format, geometry: *const Geometry, data: *const c_void);

    fn wlc_surface_render(surface: uintptr_t, geometry: *const Geometry);

    /// Schedules output for rendering next frame. If output was already scheduled this is no-op,
    /// if output is currently rendering, it will render immediately after.
    fn wlc_output_schedule_render(output: WlcOutput);

    /// Adds frame callbacks of the given surface for the next output frame.
    /// It applies recursively to all subsurfaces.
    /// Useful when the compositor creates custom animations which require disabling internal rendering,
    /// but still need to update the surface textures (for ex. video players).
    fn wlc_surface_flush_frame_callbacks(surface: uintptr_t);

    /// Returns currently active renderer on the given output
    fn wlc_output_get_renderers(output: WlcOutput);

    /// Fills out_textures[] with the textures of a surface. Returns false if surface is invalid.
    /// Array must have at least 3 elements and should be refreshed at each frame.
    /// Note that these are not only OpenGL textures but rather render-specific.
    /// For more info what they are check the renderer's source code */
    fn wlc_surface_get_textures(surface: uintptr_t,
                                out_textures: *mut uint32_t,
                                out_format: *mut wlc_surface_format);
}
