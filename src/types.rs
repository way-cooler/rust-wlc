//! Contains struct and enum declarations for
//! structs defined by the wlc protocl.

use std::fmt;

// Types

/// Log level to pass into wlc logging
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogType {
    Info,
    Warn,
    Error,
    Wayland
}

/// Type of backend that a window is being composited in
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
bitflags! {
    flags EventBit: u32 {
        /// Event can be read
        const Readable = 1,
        /// Event can be written
        const Writeable = 2,
        /// Event is hung up (?)
        const Hangup = 4,
        /// Event is in error
        const Error = 8
    }
}

/// How and window is being viewed
#[repr(C)]
bitflags! {
    flags ViewState: u32 {
        /// Window maximized
        const Maximized = 1,
        /// Window fullscreen
        const Fullscreen = 2,
        /// Window resizing
        const Resizing = 4,
        /// Window moving
        const Moving = 8,
        /// Window activated
        const Activated = 16
    }
}

/// Viewtype - like x11 flags
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
//#[repr(C)]
bitflags! {
    flags ResizeEdge: u32 {
        const TOP = 1,
        const BOTTOM = 2,
        const LEFT = 4,

        const TOPLEFT = TOP.bits & LEFT.bits,
        const BOTTOMLEFT = BOTTOM.bits & LEFT.bits,
        const RIGHT = 8,
        const TOPRIGHT = RIGHT.bits & TOP.bits,
        const BOTTOMRIGHT = RIGHT.bits & BOTTOM.bits
    }
}

/// Represents which keyboard meta keys are being pressed.
//#[repr(C)]
bitflags! {
    flags KeyMod: u32 {
        const SHIFT = 1,
        const CAPS = 2,
        const CTRL = 4,
        const ALT= 8,
        const MOD2 = 16,
        const MOD3 = 32,
        /// Mod4?
        const MOD4 = 64,
        const MOD5 = 128
    }
}

/// "LEDs" or active key-locks.
/// i.e. caps lock, scroll lock
//#[repr(C)]
//#[derive(Debug, Clone, PartialEq, Eq)]
bitflags! {
    flags KeyboardLed: u32 {
        const NUM_LOCK = 1,
        const CAPS_LOCK = 2,
        const SCROL_LLOCK = 4
    }
}

/// Represents a key state in key events
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyState {
    Released = 0,
    Pressed = 1
}

/// Represents a button state in button events
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Released = 0,
    Pressed = 1
}

/// Which axis of the scroll wheel is being used
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrollAxis {
    None = 0,
    Vertical = 1,
    Horizontal = 2
}

/// Touch type in touch interface handler
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardModifiers {
    pub leds: KeyboardLed,
    pub mods: KeyMod
}

/// Standard x, y i32 point
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "({}, {})", self.x, self.y)
    }
}

/// Represents the height and width of a program
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Size {
    pub w: u32,
    pub h: u32
}

/// Represents the location and size of a program
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Geometry {
    pub origin: Point,
    pub size: Size
}

/// Not currently supporting libinput
#[repr(C)]
pub struct LibinputDevice;

/// Represents a wayland display.
enum WLDisplay { }

/// Represents a wayland resource.
/// This object can be rendered in pre and post render hooks.
enum WLResource { }
