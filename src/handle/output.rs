//! Contains definitions for wlc handle types.
//!
//! # Implementations
//! - **Debug**: pointer-prints the underlying `uintptr_t` handle
//! - **Eq, Ord**: compare the underlying `uintptr_t` handle
//! - **Clone**: View handles can safely be cloned.

use libc::{uintptr_t, c_char, c_void, size_t};

use super::{WlcView, WlcOutput};
use super::{wlc_handle_get_user_data, wlc_handle_set_user_data};
use super::super::pointer_to_string;
use types::Size;

#[link(name = "wlc")]
extern "C" {
    fn wlc_get_outputs(memb: *mut size_t) -> *const uintptr_t;

    fn wlc_get_focused_output() -> uintptr_t;

    fn wlc_output_get_name(output: uintptr_t) -> *const c_char;

    // Defined in wlc-render.h
    fn wlc_output_schedule_render(output: uintptr_t);

    fn wlc_output_get_sleep(output: uintptr_t) -> bool;

    fn wlc_output_set_sleep(output: uintptr_t, sleep: bool);

    fn wlc_output_get_resolution(output: uintptr_t) -> *const Size;

    fn wlc_output_set_resolution(output: uintptr_t, resolution: *const Size);

    fn wlc_output_get_mask(output: uintptr_t) -> u32;

    fn wlc_output_set_mask(output: uintptr_t, mask: u32);

    // TODO tricky definition here
    //fn wlc_output_get_pixels(output: WlcHandle) -> ();

    fn wlc_output_get_views(output: uintptr_t,
                            out_memb: *mut size_t) -> *const uintptr_t;

    fn wlc_output_set_views(output: uintptr_t, views: *const uintptr_t, memb: size_t) -> bool;

    fn wlc_output_focus(output: uintptr_t);
}

impl From<WlcView> for WlcOutput {
    fn from(view: WlcView) -> Self {
        WlcOutput(view.0)
    }
}

impl WlcOutput {

    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    pub fn as_view(self) -> WlcView {
        return WlcView::from(self)
    }

    /// Create a dummy WlcOutput for testing purposes.
    ///
    /// # Unsafety
    /// The following operations on a dummy WlcOutput will cause crashes:
    ///
    /// - `WlcOutput::focused` when wlc is not running
    /// - `WlcOutput::list` when wlc is not running
    /// - `WlcOutput::set_resolution` on a dummy output
    ///
    /// In addition, `WlcOutput::set_views` will return an error.
    ///
    /// All other methods can be used on dummy outputs.
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcOutput;
    /// let output = WlcOutput::dummy(0u32);
    /// let output2 = WlcOutput::dummy(1u32);
    /// assert!(output < output2);
    /// assert!(output != output2);
    /// ```
    pub fn dummy(code: u32) -> WlcOutput {
        WlcOutput(code as uintptr_t)
    }

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn get_user_data<T>(&self) -> &mut T {
        let raw_data = wlc_handle_get_user_data(self.0);
        return &mut *(raw_data as *mut T);
    }

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn set_user_data<T>(&self, data: &T) {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        wlc_handle_set_user_data(self.0, data_ptr);
    }

    /// Schedules output for rendering next frame.
    ///
    /// If the output was already scheduled, this is
    /// a no-op; if output is currently rendering,
    /// it will render immediately after.
    pub fn schedule_render(&self) {
        unsafe { wlc_output_schedule_render(self.0) };
    }

    /// Gets a list of the current outputs.
    ///
    /// # Safety
    /// This function will crash the program if run when wlc is not running.
    pub fn list() -> Vec<WlcOutput> {
        unsafe {
            let mut out_memb: size_t = 0;
            let outputs = wlc_get_outputs(&mut out_memb);
            if outputs.is_null() {
                return Vec::new();
            }
            let mut result = Vec::with_capacity(out_memb);
            for index in (0 as isize) .. (out_memb as isize) {
                result.push(WlcOutput(*(outputs.offset(index))));
            }
            result
        }
    }

    /// Gets the currently focused output.
    ///
    /// # Safety
    /// This function will crash the program if run when wlc is not running.
    pub fn focused() -> WlcOutput {
        unsafe { WlcOutput(wlc_get_focused_output()) }
    }

    /// Gets the name of the WlcOutput.
    ///
    /// Names are usually assigned in the format WLC-n,
    /// where the first output is WLC-1.
    pub fn get_name(&self) -> String {
        let name: *const i8;
        unsafe {
            name = wlc_output_get_name(self.0);
            pointer_to_string(name)
        }
    }

    /// Gets the sleep status of the output.
    ///
    /// Returns `true` if the monitor is sleeping,
    /// such as having been set with `set_sleep`.
    pub fn get_sleep(&self) -> bool {
        unsafe { wlc_output_get_sleep(self.0) }
    }

    /// Sets the sleep status of the output.
    pub fn set_sleep(&self, sleep: bool) {
        unsafe { wlc_output_set_sleep(self.0, sleep); }
    }

    /// Gets the output resolution in pixels.
    pub fn get_resolution(&self) -> &Size {
        unsafe { &*wlc_output_get_resolution(self.0) }
    }

    /// Sets the resolution of the output.
    ///
    /// # Safety
    /// This method will crash the program if use when wlc is not running.
    pub fn set_resolution(&self, size: Size) {
        unsafe { wlc_output_set_resolution(self.0, &size); }
    }

    /// Get views in stack order.
    ///
    /// This is mainly useful for wm's who need another view stack for inplace sorting.
    /// For example tiling wms, may want to use this to keep their tiling order separated
    /// from floating order.
    /// This handles `wlc_output_get_views` and `wlc_output_get_mutable_views`.
    pub fn get_views(&self) -> Vec<WlcView> {
        unsafe {
            let mut out_memb: size_t = 0;
            let views = wlc_output_get_views(self.0, &mut out_memb);
            if views.is_null() {
                return Vec::new();
            }
            let mut result = Vec::with_capacity(out_memb);

            for index in (0 as isize) .. (out_memb as isize) {
                  result.push(WlcView(*(views.offset(index))));
            }
            return result;
        }
    }

    /// Gets the mask of this output
    pub fn get_mask(&self) -> u32 {
        unsafe { wlc_output_get_mask(self.0) }
    }

    /// Sets the mask for this output
    pub fn set_mask(&self, mask: u32) {
        unsafe { wlc_output_set_mask(self.0, mask) }
    }

    /// # Deprecated
    /// This function is equivalent to simply calling get_views
    pub fn get_mutable_views(&self) -> Vec<WlcView> {
        self.get_views()
    }

    /// Attempts to set the views of a given output.
    ///
    /// Returns success if operation succeeded. An error will be returned
    /// if something went wrong or if wlc isn't running.
    pub fn set_views(&self, views: &mut Vec<&WlcView>) -> Result<(), &'static str> {
            let view_len = views.len() as size_t;
            let view_vals: Vec<uintptr_t> = views.into_iter().map(|v| v.0).collect();
            let const_views = view_vals.as_ptr();
        unsafe {
            match wlc_output_set_views(self.0, const_views, view_len) {
                true => Ok(()),
                false => Err("Could not set views on output"),
            }
        }
    }

    /// Focuses compositor on a specific output.
    ///
    /// Pass in Option::None for no focus.
    pub fn focus(output: Option<&WlcOutput>) {
        unsafe {
            match output {
                Some(output) => wlc_output_focus(output.0),
                None => wlc_output_focus(0)
            }
        }
    }
}
