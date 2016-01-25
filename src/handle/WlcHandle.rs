use std::ffi;
use std::libc;

pub type WlcHandle = libc::uintptr_t;

extern "C" {
    fn wlc_output_get_name(output: WlcHandle) -> ffi::CString;

    fn wlc_handle_get_user_data(handle: WlcHandle) -> ();

    fn wlc_handle_set_user_data(handle: WlcHandle, userdata: ffi::NulError) -> ();

    fn wlc_output_get_name(output: WlcHandle) -> ffi::CString;

    fn wlc_output_get_sleep(output: WlcHandle) -> bool;

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

    fn wlc_view_set_type(view: WlcHandle, type: ViewType, toggle: bool) -> ();

    fn wlc_view_get_state(view: WlcHandle) -> u32;

    fn wlc_view_set_state(view: WlcHandle, state: ViewState, toggle: bool) ->();

    fn wlc_view_get_parent(view: WlcHandle) -> WlcHandle;

    fn wlc_view_set_parent(view: WlcHandle, parent: WlcHandle) -> ();

    fn wlc_view_get_title(view: WlcHandle) -> ffi::CString;

    fn wlc_view_get_class(view: WlcHandle) -> ffi::CString;

    fn wlc_view_get_app_id(view: WlcHandle) -> ffi::CString;
}
