//! Contains methods for interacting with the pointer
//! and keyboard of wlc.

use super::types::{KeyMod, Point};
use libc::size_t;

#[link(name = "wlc")]
extern "C" {
    fn wlc_keyboard_get_current_keys(out_memb: *const size_t) -> *const u32;

    fn wlc_keyboard_get_keysym_for_key(key: u32, modifiers: &KeyMod) -> u32;

    fn wlc_keyboard_get_utf32_for_key(key: u32, modifiers: &KeyMod) -> u32;

    // Pointer functions
    fn wlc_pointer_get_position(out_position: *mut Point);

    fn wlc_pointer_set_position(position: &Point);
}

pub mod pointer {
//! Methods for interacting with the mouse
    use super::super::types::{Point};

    /// Gets the current position of the mouse.
    pub fn get_position() -> Point {
        unsafe {
            let mut point = Point { x: 0, y: 0 };
            super::wlc_pointer_get_position(&mut point);
            return point;
        }
    }

    /// Sets the current mouse position. Required on mouse move callback.
    pub fn set_position(point: &Point) {
        unsafe { super::wlc_pointer_set_position(point); }
    }
}

pub mod keyboard {
//! Methods for interacting with the keyboard
    use super::super::types::{KeyMod};
    use super::super::xkb::Keysym;
    use libc::size_t;
    use std::slice;

    /// Get currently held keys.
    pub fn get_current_keys<'a>() -> &'a[u32] {
        let mut out_memb: size_t = 0;
        unsafe {
            let keys = super::wlc_keyboard_get_current_keys(&mut out_memb);
            return slice::from_raw_parts(keys, out_memb as usize);
        }
    }

    /// Gets a keysym given a key and modifiers.
    pub fn get_keysym_for_key(key: u32, modifiers: &KeyMod) -> Keysym {
        unsafe { Keysym::from(super::wlc_keyboard_get_keysym_for_key(key, modifiers)) }
    }

    /// Gets a UTF32 value for a given key and modifiers.
    pub fn get_utf32_for_key(key: u32, modifiers: &KeyMod) -> u32 {
        unsafe { super::wlc_keyboard_get_utf32_for_key(key, modifiers) }
    }
}
