//! Some libxkbcommon bindings.
//!
//! We do not wrap the full funcionality of xkb, as wlc handles
//! most of the setup.
use std::ffi::{CStr};
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

    fn xkb_keysym_to_utf8(keysym: u32, buffer: &mut char, size: libc::size_t) -> i32;

    fn xkb_keysym_to_utf32(keysym: u32) -> u32;
}

impl Keysym {

    /// Whether this keysym is valid or is `XKB_KEY_NoSymbol`
    fn is_valid(&self) {
        return self.0 != 0;
    }

    /// Gets the Keysym for the given name.
    fn from_name(name: &str, flags: KeyboardFlags) -> Option<Keysym> {
        unsafe {
            let c_name = Cstr::new(name).unwrap() as *const char;
            let sym_val: u32 = xkb_keysym_from_name(c_name, flags);
            match sym_val {
                0 => None,
                _ => Some(Keysym(sym_val))
            }
        }
    }

    /// Gets 
    fn get_name(&self) -> Option<String> {
        // create buffer
        // Call get_name with buffer
        // if get_name == -1 None
        // Convert buffer to String
        // The xkb documentation specifically recommends 7 as a buffer length
        const BUFFER_LEN: usize = 7usize;
        let buffer_vec: Vec<char> = Vec::with_capacity(BUFFER_LEN);
        unsafe {
            let mut buffer: &mut char = buffer_vec.as_mut_slice();
            let result = xkb_keysym_get_name(self.0, buffer, BUFFER_LEN);
            match result {
                -1 => None,
                _ => str::from_utf8_lossy(buffer)
        }
    }

    fn to_utf8() -> i32 {
        // create buffer
        // call to_utf8 with buffer
        // Convert buffer to String
    }

    fn to_utf32() -> u32 {
        
    }
}
