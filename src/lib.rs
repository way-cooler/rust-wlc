//#![feature(libc)]
// This code will be used
#![allow(dead_code)]
extern crate libc;

// Types

/// Log level to pass into wlc logging
enum LogType {
    Info,
    Warn,
    Error,
    Wayland
}

/// Type of backend that a window is being composited in
enum BackendType {
    /// Backend type is unknown
    None,
    /// Standard wayland client
    DRM,
    /// wayland-x11 client
    X11
}

/// Bitflags describing wayland events
enum EventBit {
    Readable = 1,
    Writeable = 2,
    Hangup = 4,
    Error = 8
}

/// How and window is being viewed
enum ViewState {
    Maximized = 1,
    Fullscreen = 2,
    Resizing = 4,
    Moving = 8,
    Activated = 16
}

/// Viewtype - like x11 flags
enum ViewType {
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
enum ResizeEdge {
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
enum KeyModifier {
    None = 0,
    Shift = 1,
    Caps = 2,
    Ctrl = 4,
    Alt = 8,
    Mod2 = 16,
    Mod3 = 32,
    Logo = 64,
    Mod5 = 128
}

/// "LEDs" or active key-locks.
/// i.e. caps lock, scroll lock
enum KeyboardLed {
    None = 0,
    NumLock = 1,
    CapsLock = 2,
    ScrollLock = 4
}

/// Represents a key state in key events
enum KeyState {
    Released = 0,
    Pressed = 1
}

/// Represents a button state in button events
enum ButtonState {
    Released = 0,
    Pressed = 1
}

/// Which axis of the scroll wheel is being used
enum ScrollAxis {
    None = 0,
    Vertical = 1,
    Horizontal = 2
}

/// Touch type in touch interface handler
enum TouchType {
    Down,
    Up,
    Motion,
    Frame,
    Cancel
}

/// State of keyoard modifiers.
/// i.e. control key, caps lock on
#[repr(C)]
struct KeyboardModifiers {
    leds: KeyboardLed,
    mods: KeyModifier
}

/// Standard x, y i32 point
#[repr(C)]
struct Point {
    x: i32,
    y: i32
}

/// Represents the height and width of a program
#[repr(C)]
struct WLCSize {
    w: i32,
    h: i32
}

/// Represents the location and size of a program
#[repr(C)]
struct Geometry {
    size: WLCSize,
    origin: Point
}

/// Function signature of some standard Wwlc callbacks
type InterfaceHandler = fn(WLCHandle) -> ();

/// Many of the wlc commands take a wlc_handle as their input for
/// manipulating clients in the compositor.
/// This library has turned it into an object which has instance
/// methods to obtain this data.
type WLCHandle = libc::uintptr_t;

/// Represents the wlc interface.
/// wlc initialization involves registering a series of callbacks to the library
/// using this interface struct.
#[repr(C)]
struct WlcInterface {
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
    created: fn(handle: WLCHandle) -> bool,
    destroyed: InterfaceHandler,
    focus: fn(handle: WLCHandle, focused: bool) -> (),
    resolution: fn(handle: WLCHandle, old_size: WLCSize, new_size: WLCSize) -> (),
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
    created: fn(handle: WLCHandle) -> bool,
    destroyed: InterfaceHandler,
    focus: fn(handle: WLCHandle, focused: bool) -> (),
    move_to_output: fn(current: WLCHandle, WLCHandle, WLCHandle) -> (),
    request: RequestInterface,
}

/// Represents window rendering callbacks
#[repr(C)]
struct RequestInterface {
    geometry: fn(handle: WLCHandle, geometry: Geometry) -> (),
    state: fn(current: WLCHandle, state: ViewState, handled: bool) -> (),
    move_: fn(handle: WLCHandle, destination: Point) -> (),
    resize: fn(handle: WLCHandle, edge: ResizeEdge, location: Point) -> (),
    render: RenderInterface,
}

/// Represents keyboard press callbacks
#[repr(C)]
struct KeyboardInterface {
    // WARNING TODO key and time might need to be switched in keyboard example
    key: fn(handle: WLCHandle, key: u32, mods: KeyboardModifiers, time: u32, state: KeyState) -> bool,
}

/// Represents mouse input callbacks
#[repr(C)]
struct PointerInterface {
    button: fn(hande: WLCHandle, button: u32, mods: KeyboardModifiers, time: u32, state: ButtonState, point: Point) -> bool,
    scroll: fn(handle: WLCHandle, button: u32, mods: KeyboardModifiers, axis: ScrollAxis, heights: [u64; 2]) -> bool,
    // dist?
    motion: fn(heights: WLCHandle, dist: u32, point: Point),
}

/// Represents touchscreen callbacks
#[repr(C)]
struct TouchInterface {
    touch: fn(WLCHandle, u32, KeyboardModifiers, TouchType, i32, Point) -> bool,
}

/// Represents a callback for initializing a callback
#[repr(C)]
struct CompositorInterface {
    ready: fn() -> ()
}

/// Represents callbacks for window creation and destruction
#[repr(C)]
struct InputInterface {
    created: fn(device: LibinputDevice) -> bool,
    destroyed: fn(device: LibinputDevice) -> ()
}

/// Not currently supporting libinput
//#[repr(C)]
enum LibinputDevice {}

// External WLC functions
extern "C" {

    //fn wlc_log(dpg: LogType, fmt: str, args:);

    /// Intitializes wlc with a callback struct
    /// and c-specified program arguments.
    fn wlc_init(interface: &WlcInterface, argc: i32, argv: *mut *mut char) -> bool;

    /// Starts 
    fn wlc_run();

    fn wlc_get_background_type() -> BackendType;

    fn wlc_terminate();


}
