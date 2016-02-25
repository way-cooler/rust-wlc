//! Some libxkbcommon bindings.
//!
//! We do not wrap the full funcionality of xkb, as wlc handles
//! most of the setup.

// Keysym utils functions

// An xkb keycode.
// Keycodes are handled by wlc
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Keycode(u32);

/// An xkb keysym.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keysym(u32);

/// Represents flags used for `Keysym::from_name`
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameFlags {
    None = 0,
    CaseInsensitive = 1
}

#[link(name = "libxkbcommon")]
extern "C" {
    fn xkb_keysym_get_name(keysym: u32, buffer: *mut char, size: libc::size_t) -> i32;

    fn xkb_keysym_from_name(name: *const char, flags: KeyboardFlags) -> u32;

    fn xkb_keysym_to_utf8(keysym: u32, buffer: *mut char, size: libc::size_t) -> i32;

    fn xkb_keysym_to_utf32(keysym: u32) -> u32;
}

impl Keysym {
    fn from_name(name: String, flags: KeyboardFlags) -> Keysym {
        
    }

    fn get_name() -> String {
        // create buffer
        // Call get_name with buffer
        // Convert buffer to String
    }

    fn to_utf8() -> i32 {
        // create buffer
        // call to_utf8 with buffer
        // Convert buffer to String
    }

    fn to_utf32() -> u32 {
        
    }
}
