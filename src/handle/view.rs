//! Contains definitions for wlc handle types.
//!
//! # Implementations
//! - **Debug**: pointer-prints the underlying `uintptr_t` handle
//! - **Eq, Ord**: compare the underlying `uintptr_t` handle
//! - **Clone**: View handles can safely be cloned.

use libc::{uintptr_t, c_char, c_void};

use super::{WlcOutput, WlcView};
use super::{wlc_handle_get_user_data, wlc_handle_set_user_data};
use super::super::pointer_to_string;
use types::{Geometry, ResizeEdge, Point, Size, ViewType, ViewState};


#[link(name = "wlc")]
extern "C" {
    // View API

    fn wlc_view_focus(view: uintptr_t);

    fn wlc_view_close(view: uintptr_t);

    // View -> Output
    fn wlc_view_get_output(view: uintptr_t) -> uintptr_t;

    // "set output. Alternatively you can use wlc_output_set_views"
    fn wlc_view_set_output(view: uintptr_t, output: uintptr_t);

    fn wlc_view_send_to_back(view: uintptr_t);

    fn wlc_view_send_below(view: uintptr_t, other: uintptr_t);

    fn wlc_view_bring_above(view: uintptr_t, other: uintptr_t);

    fn wlc_view_bring_to_front(view: uintptr_t);

    fn wlc_view_get_mask(view: uintptr_t) -> u32;

    fn wlc_view_set_mask(view: uintptr_t, mask: u32);

    fn wlc_view_get_geometry(view: uintptr_t) -> *const Geometry;

    fn wlc_view_get_visible_geometry(view: uintptr_t, geo: *mut Geometry);

    fn wlc_view_set_geometry(view: uintptr_t, edges: u32, geo: *const Geometry);

    fn wlc_view_get_type(view: uintptr_t) -> ViewType;

    fn wlc_view_set_type(view: uintptr_t, view_type: ViewType, toggle: bool);

    fn wlc_view_get_state(view: uintptr_t) -> ViewState;

    fn wlc_view_set_state(view: uintptr_t, state: ViewState, toggle: bool);

    // Parent is Option<View>
    fn wlc_view_get_parent(view: uintptr_t) -> uintptr_t;

    // Parent is Option<View>
    fn wlc_view_set_parent(view: uintptr_t, parent: uintptr_t);

    fn wlc_view_get_title(view: uintptr_t) -> *const c_char;

    fn wlc_view_get_class(view: uintptr_t) -> *const c_char;

    fn wlc_view_get_app_id(view: uintptr_t) -> *const c_char;
}

impl From<WlcOutput> for WlcView {
    fn from(output: WlcOutput) -> Self {
        WlcView(output.0)
    }
}

impl WlcView {

    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    pub fn as_output(self) -> WlcOutput {
        WlcOutput::from(self)
    }

    /// Create a dummy WlcView for testing purposes.
    ///
    /// # Unsafety
    /// The following methods on views may crash the program:
    ///
    /// - `WlcView::focus` if wlc is not running
    /// - `WlcView::send_to_back` if wlc is not running
    /// - `WlcView::send_below` if wlc is not running
    /// - `WlcView::bring_above` if wlc is not running
    /// - `WlcView::bring_to_font` if wlc is not running
    ///
    /// All other methods can be used on dummy views.
    ///
    /// # Note
    /// `WlcView::root()` is equivalent to `WlcView::dummy(0)`.
    ///
    /// ```rust
    /// # use rustwlc::WlcView;
    /// assert!(WlcView::root() == WlcView::dummy(0))
    /// ```
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcView;
    /// let view = WlcView::dummy(0u32);
    /// let view2 = WlcView::dummy(1u32);
    /// assert!(view < view2);
    /// assert!(view != view2);
    /// ```
    pub fn dummy(code: u32) -> WlcView {
        WlcView(code as uintptr_t)
    }

    /// Returns a reference to the root window (desktop background).
    ///
    /// # Example
    /// ```
    /// # use rustwlc::WlcView;
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    pub fn root() -> WlcView {
        WlcView(0)
    }

    /// Whether this view is the root window (desktop background).
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcView;
    /// # // This example can be run because WlcView::root() does not interact with wlc
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    #[inline]
    pub fn is_root(&self) -> bool {
        self.0 == 0
    }

    /// Whether this view is not the root window (desktop background).
    ///
    /// # Usage
    /// A convenience method, the opposite of `view.is_root()`.
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcView;
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// assert!(!view.is_window());
    /// ```
    #[inline]
    pub fn is_window(&self) -> bool {
        self.0 != 0
    }

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly does a conversion
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
    /// for raw C data. This function will internaly does a conversion
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

    /// Closes this view.
    ///
    /// For the main windows of most programs, this should close the program where applicable.
    ///
    /// # Behavior
    /// This function will not do anything if `view.is_root()`.
    pub fn close(&self) {
        if self.is_root() { return };
        unsafe { wlc_view_close(self.0); }
    }

    /// Gets the WlcOutput this view is currently part of.
    pub fn get_output(&self) -> WlcOutput {
        unsafe { WlcOutput(wlc_view_get_output(self.0)) }
    }

    /// Sets the output that the view renders on.
    ///
    /// This may not be supported by wlc at this time.
    pub fn set_output(&self, output: &WlcOutput) {
        unsafe { wlc_view_set_output(self.0, output.0) }
    }

    /// Brings this view to focus.
    ///
    /// Can be called on `WlcView::root()` to lose all focus.
    pub fn focus(&self) {
        unsafe { wlc_view_focus(self.0); }
    }

    /// Sends the view to the back of the compositor
    pub fn send_to_back(&self) {
        unsafe { wlc_view_send_to_back(self.0); }
    }

