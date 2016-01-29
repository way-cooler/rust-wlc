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
#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a handle to a wlc view.
pub struct WlcView(libc::uintptr_t);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a handle to a wlc output
pub struct WlcOutput(libc::uintptr_t);

#[link(name = "wlc")]
extern "C" {
    fn wlc_output_get_name(output: &WlcOutput) -> *const c_char;

    //fn wlc_handle_get_user_data(handle: WlcHandle) -> ();

    // TODO need representation of userdata
    //fn wlc_handle_set_user_data(handle: WlcHandle, userdata: ?????) -> ();

    fn wlc_output_get_sleep(output: &WlcOutput) -> bool;

    fn wlc_output_set_sleep(output: &WlcOutput, sleep: bool);

    fn wlc_output_get_resolution(output: &WlcOutput) -> Size;

    fn wlc_output_set_resolution(output: &WlcOutput, resolution: Size);

    fn wlc_output_get_mask(output: &WlcOutput) -> u32;

    fn wlc_output_set_mask(output: &WlcOutput, mask: u32);

    // TODO tricky definition here
    //fn wlc_output_get_pixels(output: WlcHandle) -> ();

    fn wlc_output_get_views(output: &WlcOutput, out_memb: *mut libc::size_t) -> *const WlcView;

    fn  wlc_output_get_mutable_views(output: &WlcOutput, out_memb: *mut libc::size_t) -> *mut WlcView;

    fn wlc_output_set_views(output: &WlcOutput, views: *const WlcView, memb: libc::size_t) -> bool;

    fn wlc_output_focus(output: Option<&WlcOutput>);

    fn wlc_view_focus(view: Option<&WlcView>);

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

    fn wlc_view_get_parent(view: &WlcView) -> Option<WlcView>;

    fn wlc_view_set_parent(view: &WlcView, parent: &WlcView);

    fn wlc_view_get_title(view: &WlcView) -> *const c_char;

    fn wlc_view_get_class(view: &WlcView) -> *const c_char;

    fn wlc_view_get_app_id(view: &WlcView) -> *const c_char;
}

impl WlcOutput {
    /// wlc internally uses one type, wlc_handle to
    /// represent views and outputs. It has functions
    /// with the signature wlc_output_... and wlc_view_...
    /// which we have mapped to the WlcView and WlcOutput
    /// structs through impl methods.
    /// If we got one of these methods wrong, or wlc
    /// has behavior that requires using a wlc_view_... method
    /// on something obtained as a wlc_get_output... for example,
    /// please feel free to use one of these conversion methods
    /// (WlcView::from_output, WlcOutput::from_view) to convert
    /// the handle. The only difference between the two is which
    /// unsafe wlc_{output, view}_... functions we wrap.
    pub fn as_view(self) -> WlcView {
        return WlcView::from_output(self)
    }

    /// wlc internally uses one type, wlc_handle to
    /// represent views and outputs. It has functions
    /// with the signature wlc_output_... and wlc_view_...
    /// which we have mapped to the WlcView and WlcOutput
    /// structs through impl methods.
    /// If we got one of these methods wrong, or wlc
    /// has behavior that requires using a wlc_view_... method
    /// on something obtained as a wlc_get_output... for example,
    /// please feel free to use one of these conversion methods
    /// (WlcView::from_output, WlcOutput::from_view) to convert
    /// the handle. The only difference between the two is which
    /// wlc_{output, view}_... functions from wlc we wrap.
    pub fn from_view(view: WlcView) -> WlcOutput {
        WlcOutput(view.0)
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let name = wlc_output_get_name(self);
            pointer_to_string(name)
        }
    }

    pub fn get_sleep(&self) -> bool {
        unsafe {
            wlc_output_get_sleep(self)
        }
    }

    pub fn set_sleep(&self, sleep: bool) {
        unsafe {
            wlc_output_set_sleep(self, sleep);
        }
    }

    /// Gets the output resolution.
    /// This is not measured in pixels.
    pub fn get_resolution(&self) -> Size {
        unsafe {
            wlc_output_get_resolution(self)
        }
    }

    /// Sets the resolution of the WlcOutput
    pub fn set_resolution(&self, size: Size) {
        unsafe {
            wlc_output_set_resolution(self, size);
        }
    }

    /// Get views in stack order. Returned array is a direct reference,
    /// careful when moving and destroying handles.
    pub fn get_views(&self) -> Vec<&WlcView> {
        unsafe {
            let mut out_memb: libc::size_t = 0;
            let views = wlc_output_get_views(self, &mut out_memb);
            let mut result = Vec::with_capacity(out_memb);
            (0isize .. out_memb as isize).map(|index|
                     result.push(&*(views.offset(index))));
            return result;
        }
    }

    /// Get mutable views in creation order. Returned array is a direct reference,
    /// careful when moving and destroying handles.
    /// This is mainly useful for wm's who need another view stack for inplace sorting.
    /// For example tiling wms, may want to use this to keep their tiling order separated
    /// from floating order.
    pub fn get_mutable_views(&self) -> Vec<WlcView> {
        unsafe {
            let mut out_memb: libc::size_t = 0;
            let mut views = wlc_output_get_mutable_views(self, &mut out_memb);
            return Vec::from_raw_parts(views, out_memb, out_memb);
                //.into_iter().map(|view| )
        }
    }

    /// Attempts to set the views of a given output.
    /// Returns true if the operation succeeded.
    pub fn set_views(&self, views: &mut Vec<&WlcView>) -> bool {
        unsafe {
            let view_len = views.len() as libc::size_t;
            let mut const_views = views.as_mut_ptr() as *const WlcView;
            return wlc_output_set_views(self, const_views, view_len);
        }
    }

    /// Focuses this output on a specific view.
    /// Can also use view.focus().
    /// Pass in Option::None for no focus.
    pub fn focus(view: Option<&WlcOutput>) {
        unsafe { wlc_output_focus(view); }
    }
}

