//! Some libxkbcommon bindings.
//!
//! We do not wrap the full funcionality of xkb, as wlc handles
//! most of the setup.

// Keysym utils functions

/// An xkb keycode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keycode(u32);

/// An xkb keysym.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keysym(u32);

enum KeyboardFlags {
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