    /// Sends this view underneath another.
    pub fn send_below(&self, other: &WlcView) {
        unsafe { wlc_view_send_below(self.0, other.0); }
    }

    /// Brings this view above another.
    pub fn bring_above(&self, other: &WlcView) {
        unsafe { wlc_view_bring_above(self.0, other.0); }
    }

    /// Brings this view to the front of the stack
    /// within its WlcOutput.
    pub fn bring_to_front(&self) {
        unsafe { wlc_view_bring_to_front(self.0); }
    }

    // TODO Get masks enum working properly
    /// Gets the current visibilty bitmask for the view.
    pub fn get_mask(&self) -> u32 {
        unsafe { wlc_view_get_mask(self.0) }
    }

    // TODO Get masks enum working properly
    /// Sets the visibilty bitmask for the view.
    pub fn set_mask(&self, mask: u32) {
        unsafe { wlc_view_set_mask(self.0, mask); }
    }

    /// Gets the geometry of the view.
    pub fn get_geometry(&self) -> Option<&Geometry> {
        unsafe {
            let geometry = wlc_view_get_geometry(self.0);
            if geometry.is_null() {
                None
            } else {
                Some(&*geometry)
            }
        }
    }

    /// Gets the geometry of the view (that wlc displays).
    pub fn get_visible_geometry(&self) -> Geometry {
        let mut geo = Geometry { origin: Point { x: 0, y: 0}, size: Size { w: 0, h: 0 }};
        unsafe {
            wlc_view_get_visible_geometry(self.0, &mut geo);
        }
        return geo;
    }

    /// Sets the geometry of the view.
    ///
    /// Set edges if geometry is caused by interactive resize.
    pub fn set_geometry(&self, edges: ResizeEdge, geometry: &Geometry) {
        unsafe { wlc_view_set_geometry(self.0, edges.bits(), geometry as *const Geometry); }
    }

    /// Gets the type bitfield of the curent view
    pub fn get_type(&self) -> ViewType {
        unsafe { wlc_view_get_type(self.0) }
    }

    /// Set flag in the type field. Toggle indicates whether it is set.
    pub fn set_type(&self, view_type: ViewType, toggle: bool) {
        unsafe { wlc_view_set_type(self.0, view_type, toggle); }
    }

    // TODO get bitflags enums
    /// Get the current ViewState bitfield.
    pub fn get_state(&self) -> ViewState {
        unsafe { wlc_view_get_state(self.0) }
    }

    /// Set ViewState bit. Toggle indicates whether it is set or not.
    pub fn set_state(&self, state: ViewState, toggle: bool) {
        unsafe { wlc_view_set_state(self.0, state, toggle); }
    }

    /// Gets parent view, returns `WlcView::root()` if this view has no parent.
    pub fn get_parent(&self) -> WlcView {
        unsafe { WlcView(wlc_view_get_parent(self.0)) }
    }

    /// Set the parent of this view.
    ///
    /// Call with `WlcView::root()` to make its parent the root window.
    pub fn set_parent(&self, parent: &WlcView) {
        unsafe { wlc_view_set_parent(self.0, parent.0); }
    }

    /// Get the title of the view
    pub fn get_title(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_title(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                    pointer_to_string(chars)
            }
        }
    }

    /// Get class (shell surface only).
    pub fn get_class(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_class(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                pointer_to_string(chars)
            }
        }
    }

    /// Get app id (xdg-surface only).
    pub fn get_app_id(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_app_id(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                pointer_to_string(chars)
            }
        }
    }
}

/// Functions defined in wlc-wayland.h
#[cfg(feature = "wlc-wayland")]
#[link(name = "wlc")]
extern "C" {
    fn wlc_handle_from_wl_surface_resource(resource: *const wl_resource) -> uintptr_t;
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
    fn wlc_view_get_role(view: uintptr_t) -> *const wl_resource;
}

#[cfg(feature = "wlc-wayland")]
use wayland_sys::{wl_resource, wl_client, wl_interface};

#[cfg(feature = "wlc-wayland")]
impl WlcView {
    /// ## Requires `wlc-wayland` feature
    ///
    /// Gets a WlcView from the given Wayland surface resource
    pub fn from_wl_surface_resource(resource: WlcResource) -> WlcView {
        unsafe {
            WlcView(wlc_handle_from_wl_surface_resource(&resource))
        }
    }

    /// ## Requires `wlc-wayland` feature
    ///
    /// Gets a `WlcView` from the given Wayland surface parameters.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers for
    /// raw C data. This function internally does a conversion between
    /// the input `T` and `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As such, usage
    /// of these functions requires an understanding of the wayland APIs that
    /// the parameters should follow. Please review Wayland and `wayland-sys`
    /// docs, in addition to wlc's, to understand usage of this function.
    pub unsafe fn from_wl_surface<I, D>(surface: wl_resource, client: wl_client,
                                     interface: wl_interface, implementation: I,
                                version: u32, id: u32, userdata: D) -> WlcView {
        let impl_potr: *const c_void = interface as *const _ as *const c_void;
        let data_ptr: *const c_void = userdata  as *const _ as *const c_void;
        WlcView(wlc_view_from_surface(surface, client, interface, impl_ptr,
                                      version, id, data_ptr))
    }

    /// ## Requires `wlc-wayland` feature
    ///
    /// Gets a surface `WlcResource` from the given view
    pub fn get_surface(parent: WlcResource) -> WlcResource {
        unsafe { WlcResource::from(wlc_view_get_surface(self.0)) }
    }

    /// ## Require `wlc-wayland` feature
    ///
    /// Gets the Wayland role of the view's surface
    pub fn get_role() -> WlcResource {
        unsafe { WlcResource::from(wlc_view_get_role(self.0)) }
    }
}
