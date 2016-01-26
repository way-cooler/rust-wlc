//! Contains struct and enum declarations for
//! structs defined by the wlc protocl.

pub mod interface;

use std::option::Option;
use std::env;
use std::ffi;
use std::os::unix::prelude;

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
    pub leds: KeyboardLed,
    pub mods: KeyModifier
}

/// Standard x, y i32 point
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

/// Represents the height and width of a program
#[repr(C)]
pub struct Size {
    pub w: i32,
    pub h: i32
}

/// Represents the location and size of a program
#[repr(C)]
pub struct Geometry {
    pub size: WLCSize,
    pub origin: Point
}

/// Function signature of some standard Wwlc callbacks
pub type InterfaceHandler = Option<extern "C" fn(WLCHandle) -> ()>;

/// Many of the wlc commands take a wlc_handle as their input for
/// manipulating clients in the compositor.
/// This library has turned it into an object which has instance
/// methods to obtain this data.
pub type WLCHandle = libc::uintptr_t;

/// Not currently supporting libinput
//#[repr(C)]
pub enum LibinputDevice {}

/// Represents a wayland display.
enum WLDisplay { }

/// Represents a wayland resource.
/// This object can be rendered in pre and post render hooks.
enum WLResource { }
