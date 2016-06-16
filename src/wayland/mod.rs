//! Contains bindings for optional Wayland dependencies

use wayland_sys::server::{wl_client, wl_display, wl_resource};
use wayland_sys::common::wl_interface;

use libc::{uintptr_t, c_void};

use types::Size;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct WlcResource(uintptr_t);

/// Functions defined in wlc-wayland.h
#[link(name = "wlc")]
extern "C" {
    fn wlc_get_wl_display() -> *const wl_display;
    fn wlc_handle_from_wl_output_resource(resourece: *const wl_resource) -> uintptr_t;
    fn wlc_handle_from_wl_surface_resource(resource: *const wl_resource) -> uintptr_t;
    fn wlc_resource_from_wl_surface_resource(resource: *const wl_resource) -> uintptr_t;
    fn wlc_surface_get_size(resource: uintptr_t) -> *const Size;
    fn wlc_view_from_surface(resource: uintptr_t, client: *const wl_client,
                             interface: *const wl_interface,
                             implementation: *const c_void, version: u32,
                             id: u32, userdata: *const c_void) -> uintptr_t;
    // view -> wlc_resource
    fn wlc_view_get_surface(view: uintptr_t) -> uintptr_t;
    // view -> wlc_resource
    fn wlc_surface_get_subsurfaces(parent: uintptr_t, out_size: *mut Size)
                                   -> *const uintptr_t;
    // resource
    fn wlc_get_subsurface_geometry(surface: uintptr_t, out_geo: *mut Geometry);
    fn wlc_view_get_role(view: uintptr_t) -> *const wl_resource;
}
