//! Contains definitions for wlc render functions (wlc-render.h)

use libc::{c_void, uint32_t, uintptr_t};
use super::handle::{WlcOutput};
use super::types::{Geometry};

#[cfg_attr(feature = "static-wlc", link(name = "wlc", kind = "static"))]
#[cfg_attr(not(feature = "static-wlc"), link(name = "wlc"))]

#[repr(C)]
pub enum wlc_pixel_format {
    WLC_RGBA8888
}

#[repr(C)]
pub enum wlc_renderer {
    WLC_RENDERER_GLES2,
    WLC_NO_RENDERER
}

#[repr(C)]
pub enum wlc_surface_format {
    SURFACE_RGB,
    SURFACE_RGBA,
    SURFACE_EGL,
    SURFACE_Y_UV,
    SURFACE_Y_U_V,
    SURFACE_Y_XUXV,
}

extern "C" {
    /// Write pixel data with the specific format to output's framebuffer.
    /// If the geometry is out of bounds, it will be automatically clamped.
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


/// Write pixel data with the specific format to output's framebuffer.
/// If the geometry is out of bounds, it will be automatically clamped.
///
/// # Unsafety
/// The data is converted to a *mut c_void and then passed to C to read.
/// It should be in the format expected by wlc, see the wlc docs.
pub fn write_pixels<T>(format: wlc_pixel_format, geometry: Geometry, data: &mut T) {
    unsafe {
        let data = data as *mut _ as *mut c_void;
        wlc_pixels_write(format, &geometry as *const Geometry, data);
    }
}

/// Reads the pixels at the specified geometry
pub fn read_pixels(format: wlc_pixel_format, mut geometry: Geometry) -> ([u8; 9], Vec<u8>) {
    let data_size = (geometry.size.w * geometry.size.h * 4) as usize;
    // magic response header size
    let header_size = 9;
    let mut out_buf: Vec<u8> = Vec::with_capacity(header_size + data_size);
    unsafe {
        wlc_pixels_read(format, &mut geometry as *mut _, out_buf.as_mut_ptr() as *mut c_void);
    }
    let mut header_response = [0u8; 9];
    let response: Vec<u8> = out_buf.drain(0..9).collect();
    header_response.copy_from_slice(response.as_slice());
    (header_response, out_buf)
}
