//! types/interface
//! Contains all structs used for initializing wlc.
//! You will only need this module for invoking
//! `rustwlc::init`.

#![feature(libc)]
extern crate libc;

use std::ffi;
use std::ffi::{CString};

use super::*;
use super::super::handle::WlcHandle;

/// Function signature of some standard Wwlc callbacks
pub type InterfaceHandler = Option<extern "C" fn(WlcHandle) -> ()>;

/// Represents the wlc callback interface.
/// wlc initialization involves registering
/// a series of callbacks to the library
/// using this interface struct.
#[repr(C)]
#[no_mangle]
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
    pub created: Option<extern "C" fn(handle: WlcHandle) -> bool>,
    pub destroyed: InterfaceHandler,
    pub focus: Option<extern "C" fn(handle: WlcHandle, focused: bool) -> ()>,
    pub resolution: Option<extern "C" fn(handle: WlcHandle, old_size: Size, new_size: Size) -> ()>,
    pub render: RenderInterface,
}

/// Represents global rendering callbacks
#[repr(C)]
pub struct RenderInterface {
    pub pre: InterfaceHandler,
    pub post: InterfaceHandler,
}

/// Represents window viewing callbacks
#[repr(C)]
pub struct ViewInterface {
    pub created: Option<extern "C" fn(handle: WlcHandle) -> bool>,
    pub destroyed: InterfaceHandler,
    pub focus: Option<extern "C" fn(handle: WlcHandle, focused: bool) -> ()>,
    pub move_to_output: Option<extern "C" fn(current: WlcHandle, WlcHandle, WlcHandle) -> ()>,
    pub request: RequestInterface,
}

/// Represents window rendering callbacks
#[repr(C)]
pub struct RequestInterface {
    pub geometry: Option<extern "C" fn(handle: WlcHandle, geometry: Geometry) -> ()>,
    pub state: Option<extern "C" fn(current: WlcHandle, state: ViewState, handled: bool) -> ()>,
    pub move_: Option<extern "C" fn(handle: WlcHandle, destination: Point) -> ()>,
    pub resize: Option<extern "C" fn(handle: WlcHandle, edge: ResizeEdge, location: Point) -> ()>,
    pub render: RenderInterface,
}

/// Represents keyboard press callbacks
#[repr(C)]
pub struct KeyboardInterface {
    // WARNING TODO key and time might need to be switched in keyboard example
    pub key: Option<extern "C" fn(handle: WlcHandle, key: u32, mods: KeyboardModifiers, time: u32, state: KeyState) -> bool>,
}

/// Represents mouse input callbacks
#[repr(C)]
pub struct PointerInterface {
    pub button: Option<extern "C" fn(hande: WlcHandle, button: libc::c_uint, mods: KeyboardModifiers, time: u32, state: ButtonState, point: Point) -> bool>,
    pub scroll: Option<extern "C" fn(handle: WlcHandle, button: u32, mods: KeyboardModifiers, axis: ScrollAxis, heights: [u64; 2]) -> bool>,
    // dist?
    pub motion: Option<extern "C" fn(heights: WlcHandle, dist: u32, point: Point)>,
}

/// Represents touchscreen callbacks
#[repr(C)]
pub struct TouchInterface {
    /// NOTE WARNING TODO Not sure if key and touch need to be switched
    pub touch: Option<extern "C" fn(handle: WlcHandle, time: libc::c_uint, mods: KeyboardModifiers, touch: TouchType, key: libc::c_int, point: Point) -> bool>,
}

/// Represents a callback for initializing the callback
#[repr(C)]
pub struct CompositorInterface {
    pub ready: Option<extern "C" fn() -> ()>
}

/// Represents callbacks for window creation and destruction
#[repr(C)]
pub struct InputInterface {
    pub created: Option<extern "C" fn(device: LibinputDevice) -> bool>,
    pub destroyed: Option<extern "C" fn(device: LibinputDevice) -> ()>
}
