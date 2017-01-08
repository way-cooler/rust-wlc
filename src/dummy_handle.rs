//! Contains dummy definitions for wlc handle types.

use std::fmt::{self, Debug};

#[cfg(not(feature = "dummy"))]
pub use super::dummy_handle::*;

extern crate libc;
use libc::{uintptr_t, c_char, c_void, uint32_t, pid_t};

#[cfg(feature="wlc-wayland")]
use wayland_sys::server::{wl_resource, wl_client};

#[cfg(feature="wlc-wayland")]
use wayland_sys::common::wl_interface;

#[cfg(feature="wlc-wayland")]
use super::wayland::WlcResource;

use super::pointer_to_string;
use super::types::{Geometry, ResizeEdge, Point, Size, ViewType, ViewState};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a handle to a wlc view.
///
pub struct WlcView {
    handle: libc::uint32_t,
    title: String,
    class: String,
    app_id: String,
    pid: pid_t,
    output: WlcOutput,
    geometry: Geometry,
    visible_geometry: Geometry,
    focus: bool,
    mask: u32,
    view_type: ViewType,
    view_state: ViewState,
}

/*impl fmt::Debug for WlcView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WlcView")
            .field("handle", &self.handle as &Debug)
            .field("title", &self.get_title() as &Debug)
            .field("class", &self.get_class() as &Debug)
            .finish()
    }
}

impl fmt::Display for WlcView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = self.get_title();
        if name.is_empty() {
            name = self.get_class();
            if name.is_empty() {
                name = format!("WlcView({handle})", handle=self.0);
            }
        }
        write!(f, "WlcOutput {{ name: {name} }}", name=name)
    }
}

*/
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a handle to a wlc output.
pub struct WlcOutput {
    handle: libc::uint32_t,
    name: String,
    sleep: bool,
    scaling: u32,
    mask: u32,
    resolution: Option<Size>,
    virtual_resolution: Option<Size>,
    views: Vec<WlcView>
}
/*

impl fmt::Debug for WlcOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WlcOutput")
            .field("handle", &self.0 as &Debug)
            .field("name", &self.get_name() as &Debug)
            .field("views", &self.get_views() as &Debug)
            .finish()
    }
}

impl fmt::Display for WlcOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.get_name();
        write!(f, "WlcOutput {{ handle: {handle} name: {name} }}", handle=self.0, name=name)
    }
}
*/


#[cfg(feature="wlc-wayland")]
impl Into<WlcResource> for WlcView {
    fn into(self) -> WlcResource {
        WlcResource::from(self.0)
    }
}

// TODO Implement this
/*
#[cfg(feature = "wlc-wayland")]
impl Into<WlcView> for wl_resource {
fn into(self) -> WlcView {
unsafe { WlcView(wlc_handle_from_wl_surface_resource(&self)) }
    }
}
 */


// TODO Implement this
/*
#[cfg(feature="wlc-wayland")]
impl Into<WlcOutput> for wl_resource {
fn into(self) -> WlcOutput {
unsafe { WlcOutput(wlc_handle_from_wl_output_resource(&self)) }
    }
}
 */

