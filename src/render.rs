//! Contains definitions for wlc render functions (wlc-render.h)

use libc::{c_void, uint32_t, uintptr_t};
use std::mem;
use super::types::{Geometry};

/// Number of bits per pixel (RGBA8888)
pub const BITS_PER_PIXEL: u32 = 32;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Allowed pixel formats
pub enum wlc_pixel_format {
    /// RGBA8888 format
    WLC_RGBA8888
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Enabled renderers
pub enum wlc_renderer {
    /// Render using GLE
    WLC_RENDERER_GLES2,
    /// Don't render (headless)
    WLC_NO_RENDERER
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum wlc_surface_format {
    SURFACE_RGB,
    SURFACE_RGBA,
    SURFACE_EGL,
    SURFACE_Y_UV,
    SURFACE_Y_U_V,
    SURFACE_Y_XUXV,
}

#[cfg_attr(feature = "static-wlc", link(name = "wlc", kind = "static"))]
#[cfg_attr(not(feature = "static-wlc"), link(name = "wlc"))]
extern "C" {
    /// Write pixel data with the specific format to output's framebuffer.
    /// If the geometry is out of bounds, it will be automatically clamped.
    pub fn wlc_pixels_write(format: wlc_pixel_format, geometry: *const Geometry, data: *const c_void);

    pub fn wlc_pixels_read(format: wlc_pixel_format,
                           geometry: *const Geometry,
                           out_geo: *mut Geometry,
                           data: *mut c_void);

    /** Renders surface. */
    pub fn wlc_surface_render(surface: uintptr_t, geometry: *const Geometry);

    /// Read pixel data from output's framebuffer.
    /// If theif output is currently rendering, it will render immediately after.
    pub fn wlc_output_schedule_render(output: uintptr_t) -> wlc_renderer;

    /// Adds frame callbacks of the given surface for the next output frame.
    /// It applies recursively to all subsurfaces.
    /// Useful when the compositor creates custom animations which require disabling internal rendering,
    /// but still need to update the surface textures (for ex. video players).
    pub fn wlc_surface_flush_frame_callbacks(surface: uintptr_t);

    /// Returns currently active renderer on the given output
    pub fn wlc_output_get_renderer(output: uintptr_t) -> wlc_renderer;

    /// Fills out_textures[] with the textures of a surface. Returns false if surface is invalid.
    /// Array must have at least 3 elements and should be refreshed at each frame.
    /// Note that these are not only OpenGL textures but rather render-specific.
    /// For more info what they are check the renderer's source code */
    pub fn wlc_surface_get_textures(surface: uintptr_t,
                                out_textures: *mut uint32_t,
                                out_format: *mut wlc_surface_format);

    pub fn wlc_output_set_gamma(output: uintptr_t,
                                size: u16,
                                red: *mut u16,
                                green: *mut u16,
                                blue: *mut u16);

    pub fn wlc_output_get_gamma_size(output: uintptr_t) -> u16 ;
}


/// Write pixel data with the specific format to output's framebuffer.
/// If the geometry is out of bounds, it will be automatically clamped.
///
/// # Unsafety
/// The data is converted to a *mut c_void and then passed to C to read.
/// The size of it should be the stride of the geometry * height of the geometry.
pub fn write_pixels(format: wlc_pixel_format, geometry: Geometry, data: &[u8]) {
    unsafe {
        let data = data as *const _ as *const c_void;
        wlc_pixels_write(format, &geometry as *const Geometry, data);
    }
}

/// Reads the pixels at the specified geometry
pub fn read_pixels(format: wlc_pixel_format, mut geometry: Geometry) -> ([u8; 9], Vec<u8>) {
    let data_size = (geometry.size.w * geometry.size.h * 4) as usize;
    // magic response header size
    let header_size = 9;
    let mut in_buf: Vec<u8> = Vec::with_capacity(header_size + data_size);
    let in_buf_ptr = in_buf.as_mut_ptr();
    mem::forget(in_buf);
    let mut out_buf = unsafe {
        let mut out_geo = Geometry::zero();
        wlc_pixels_read(format,
                        &mut geometry as *mut _,
                        &mut out_geo as *mut _,
                        in_buf_ptr as *mut c_void);
        // TODO read the header for this information!
        let size = header_size +
            out_geo.size.w as usize * out_geo.size.h as usize * 4;
        Vec::from_raw_parts(in_buf_ptr, size, size)
    };
    let mut header_response = [0u8; 9];
    let response: Vec<u8> = out_buf.drain(0..9).collect();
    header_response.copy_from_slice(response.as_slice());
    (header_response, out_buf)
}

/// Calculates the stride for ARGB32 encoded buffers
pub fn calculate_stride(width: u32) -> u32 {
    // function stolen from CAIRO_STRIDE_FOR_WIDTH macro in carioint.h
    // can be found in the most recent version of the cairo source
    let stride_alignment = ::std::mem::size_of::<u32>() as u32;
    ((BITS_PER_PIXEL * width + 7 ) / 8 + (stride_alignment - 1))  & (stride_alignment.overflowing_neg().0)
}
