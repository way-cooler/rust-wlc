//! Contains struct and enum declarations for
//! structs defined by wlc.

use std::fmt;

/// Log level to pass into wlc logging
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogType {
    /// Info log type
    Info,
    /// Warn log type
    Warn,
    /// Error log type
    Error,
    /// Wayland logs
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

bitflags! {
    /// Flags describing wayland events
    #[repr(C)]
    pub flags EventBit: u32 {
        /// Event can be read
        const EVENT_READABLE = 1,
        /// Event can be written
        const EVENT_WRITEABLE = 2,
        /// Event is hung up (?)
        const EVENT_HANGUP = 4,
        /// Event is in error
        const EVENT_ERROR = 8
    }
}

bitflags! {
    /// How window is being viewed
    #[repr(C)]
    pub flags ViewState: u32 {
        /// Window maximized
        const VIEW_MAXIMIZED = 1,
        /// Window fullscreen
        const VIEW_FULLSCREEN = 2,
        /// Window resizing
        const VIEW_RESIZING = 4,
        /// Window moving
        const VIEW_MOVING = 8,
        /// Window activated
        const VIEW_ACTIVATED = 16
    }
}

bitflags! {
    /// Viewtype - like x11 flags
    #[repr(C)]
    pub flags ViewType: u32 {
        /// Override redirect (X11)
        const VIEW_BIT_OVERRIDE_REDIRECT = 1,
        /// Tooltips (X11)
        const VIEW_BIT_UNMANAGED = 2,
        /// Splash Screens (X11)
        const VIEW_BIT_SPLASH = 4,
        /// Modal Windows (X11)
        const VIEW_BIT_MODAL = 8,
        /// xdg-shell, wl-shell popups
        const VIEW_BIT_POPUP = 16
    }
}

bitflags! {
    /// Which edge is being used to resize a window.
    #[repr(C)]
    pub flags ResizeEdge: u32 {
        /// No edge
        const EDGE_NONE = 0,
        /// Top edge
        const RESIZE_TOP = 1,
        /// Bottom edge
        const RESIZE_BOTTOM = 2,
        /// Left edge
        const RESIZE_LEFT = 4,
        /// Top left edge
        const RESIZE_TOPLEFT = 5,
        /// Bottom left edge
        const RESIZE_BOTTOMLEFT = 6,
        /// Right edge
        const RESIZE_RIGHT = 8,
        /// Top right edge
        const RESIZE_TOPRIGHT = 9,
        /// Bottom right edge
        const RESIZE_BOTTOMRIGHT = 10
    }
}

bitflags! {
    /// Represents which keyboard meta keys are being pressed.
    #[repr(C)]
    pub flags KeyMod: u32 {
        /// No modifiers
        const MOD_NONE = 0,
        /// Shift
        const MOD_SHIFT = 1,
        /// Caps lock
        const MOD_CAPS = 2,
        /// Control
        const MOD_CTRL = 4,
        /// Alt
        const MOD_ALT = 8,
        /// Mod2
        const MOD_MOD2 = 16,
        /// Mod3
        const MOD_MOD3 = 32,
        /// Mod4/logo
        const MOD_MOD4 = 64,
        /// 5Mod5Me
        const MOD_MOD5 = 128
    }
}

bitflags! {
    /// "LEDs" or active key-locks.
    /// i.e. caps lock, scroll lock
    #[repr(C)]
    pub flags KeyboardLed: u32 {
        /// Num lock is pressed
        const NUM_LOCK = 1,
        /// Caps lock is pressed
        const CAPS_LOCK = 2,
        /// Original typo of SCROLL_LOCK
        ///
        /// # Deprecated
        /// Please use SCROLL_LOCK instead.
        const SCROL_LLOCK = 4,
        /// Scroll lock key is being pressed.
        const SCROLL_LOCK = 4
    }
}

/// Represents a key state in key events
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyState {
    /// Key is being pressed
    Released = 0,
    /// Key is being released
    Pressed = 1
}

/// Represents a button state in button events
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonState {
    /// Button is being pressed
    Released = 0,
    /// Button is being released
    Pressed = 1
}

/// Which axis of the scroll wheel is being used
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrollAxis {
    /// No axes
    None = 0,
    /// Vertical scroll
    Vertical = 1,
    /// Horizontal scroll
    Horizontal = 2,
    /// Both scrolls
    Both = 3
}

/// Touch type in touch interface handler
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TouchType {
    /// Touch down
    Down,
    /// Touch up
    Up,
    /// Touch motion
    Motion,
    /// Touch frame
    Frame,
    /// Touch cancelled
    Cancel
}

/// State of keyoard modifiers.
/// i.e. control key, caps lock on
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardModifiers {
    /// Which "lock" keys are being pressed
    pub leds: KeyboardLed,
    /// Which control/meta keys are being pressed
    pub mods: KeyMod
}

/// Represents the location of a view.
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point {
    /// x coordinate
    pub x: i32,
    /// y coordinate
    pub y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "({}, {})", self.x, self.y)
    }
}

/// Represents the height and width of a view.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Size {
    /// Width
    pub w: u32,
    /// Height
    pub h: u32
}

impl fmt::Display for Size {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{} x {}", self.w, self.h)
    }
}

/// Represents the location and size of a view
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Geometry {
    /// The location of the object
    pub origin: Point,
    /// The size of the object
    pub size: Size
}

impl fmt::Display for Geometry {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "[{} at {}]", self.size, self.origin)
    }
}

/// Not currently supporting libinput
#[repr(C)]
pub struct LibinputDevice;