impl WlcOutput {
    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    pub fn as_view(self) -> WlcView {
        WlcView::from(self)
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
    /// unsafe {
    ///     let output = WlcOutput::dummy(0u32);
    ///     let output2 = WlcOutput::dummy(1u32);
    ///     assert!(output < output2);
    ///     assert!(output != output2);
    /// }
    /// ```
    pub unsafe fn dummy(code: u32) -> WlcOutput {
        WlcOutput {
            handle: code as libc::uintptr_t,
            name: "",
            sleep: false,
            scaling: 1,
            mask: 0,
            resolution: None,
            virtual_resolution: None,
            views: Vec::new()
        }
    }

    // TODO Implement mocks for user data

    /// Dummy gets user-specified data.
    ///
    /// Always returns None
    pub unsafe fn get_user_data<T>(&self) -> Option<&mut T> {
        None
    }

    /// Dummy sets user-specified data.
    ///
    /// Always panics w/ `unimplemented!`
    pub unsafe fn set_user_data<T>(&self, data: &T) {
        unimplemented!()
    }

    /// Dummy scheduling for output for rendering next frame.
    ///
    /// If the output was already scheduled, this is
    /// a no-op; if output is currently rendering,
    /// it will render immediately after.
    pub fn schedule_render(self) {
        println!("Dummy call to wlc_output_schedule_render")
    }

    // TODO Mock this

    /// Dummy gets a list of the current outputs.
    ///
    /// Always returns an empty list.
    pub fn list() -> Vec<WlcOutput> {
        Vec::new()
    }

    /// Dummy gets the currently focused output.
    pub fn focused() -> WlcOutput {
        println!("Dummy call to wlc_get_focused_output")
    }

    /// Dummy gets the name of the WlcOutput.
    pub fn get_name(self) -> String {
        self.name
    }

    /// Dummy gets the sleep status of the output.
    pub fn get_sleep(self) -> bool {
        self.sleep
    }

    /// Dummy sets the sleep status of the output.
    pub fn set_sleep(self, sleep: bool) {
        self.sleep = sleep
    }

    /// Dummy gets the output's real resolution. Do not use for coordinate boundary.
    pub fn get_resolution(self) -> Option<Size> {
        self.resolution
    }

    /// Dummy get the virtual resolution. Helpful for getting resolution on high dpi displays.
    pub fn get_virtual_resolution(self) -> Option<Size> {
        self.virtual_resolution
    }

    /// Dummy sets the resolution of the output.
    pub fn set_resolution(self, size: Size, scaling: u32) {
        self.scaling = scaling;
        self.resolution = Some(Size {
            w: size.w * scaling,
            h: size.h * scaling
        })
    }

    /// Dummy gets the scaling for the output.
    pub fn get_scale(self) -> u32 {
        self.scaling
    }

    /// Dummy get views in stack order.
    pub fn get_views(self) -> Vec<WlcView> {
        self.views
    }

    /// Dummy gets the mask of this output
    pub fn get_mask(self) -> u32 {
        self.mask
    }

    /// Dummy sets the mask for this output
    pub fn set_mask(self, mask: u32) {
        self.mask = mask
    }

    /// # Deprecated
    /// This function is equivalent to simply calling get_views
    #[deprecated(since = "0.5.3", note = "please use `get_views`")]
    pub fn get_mutable_views(self) -> Vec<WlcView> {
        self.get_views()
    }

    /// Dummy set the views of a given output.
    ///
    /// Always succeeds
    pub fn set_views(self, views: &[WlcView]) -> Result<(), &'static str> {
        Ok(self.views = views.iter().collect())
    }

