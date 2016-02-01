//! Contains methods for interacting with the pointer
//! and keyboard of wlc. It's beyond our scope to wrap
//! the xkb enums so we may split that into its own
//! project.

use super::types::{KeyModifier, Point};

#[link(name = "wlc")]
extern "C" {
    // Keyboard functions
    fn wlc_keyboard_get_keysym_for_key(key: u32, modifiers: &KeyModifier) -> u32;

    fn wlc_keyboard_get_utf32_for_key(key: u32, modifiers: &KeyModifier) -> u32;

    // Pointer functions
    fn wlc_pointer_get_position(out_position: *mut Point);

    fn wlc_pointer_set_position(position: &Point);
}

pub mod pointer {
    use super::super::types::{Point};

    /// Gets the current position of the mouse.
    pub fn get_position() -> Point {
        unsafe {
            let mut point = Point { x: 0, y: 0 };
            super::wlc_pointer_get_position(&mut point);
            return point;
        }
    }

    // Sets the current mouse position. Required on mouse move callback.
    pub fn set_position(point: &Point) {
        unsafe { super::wlc_pointer_set_position(point); }
    }
}

pub mod keyboard {
    use super::super::types::{KeyModifier};

    pub fn get_keysym_for_key(key: u32, modifiers: &KeyModifier) -> u32 {
        unsafe { super::wlc_keyboard_get_keysym_for_key(key, modifiers) }
    }

    pub fn get_utf32_for_key(key: u32, modifiers: &KeyModifier) -> u32 {
        unsafe { super::wlc_keyboard_get_utf32_for_key(key, modifiers) }
    }
}
