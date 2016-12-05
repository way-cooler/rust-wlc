//! Contains methods for interacting with the pointer
//! and keyboard of wlc.

use libc::{size_t, uint32_t};
use super::types::{KeyboardModifiers, Point};

#[link(name = "wlc", kind="static")]
extern "C" {
    fn wlc_keyboard_get_current_keys(out_memb: *const size_t) -> *const uint32_t;

    fn wlc_keyboard_get_keysym_for_key(key: uint32_t,
                                       modifiers: *const KeyboardModifiers) -> uint32_t;

    fn wlc_keyboard_get_utf32_for_key(key: uint32_t,
                                      modifiers: *const KeyboardModifiers) -> uint32_t;

    // Pointer functions
    fn wlc_pointer_get_position(out_position: *mut Point);

    fn wlc_pointer_set_position(position: *const Point);
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
    pub fn set_position(point: Point) {
        unsafe { super::wlc_pointer_set_position(&point); }
    }
}

pub mod keyboard {
    //! Methods for interacting with the keyboard

    use std::slice;
    use libc::size_t;
    use super::super::types::KeyboardModifiers;
    #[allow(deprecated)]
    use super::super::xkb::Keysym;

    /// Get currently held keys.
    /// # Panics
    /// All the time, this function hasn't been implemented yet
    pub fn get_current_keys<'a>() -> Option<&'a[u32]> {
        let mut size: size_t = 0;
        unsafe {
            let out_ptr = super::wlc_keyboard_get_current_keys(&mut size);
            if size == 0 || out_ptr.is_null() {
                None
            }
            else {
                Some(slice::from_raw_parts(out_ptr, size as usize))
            }
        }
    }

    /// Gets a keysym given a key and modifiers.
    ///
    /// In order to delay breaking backwards compatibility this method is _not_
    /// deprecated. Please use `Keysym::raw` on the Keysym returned from this
    /// function for now. **In version 0.6 this will return a u32**.
    #[allow(deprecated)]
    pub fn get_keysym_for_key(key: u32, modifiers: KeyboardModifiers) -> Keysym {
        unsafe {
            Keysym::from(super::wlc_keyboard_get_keysym_for_key(key, &modifiers) as u32)
        }
    }

    /// Gets a UTF32 value for a given key and modifiers.
    pub fn get_utf32_for_key(key: u32, modifiers: KeyboardModifiers) -> u32 {
        unsafe { super::wlc_keyboard_get_utf32_for_key(key, &modifiers) }
    }
}
