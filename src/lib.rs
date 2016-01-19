// This code will be used later
#![allow(dead_code)]
extern crate libc;

use std::option::Option;

// Types

/// Log level to pass into wlc logging
#[repr(C)]
pub enum LogType {
    Info,
    Warn,
    Error,
    Wayland
}

/// Type of backend that a window is being composited in
#[repr(C)]
pub enum BackendType {
    /// Backend type is unknown
    None,
    /// Standard wayland client
    DRM,
    /// wayland-x11 client
    X11
}

/// Bitflags describing wayland events
#[repr(C)]
pub enum EventBit {
    /// Event can be read
    Readable = 1,
    /// Event can be written
    Writeable = 2,
    /// Event is hung up (?)
    Hangup = 4,
    /// Event is in error
    Error = 8
}

/// How and window is being viewed
#[repr(C)]
pub enum ViewState {
    /// Window maximized
    Maximized = 1,
    /// Window fullscreen
    Fullscreen = 2,
    /// Window resizing
    Resizing = 4,
    /// Window moving
    Moving = 8,
    /// Window activated
    Activated = 16
}

/// Viewtype - like x11 flags
pub enum ViewType {
    /// Override redirect (X11)
    OverrideRedirect = 1,
    /// Tooltips (X11)
    Unmanaged = 2,
    /// Splash Screens (X11)
    Splash = 4,
    /// Modal Windows (X11)
    Modal = 8,
    /// xdg-shell, wl-shell popups
    Popup = 16
}

// Which edge is being used to resize a window.
// Works like bitflags but also has all the options in the enum
#[repr(C)]
pub enum ResizeEdge {
    None = 0,
    Top = 1,
    Bottom = 2,
    Left = 4,
    TopLeft = 5,
    BottomLeft = 6,
    Right = 8,
    TopRight = 9,
    BottomRight = 10
}

/// Represents which keyboard meta keys are being pressed.
#[repr(C)]
pub enum KeyModifier {
    /// (assumed)
    None = 0,
    Shift = 1,
    Caps = 2,
    Ctrl = 4,
    Alt = 8,
    Mod2 = 16,
    Mod3 = 32,
    /// Mod4?
    Logo = 64,
    Mod5 = 128
}

/// "LEDs" or active key-locks.
/// i.e. caps lock, scroll lock
#[repr(C)]
pub enum KeyboardLed {
    None = 0,
    NumLock = 1,
    CapsLock = 2,
    ScrollLock = 4
}

/// Represents a key state in key events
#[repr(C)]
pub enum KeyState {
    Released = 0,
    Pressed = 1
}

/// Represents a button state in button events
#[repr(C)]
pub enum ButtonState {
    Released = 0,
    Pressed = 1
}

/// Which axis of the scroll wheel is being used
#[repr(C)]
pub enum ScrollAxis {
    None = 0,
    Vertical = 1,
    Horizontal = 2
}

/// Touch type in touch interface handler
#[repr(C)]
pub enum TouchType {
    Down,
    Up,
    Motion,
    Frame,
    Cancel
}

/// State of keyoard modifiers.
/// i.e. control key, caps lock on
#[repr(C)]
pub struct KeyboardModifiers {
    leds: KeyboardLed,
    mods: KeyModifier
}

/// Standard x, y i32 point
#[repr(C)]
pub struct Point {
    x: i32,
    y: i32
}

/// Represents the height and width of a program
#[repr(C)]
pub struct WLCSize {
    w: i32,
    h: i32
}

/// Represents the location and size of a program
#[repr(C)]
pub struct Geometry {
    size: WLCSize,
    origin: Point
}

/// Function signature of some standard Wwlc callbacks
type InterfaceHandler = Option<extern "C" fn(WLCHandle) -> ()>;

/// Many of the wlc commands take a wlc_handle as their input for
/// manipulating clients in the compositor.
/// This library has turned it into an object which has instance
/// methods to obtain this data.
pub type WLCHandle = libc::uintptr_t;

/// Represents the wlc callback interface.
/// wlc initialization involves registering a series of callbacks to the library
/// using this interface struct.
#[repr(C)]
#[no_mangle]
pub struct WlcInterface {
    output: OutputInterface,
    view: ViewInterface,
    keyboard: KeyboardInterface,
    pointer: PointerInterface,
    touch: TouchInterface,
    compositor: CompositorInterface,
    input: InputInterface
}

/// Represents window callbacks
#[repr(C)]
struct OutputInterface {
    created: Option<extern "C" fn(handle: WLCHandle) -> bool>,
    destroyed: InterfaceHandler,
    focus: Option<extern "C" fn(handle: WLCHandle, focused: bool) -> ()>,
    resolution: Option<extern "C" fn(handle: WLCHandle, old_size: WLCSize, new_size: WLCSize) -> ()>,
    render: RenderInterface,
}

