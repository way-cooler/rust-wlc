//! types/interface
//! Contains all structs used for initializing wlc.
//! You will only need this module for invoking
//! `rustwlc::init`.

extern crate libc;

use std::option::Option;

use super::types::*;
use super::handle::{WlcOutput, WlcView};

/// Represents the wlc callback interface.
/// wlc initialization involves registering
/// a series of callbacks to the library
/// using this interface struct.
#[repr(C)]
pub struct WlcInterface {
    pub output: OutputInterface,
    pub view: ViewInterface,
    pub keyboard: KeyboardInterface,
    pub pointer: PointerInterface,
    pub touch: TouchInterface,
    pub compositor: CompositorInterface,
    pub input: InputInterface
}

/// Represents window callbacks
#[repr(C)]
pub struct OutputInterface {
    /// Output was created
    pub created: Option<extern "C" fn(handle: WlcOutput) -> bool>,
    /// Output lost or destroyed
    pub destroyed: Option<extern "C" fn(handle: WlcOutput)>,
    pub focus: Option<extern "C" fn(handle: WlcOutput, focused: bool)>,
    /// Output resolution changed
    pub resolution: Option<extern "C" fn(handle: WlcOutput, old_size: &Size, new_size: &Size)>,
    pub render: OutputRenderInterface,
}

/// Represents rendering callbacks for outputs
#[repr(C)]
pub struct OutputRenderInterface {
    /// Pre render hook
    pub pre: Option<extern "C" fn(handle: WlcOutput)>,
    /// Post render hook
    pub post: Option<extern "C" fn(handle: WlcOutput)>,
}

/// Represents window viewing callbacks
#[repr(C)]
pub struct ViewInterface {
    /// View was created. Return false if you want to destroy the view
    /// (e.g. failed to allocated data related to the view)
    pub created: Option<extern "C" fn(handle: WlcView) -> bool>,
    /// View was destroyed
    pub destroyed: Option<extern "C" fn(handle: WlcView)>,
    /// View lost or got focus
    pub focus: Option<extern "C" fn(handle: WlcView, focused: bool)>,
    /// View was moved to to output
    pub move_to_output: Option<extern "C" fn(current: WlcView, from_output: WlcView, to_output: WlcView)>,
    pub request: RequestInterface,
}

/// Represents window rendering callbacks
#[repr(C)]
pub struct RequestInterface {
    /// Request to set given geometry to view. Apply using
    /// wlc_view_set_geometry (handle.set_geometry) to agree.
    pub geometry: Option<extern "C" fn(handle: WlcView, geometry: &Geometry)>,
    /// Request to disable or enable the given state for a view.
    /// Apply using wlc_view_set_state to agree.
    pub state: Option<extern "C" fn(current: WlcView, state: ViewState, handled: bool)>,
    /// View requests to move itself. Start an interactive move to agree.
    pub move_: Option<extern "C" fn(handle: WlcView, destination: &Point)>,
    /// Request to resize itself with the given edges.
    /// Start and interactive move to agree
    pub resize: Option<extern "C" fn(handle: WlcView, edge: ResizeEdge, location: &Point)>,

    /// View rendering hooks
    pub render: ViewRenderInterface,
}

/// Represents rendering callbacks for views
#[repr(C)]
pub struct ViewRenderInterface {
    pub pre: Option<extern "C" fn(view: WlcView)>,
    pub post: Option<extern "C" fn(view: WlcView)>
}

/// Represents keyboard press callbacks
#[repr(C)]
pub struct KeyboardInterface {
    /// Key event was triggered, handle will be None if there was no focus
    /// Return true to prevent sending the event to clients.
    pub key: Option<extern "C" fn(view: WlcView, time: u32, mods: &KeyboardModifiers, key: u32, state: KeyState) -> bool>,
}

/// Represents mouse input callbacks
#[repr(C)]
pub struct PointerInterface {
    /// Button event was triggered, view will be None if there was no
    /// focus. Return true to prevent sending the event to clients.
    pub button: Option<extern "C" fn(hande: WlcView, time: u32, mods: &KeyboardModifiers, button: u32, state: ButtonState, point: &Point) -> bool>,

    /// Scroll event was triggered, view handle will be None if there was
    /// no focus. Return true to prevent sending the event to clients.
    pub scroll: Option<extern "C" fn(handle: WlcView, time: u32, mods: &KeyboardModifiers, axis: ScrollAxis, amount: [u64; 2]) -> bool>,
    /// Mouse was moved, view will be none if there was no focus.
    /// Use wlc_pointer_set_position to agree. Return true to prevent
    /// sending event to clients.
    pub motion: Option<extern "C" fn(heights: WlcView, time: u32, point: &Point) -> bool>,
}

/// Represents touchscreen callbacks
#[repr(C)]
pub struct TouchInterface {
    /// Screen was touched, handle will be None if there was no focus.
    /// Return true to prevent sending the event to clients.
    pub touch: Option<extern "C" fn(handle: WlcView, time: u32, mods: &KeyboardModifiers, touch: TouchType, slot: i32, point: &Point) -> bool>,
}

/// Represents a callback for initializing the callback
#[repr(C)]
pub struct CompositorInterface {
    /// Compositor is ready to accept clients.
    pub ready: Option<extern "C" fn()>
}

/// Represents experimencallbacks for libinput events
#[repr(C)]
pub struct InputInterface {
    pub created: Option<extern "C" fn(device: &LibinputDevice) -> bool>,
    pub destroyed: Option<extern "C" fn(device: &LibinputDevice)>
}