impl WlcView {

    /// wlc internally uses one type, wlc_handle to
    /// represent views and outputs. It has functions
    /// with the signature wlc_output_... and wlc_view_...
    /// which we have mapped to the WlcView and WlcOutput
    /// structs through impl methods.
    /// If we got one of these methods wrong, or wlc
    /// has behavior that requires using a wlc_view_... method
    /// on something obtained as a wlc_get_output... for example,
    /// please feel free to use one of these conversion methods
    /// (WlcView::from_output, WlcOutput::from_view) to convert
    /// the handle. The only difference between the two is which
    /// unsafe wlc_{output, view}_... functions we wrap.
    pub fn as_output(self) -> WlcOutput {
        WlcOutput::from_view(self)
    }

    /// wlc internally uses one type, wlc_handle to
    /// represent views and outputs. It has functions
    /// with the signature wlc_output_... and wlc_view_...
    /// which we have mapped to the WlcView and WlcOutput
    /// structs through impl methods.
    /// If we got one of these methods wrong, or wlc
    /// has behavior that requires using a wlc_view_... method
    /// on something obtained as a wlc_get_output... for example,
    /// please feel free to use one of these conversion methods
    /// (WlcView::from_output, WlcOutput::from_view) to convert
    /// the handle. The only difference between the two is which
    /// unsafe wlc_{output, view}_... functions we wrap.
    pub fn from_output(output: WlcOutput) -> WlcView {
        WlcView(output.0)
    }

    /// Closes this WlcView
    pub fn close(&self) {
        unsafe { wlc_view_close(self); }
    }

    /// Gets the WlcOutput this view is currently part of
    pub fn get_output(&self) -> WlcOutput {
        unsafe { wlc_view_get_output(self) }
    }

    /// Brings this view to focus.
    /// Pass in Option::None for no focus.
    /// Can also use WlcOutput::focus()
    pub fn focus_on(view: Option<&WlcView>) {
        unsafe { wlc_view_focus(view); }
    }

    /// Brings this view to focus.
    /// To un-set focus to nothing, call WlcView::focus_on(None)
    pub fn focus(&self) {
        WlcView::focus_on(Some(self));
    }

    /// Sends the view to the back of the compositor
    pub fn send_to_back(&self) {
        unsafe { wlc_view_send_to_back(self); }
    }

    /// Sends this view underneath another.
    pub fn send_below(&self, other: &WlcView) {
        unsafe { wlc_view_send_below(self, other); }
    }

    /// Brings this view above another.
    pub fn bring_above(&self, other: &WlcView) {
        unsafe { wlc_view_bring_above(self, other); }
    }

    /// Brings this view to the front of the stack
    /// within its WlcOutput.
    pub fn bring_to_front(&self) {
        unsafe { wlc_view_bring_to_front(self); }
    }

    // TODO Get masks enum working properly
    /// Gets the current visibilty bitmask for the view.
    pub fn get_mask(&self) -> u32 {
        unsafe { wlc_view_get_mask(self) }
    }

    // TODO Get masks enum working properly
    /// Sets the visibilty bitmask for the view.
    pub fn set_mask(&self, mask: u32) {
        unsafe { wlc_view_set_mask(self, mask); }
    }

    /// Gets the geometry of the current view
    pub fn get_geometry(&self) -> Geometry {
        unsafe { wlc_view_get_geometry(self) }
    }

    /// Sets geometry. Set edges if geometry is caused by interactive resize.
    pub fn set_geometry(&self, edges: u32, geometry: Geometry) {
        unsafe { wlc_view_set_geometry(self, edges, geometry); }
    }

    // TODO Return ViewType enum value.
    /// Gets the type bitfield of the curent view
    pub fn get_type(&self) -> u32 {
        unsafe { wlc_view_get_type(self) }
    }

    /// Set flag in the type field. Toggle indicates whether it is set.
    pub fn set_type(&self, view_type: ViewType, toggle: bool) {
        unsafe { wlc_view_set_type(self, view_type, toggle); }
    }

    // TODO get bitflags enums
    /// Get the current ViewState bitfield.
    pub fn get_state(&self) -> u32 {
        unsafe { wlc_view_get_state(self) }
    }

    /// Set ViewState bit. Toggle indicates whether it is set or not.
    pub fn set_state(&self, state: ViewState, toggle: bool) {
        unsafe { wlc_view_set_state(self, state, toggle); }
    }

    /// Gets parent view, returns None if this view has no parent.
    pub fn get_parent(&self) -> Option<WlcView> {
        unsafe { wlc_view_get_parent(self) }
    }

    /// Set the parent of this view.
    pub fn set_parent(&self, parent: &WlcView) {
        unsafe { wlc_view_set_parent(self, parent); }
    }

    /// Get the title of the view
    pub fn get_title(&self) -> String {
        unsafe {
            let chars = wlc_view_get_title(self);
            return pointer_to_string(chars);
        }
    }

    /// Get class (shell surface only).
    pub fn get_class(&self) -> String {
        unsafe {
            let chars = wlc_view_get_class(self);
            return pointer_to_string(chars);
        }
    }

    /// Get app id (xdg-surface only)
    pub fn get_app_id(&self) -> String {
        unsafe {
            let chars = wlc_view_get_app_id(self);
            return pointer_to_string(chars);
        }
    }
}
