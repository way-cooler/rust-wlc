//! Contains definitions for wlc handle types.
//! WlcHandle is the main object used in wlc.
//! wlc provided many functions for manipulating
//! wayland based on pointers to such handles.
//! rustwlc provides implp blocks for handles,
//! allowing the programmer to use "instance methods"
//! on each handle.
//!
//! These handles wrap wayland surfaces and other resources.
//! Their functionality is ecposed in their available methods.

use std::ffi;
extern crate libc;
use libc::c_char;

use super::pointer_to_string;

use super::types::{Geometry, Size, ViewType, ViewState};

#[repr(C)]
/// Reoresents a handle to a wlc window.
pub struct WlcHandle(libc::uintptr_t);

#[link(name = "wlc")]
extern "C" {
    fn wlc_output_get_name(output: &WlcHandle) -> *const c_char;

    fn wlc_handle_get_user_data(handle: WlcHandle) -> ();

    // TODO need representation of userdata
    //fn wlc_handle_set_user_data(handle: WlcHandle, userdata: ?????) -> ();

    fn wlc_output_get_sleep(output: &WlcHandle) -> bool;

    fn wlc_output_set_sleep(output: WlcHandle, sleep: bool) -> ();

    fn wlc_output_get_resolution(output: WlcHandle) -> Size;

    fn wlc_output_set_resolution(output: WlcHandle, resolution: Size) -> ();

    fn wlc_output_get_mask(output: WlcHandle) -> u32;

    fn wlc_output_set_mask(output: WlcHandle, mask: u32) -> ();

    // TODO tricky definition here
    //fn wlc_output_get_pixels(output: WlcHandle) -> ();

    fn wlc_output_get_views(output: WlcHandle, out_memb: libc::size_t);

    fn  wlc_output_get_mutable_views(output: WlcHandle, out_memb: libc::size_t) -> WlcHandle;

    fn wlc_output_set_views(output: WlcHandle, views: WlcHandle, memb: libc::size_t) -> bool;

    fn wlc_output_focus(output: WlcHandle) -> ();

    fn wlc_view_close(view: WlcHandle) -> ();

    fn wlc_view_get_output(view: WlcHandle) -> WlcHandle;

    // "set output. Alternatively you can use wlc_output_set_views"
    fn wlc_view_set_output(view: WlcHandle, output: WlcHandle) -> ();

    fn wlc_view_send_to_back(view: WlcHandle) -> ();

    fn wlc_view_send_below(view: WlcHandle, other: WlcHandle) -> ();

    fn wlc_view_bring_above(view: WlcHandle, other: WlcHandle) -> ();

    fn wlc_view_bring_to_front(view: WlcHandle) -> ();

    fn wlc_view_get_mask(view: WlcHandle) -> u32;

    fn wlc_view_set_mask(view: WlcHandle, mask: u32) -> ();

    fn wlc_view_get_geometry(view: WlcHandle) -> Geometry;

    fn wlc_view_set_geometry(view: WlcHandle, edges: u32, geo: Geometry) -> ();

    fn wlc_view_get_type(view: WlcHandle) -> u32;

    fn wlc_view_set_type(view: WlcHandle, view_type: ViewType, toggle: bool) -> ();

    fn wlc_view_get_state(view: WlcHandle) -> u32;

    fn wlc_view_set_state(view: WlcHandle, state: ViewState, toggle: bool) ->();

    fn wlc_view_get_parent(view: WlcHandle) -> WlcHandle;

    fn wlc_view_set_parent(view: WlcHandle, parent: WlcHandle) -> ();

    fn wlc_view_get_title(view: WlcHandle) -> *const c_char;

    fn wlc_view_get_class(view: WlcHandle) -> *const c_char;

    fn wlc_view_get_app_id(view: WlcHandle) -> *const c_char;
}

impl WlcHandle {
    fn get_name(&self) -> String {
        unsafe {
            let name = wlc_output_get_name(self);
            pointer_to_string(name)
        }
    }

    
    fn get_sleep(&self) -> bool {
        unsafe {
            wlc_output_get_sleep(self)
        }
    }
}
