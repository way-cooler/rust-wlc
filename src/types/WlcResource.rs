#![feature(libc)]
extern crate libc;
use std::ffi;

/// Represents a wlc resource, which represents a wayland surface.
/// This object can be queried for its size wayland surface properties
/// and rendered in pre and post render hooks.
///
/// # Usage
///
/// This type is not functional. It will be added as the feature list expands.
pub type WlcResource = libc::uintptr_t;

extern "C" {
    
}
