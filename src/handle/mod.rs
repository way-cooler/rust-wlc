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
/// Represents a handle to a wlc view.
pub struct WlcView(libc::uintptr_t);

#[repr(C)]
pub struct WlcOutput(libc::uintptr_t);

#[link(name = "wlc")]
extern "C" {
    fn wlc_output_get_name(output: &WlcOutput) -> *const c_char;

    //fn wlc_handle_get_user_data(handle: WlcHandle) -> ();

    // TODO need representation of userdata
    //fn wlc_handle_set_user_data(handle: WlcHandle, userdata: ?????) -> ();

    fn wlc_output_get_sleep(output: &WlcOutput) -> bool;

    fn wlc_output_set_sleep(output: &WlcOutput, sleep: bool) -> ();

    fn wlc_output_get_resolution(output: &WlcOutput) -> Size;

    fn wlc_output_set_resolution(output: &WlcOutput, resolution: Size) -> ();

    fn wlc_output_get_mask(output: &WlcOutput) -> u32;

    fn wlc_output_set_mask(output: &WlcOutput, mask: u32) -> ();

    // TODO tricky definition here
    //fn wlc_output_get_pixels(output: WlcHandle) -> ();

    fn wlc_output_get_views(output: &WlcOutput, out_memb: libc::size_t) -> *const WlcView;

    fn  wlc_output_get_mutable_views(output: &WlcOutput, out_memb: libc::size_t) -> *mut WlcOutput;

    fn wlc_output_set_views(output: &WlcOutput, views: *const WlcView, memb: libc::size_t) -> bool;

    fn wlc_output_focus(output: &WlcOutput);

    fn wlc_view_close(view: &WlcView);

    fn wlc_view_get_output(view: &WlcView) -> WlcOutput;

    // "set output. Alternatively you can use wlc_output_set_views"
    fn wlc_view_set_output(view: &WlcView, output: WlcOutput);

    fn wlc_view_send_to_back(view: &WlcView);

    fn wlc_view_send_below(view: &WlcView, other: &WlcView);

    fn wlc_view_bring_above(view: &WlcView, other: &WlcView);

    fn wlc_view_bring_to_front(view: &WlcView);

    fn wlc_view_get_mask(view: &WlcView) -> u32;

    fn wlc_view_set_mask(view: &WlcView, mask: u32);

    fn wlc_view_get_geometry(view: &WlcView) -> Geometry;

    fn wlc_view_set_geometry(view: &WlcView, edges: u32, geo: Geometry);

    fn wlc_view_get_type(view: &WlcView) -> u32;

    fn wlc_view_set_type(view: &WlcView, view_type: ViewType, toggle: bool);

    fn wlc_view_get_state(view: &WlcView) -> u32;

    fn wlc_view_set_state(view: &WlcView, state: ViewState, toggle: bool);

    fn wlc_view_get_parent(view: &WlcView) -> WlcView;

    fn wlc_view_set_parent(view: &WlcView, parent: &WlcView);

    fn wlc_view_get_title(view: &WlcView) -> *const c_char;

    fn wlc_view_get_class(view: &WlcView) -> *const c_char;

    fn wlc_view_get_app_id(view: &WlcView) -> *const c_char;
}

impl WlcOutput {
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
