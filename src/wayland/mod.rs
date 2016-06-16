//! # The wlc-wayland feature
//! This crate requires the `wlc-wayland` feature to be enabled.
//! The `wlc-wayland` feature adds additional methods to `WlcHandle` and
//! `WlcOutput` as well. It uses the [wayland_sys_crate][] crate for Wayland
//! types. See [the wayland_sys docs](wayland_sys_docs) for info on how to use
//! them.
//!
//! Usage of `wlc-wayland` or Wayland bindings _is not_ required to make a
//! compositor with rustwlc. wlc itself provides enough API around Wayland's
//! that a complete compositor can be written without any Rust bindings
//! to Wayland itself. `way-cooler` only uses Wayland for the standard Wayland
//! protocol and some `wl_surface` info.
//!
//! That said, if you already use `wayland-client` or `wayland-sys`, this module
//! provides compatability.
//!
//! # Wayland bindings
//! This crate contains wlc bindings to Wayland objects (defined in [wayland_sys]
//! (wayland_sys_docs)) and the `WlcResource` struct, which represents
//! wlc's Wayland resources. 
//!
//! [wayland_sys_docs]:http://vberger.github.io/wayland-client-rs/wayland_sys/index.html
//! [wayland_sys_crate]:https://crates.io/crates/wayland_sys
use wayland_sys::server::{wl_client, wl_display, wl_resource};
use wayland_sys::common::wl_interface;

use libc::{uintptr_t, c_void};

use types::Size;

/// ## Requires `wlc-wayland` feature
///
/// A wlc resource for Wayland interop
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct WlcResource(uintptr_t);

/// Functions defined in wlc-wayland.h
#[link(name = "wlc")]
extern "C" {
    fn wlc_get_wl_display() -> *const wl_display;
    fn wlc_resource_from_wl_surface_resource(resource: *const wl_resource) -> uintptr_t;
    fn wlc_surface_get_size(resource: uintptr_t) -> *const Size;
    // resource -> Vec<wlc_resource>
    fn wlc_surface_get_subsurfaces(parent: uintptr_t, out_size: *mut Size)
                                   -> *const uintptr_t;
    // resource
    fn wlc_get_subsurface_geometry(surface: uintptr_t, out_geo: *mut Geometry);
}

pub fn get_display() -> wl_display {
    unsafe { wlc_get_wl_display().clone() }
}

impl From<uintptr_t> for WlcResource {
    /// ## Requires `wlc-wayland` feature
    ///
    /// Creates a new WlcResource from the given pointer.
    fn from(ptr: uintptr_t) -> WlcResource {
        WlcResource(ptr)
    }
}

impl WlcResource {

    /// ## Requires `wlc-wayland` feature
    ///
    /// Creates a new WlcResource from the given Wayland surface
    pub fn from_wl_surface_resource(resource: wl_resource) {
        unsafe {
            WlcResource::from(wlc_resource_from_wl_surface_resource(resource))
        }
    }

    /// # Requires `wlc-wayland` feature
    ///
    /// Gets the size of this surface
    pub fn get_surface_size(&self) -> Size {
        unsafe { *wlc_surface_get_size(self.0).clone() }
    }

    /// ## Requires `wlc-wayland` feature
    ///
    /// Gets a list of subsurfaces from the given view
    pub fn get_subsurfaces(&self) -> Vec<WlcResource> {
        unsafe {
            let mut out_memb: size_t = 0;
            let subs = wlc_surface_get_subsurfaces(parent.pointer(), self.0);
            if subs.is_null() {
                return Vec::new()
            }
            let mut result = Vec::with_capacity(out_memb);
            for index in 0isize .. out_memb as isize {
                result.push(WlcResource::from(*subs.offset(index)))
            }
            return result
        }
    }

    /// # Requires `wlc-wayland` feature
    ///
    /// Gets the subsurface geometry of this WlcResource
    pub fn get_subsurface_geometry(&self) -> Geometry {
        let mut geo = Geometry::zero();
        unsafe {
            wlc_get_subsurface_geometry(self.0, geo as *mut Geometry);
        }
        geo
    }
}