    /// Dummy focuses compositor on a specific output.
    ///
    /// Does nothing.
    pub fn focus(output: Option<WlcOutput>) {
        println!("Dummy call to wlc_output_focus");
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
    /// assert!(WlcView::root() == unsafe { WlcView::dummy(0) })
    /// ```
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcView;
    /// unsafe {
    ///     let view = WlcView::dummy(0u32);
    ///     let view2 = WlcView::dummy(1u32);
    ///     assert!(view < view2);
    ///     assert!(view != view2);
    /// }
    /// ```
    pub unsafe fn dummy(code: u32) -> WlcView {
        WlcView {
            handle: code as uintptr_t,
            title: "",
            class: "",
            app_id: "",
            pid: 0 as pid_t,
            output: WlcOutput::dummy(0),
            geometry: Geometry::zero(),
            visible_geometry: Geometry::zero(),
            focus: false,
            mask: 0,
            view_type: ViewType::empty(),
            view_state: ViewState::empty(),
        }
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
        WlcView::dummy(0)
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
    pub fn is_root(self) -> bool {
        self.handle == 0
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
    pub fn is_window(self) -> bool {
        self.handle != 0
    }

    // TODO Mock user data

    /// Dummy gets user-specified data.
    ///
    /// Always returns `None`
    pub unsafe fn get_user_data<T>(&self) -> Option<&mut T> {
        None
    }

    /// Dummy sets user-specified data.
    ///
    /// Always panics w/ `unimplemented!`
    pub unsafe fn set_user_data<T>(&self, data: &T) {
        unimplemented!()
    }

    /// Dummy closes this view.
    ///
    /// Does nothing
    pub fn close(self) {
        println!("Dummy call to wlc_view_close")
    }

    /// Dummy gets the WlcOutput this view is currently part of.
    pub fn get_output(self) -> WlcOutput {
        self.output
    }

    /// Dummy sets the output that the view renders on.
    pub fn set_output(self, output: WlcOutput) {
        self.output = output
    }

    /// Dummy brings this view to focus.
    pub fn focus(self) {
        self.focus = true
    }

    /// Dummy sends the view to the back of the compositor
    ///
    /// Does nothing
    pub fn send_to_back(self) {
        println!("Dummy call to wlc_view_send_to_back")
    }

    /// Dummy sends this view underneath another.
    ///
    /// Does nothing
    pub fn send_below(self, other: WlcView) {
        println!("Dummy call to wlc_view_send_below")
    }

    /// Dummy brings this view above another.
    ///
    /// Does nothing
    pub fn bring_above(self, other: WlcView) {
        println!("Dummy call to wlc_view_bring_above")
    }

    /// Dummy brings this view to the front of the stack
    /// within its WlcOutput.
    ///
    /// Does nothing
    pub fn bring_to_front(self) {
        println!("Dummy call to wlc_view_bring_to_front")
    }

    /// Dummy gets the current visibilty bitmask for the view.
    pub fn get_mask(self) -> u32 {
        self.mask
    }

    /// Dummy sets the visibilty bitmask for the view.
    pub fn set_mask(self, mask: u32) {
        self.mask = mask
    }

    /// Dummy gets the geometry of the view.
    ///
    /// Always returns Some
    pub fn get_geometry(self) -> Option<Geometry> {
        self.geometry
    }

    /// Dummy gets the geometry of the view (that wlc displays).
    pub fn get_visible_geometry(self) -> Geometry {
        self.visible_geometry
    }

    /// Dummy sets the geometry of the view.
    ///
    /// Ignores `edges`
    pub fn set_geometry(self, edges: ResizeEdge, geometry: Geometry) {
        self.geometry = geometry;
    }

    /// Gets the type bitfield of the curent view
    pub fn get_type(self) -> ViewType {
        self.view_type
    }

    /// Dummy set flag in the type field. Toggle indicates whether it is set.
    pub fn set_type(self, view_type: ViewType, toggle: bool) {
        if toggle {
            self.view_type = self.view_type.insert(view_type)
        } else {
            self.view_type = self.view_type.remove(view_type)
        }
    }

    /// Dummy get the current ViewState bitfield.
    pub fn get_state(self) -> ViewState {
        self.view_state
    }

    /// Dummy set ViewState bit. Toggle indicates whether it is set or not.
    pub fn set_state(self, state: ViewState, toggle: bool) {
        if toggle {
            self.view_state = self.view_state.insert(state)
        } else {
            self.view_state = self.view_state.remove(state)
        }
    }

    /// Dummy gets parent view, returns `WlcView::root()` if this view has no parent.
    pub fn get_parent(self) -> WlcView {
        self.parent
    }

    /// Dummy set the parent of this view.
    pub fn set_parent(self, parent: &WlcView) {
        self.parent = *parent
    }

    /// Dummy get the title of the view
    pub fn get_title(self) -> String {
        self.title
    }

    /// Dummy get class (shell surface only).
    pub fn get_class(self) -> String {
        self.class
    }

    /// Dummy get app id (xdg-surface only).
    pub fn get_app_id(self) -> String {
        self.app_id
    }

    /// Get the pid associated with this `WlcView`.
    pub fn get_pid(self) -> pid_t {
        self.pid
    }

    // TODO Mock these functions

    /// Dummy get the wl_client associated with this `WlcView`.
    ///
    /// Always return a null pointer
    #[cfg(feature="wlc-wayland")]
    pub fn get_client(self) -> *mut wl_client {
        c_void as *mut _
    }

    /// Dummy get the wl_role associated with surface that this WLC view refers to.
    ///
    /// Always return a null pointer
    #[cfg(feature="wlc-wayland")]
    pub fn get_role(self) -> *mut wl_resource {
        c_void as *mut _
    }

    #[cfg(feature="wlc-wayland")]
    /// Dummy turns a wl_surface into a wlc view.
    ///
    /// Always returns None
    pub fn view_from_surface(surface: WlcResource,
                             client: *mut wl_client,
                             interface: *const wl_interface,
                             implementation: *const c_void,
                             version: uint32_t,
                             id: uint32_t,
                             userdata: *mut c_void )
                             -> Option<Self> {
        None
    }
}
