//! Contains methods for interacting with the pointer
//! and keyboard of wlc.

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(deprecated)]

pub mod pointer {
//! Methods for interacting with the mouse
    use super::super::types::{Point};

    /// Gets the current position of the mouse.
    pub fn get_position() -> Point {
        let point = Point { x: 0, y: 0 };
        return point;
    }

    /// Sets the current mouse position. Required on mouse move callback.
    pub fn set_position(point: Point) {
    }
}

pub mod keyboard {
//! Methods for interacting with the keyboard
    use super::super::types::{KeyboardModifiers};
    use super::super::xkb::Keysym;

    /// Get currently held keys.
    /// # Panics
    /// All the time, this function hasn't been implemented yet
    pub fn get_current_keys<'a>() -> &'a[u32] {
        unimplemented!();
    }

    /// Gets a keysym given a key and modifiers.
    pub fn get_keysym_for_key(key: u32, modifiers: KeyboardModifiers) -> Keysym {
        unimplemented!()
    }

    /// Gets a UTF32 value for a given key and modifiers.
    pub fn get_utf32_for_key(key: u32, modifiers: KeyboardModifiers) -> u32 {
        unimplemented!()
    }
}