/// Represents global rendering callbacks
#[repr(C)]
struct RenderInterface {
    pre: InterfaceHandler,
    post: InterfaceHandler,
}

/// Represents window viewing callbacks
#[repr(C)]
struct ViewInterface {
    created: Option<extern "C" fn(handle: WLCHandle) -> bool>,
    destroyed: InterfaceHandler,
    focus: Option<extern "C" fn(handle: WLCHandle, focused: bool) -> ()>,
    move_to_output: Option<extern "C" fn(current: WLCHandle, WLCHandle, WLCHandle) -> ()>,
    request: RequestInterface,
}

/// Represents window rendering callbacks
#[repr(C)]
struct RequestInterface {
    geometry: Option<extern "C" fn(handle: WLCHandle, geometry: Geometry) -> ()>,
    state: Option<extern "C" fn(current: WLCHandle, state: ViewState, handled: bool) -> ()>,
    move_: Option<extern "C" fn(handle: WLCHandle, destination: Point) -> ()>,
    resize: Option<extern "C" fn(handle: WLCHandle, edge: ResizeEdge, location: Point) -> ()>,
    render: RenderInterface,
}

/// Represents keyboard press callbacks
#[repr(C)]
struct KeyboardInterface {
    // WARNING TODO key and time might need to be switched in keyboard example
    key: Option<extern "C" fn(handle: WLCHandle, key: u32, mods: KeyboardModifiers, time: u32, state: KeyState) -> bool>,
}

/// Represents mouse input callbacks
#[repr(C)]
struct PointerInterface {
    button: Option<extern "C" fn(hande: WLCHandle, button: libc::c_uint, mods: KeyboardModifiers, time: u32, state: ButtonState, point: Point) -> bool>,
    scroll: Option<extern "C" fn(handle: WLCHandle, button: u32, mods: KeyboardModifiers, axis: ScrollAxis, heights: [u64; 2]) -> bool>,
    // dist?
    motion: Option<extern "C" fn(heights: WLCHandle, dist: u32, point: Point)>,
}

/// Represents touchscreen callbacks
#[repr(C)]
struct TouchInterface {
    /// NOTE WARNING TODO Not sure if key and touch need to be switched
    touch: Option<extern "C" fn(handle: WLCHandle, time: libc::c_uint, mods: KeyboardModifiers, touch: TouchType, key: libc::c_int, point: Point) -> bool>,
}

/// Represents a callback for initializing the callback
#[repr(C)]
struct CompositorInterface {
    ready: Option<extern "C" fn() -> ()>
}

/// Represents callbacks for window creation and destruction
#[repr(C)]
struct InputInterface {
    created: Option<extern "C" fn(device: LibinputDevice) -> bool>,
    destroyed: Option<extern "C" fn(device: LibinputDevice) -> ()>
}

/// Not currently supporting libinput
//#[repr(C)]
enum LibinputDevice {}

// External WLC functions
extern "C" {

    //fn wlc_log(dpg: LogType, fmt: str, args:);

    /// Intitializes wlc with a callback struct
    /// and c-specified program arguments.
    pub fn wlc_init(interface: *const WlcInterface, argc: libc::c_int, argv: *mut *mut libc::c_char) -> bool;

    /// Starts wlc compositor
    fn wlc_run();

    fn wlc_get_background_type() -> BackendType;

    fn wlc_terminate();


}

// From wlc_wayland.h

/// Represents a wlc resource, which represents a wayland surface.
/// This object can be queried for its size wayland surface properties
/// and rendered in pre and post render hooks.
type WLCResource = libc::uintptr_t;

/// Represents a wayland display.
enum WLDisplay { }

/// Represents a wayland resource.
/// This object can be rendered in pre and post render hooks.
enum WLResource { }

extern "C" {
    /// Returns Wayland display
    fn wlc_get_wl_display() -> WLDisplay;

    /// Returns view handle from WLSurface resource
    fn wlc_handle_from_wl_surface_resource(resource: WLResource) -> WLCHandle;

    /// Returns output handle from WLOutput resource
    fn wlc_handle_from_wl_output_resource(resource: WLResource) -> WLCHandle;

    /// Returns internal WLC surface from WLSurface resource
    fn wlc_resource_from_wl_surface_resource(resource: WLResource) -> WLCResource;

    /// Returns internal WLC surface from view handle
    fn wlc_view_get_surface(handle: WLCHandle) -> WLCResource;

    /// Gets the size of a surface
    fn wlc_surface_get_size(resource: WLCResource) -> WLCSize;

    /// Renders surfaces inside pre and post render hooks
    fn wlc_surface_render(surface: WLCResource, geometry: &Geometry) -> ();
}
