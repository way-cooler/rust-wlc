//! Dummy code for wayland functions
use wayland_sys::server::{wl_display};

use libc::uintptr_t;

use types::{Size, Geometry};


/// ## Requires `wlc-wayland` feature
///
/// A wlc resource for Wayland interop
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct WlcResource {
    handle: uintptr_t,
    size: Size,
    subsurface_geometry: Geometry,
    subsurfaces: Vec<WlcResource>
}

/// Get the wayland display for the current session.
///
/// Always panics.
pub fn get_display() -> *mut wl_display {
    unimplemented!()
}


impl WlcResource {
    /// # Requires `wlc-wayland` feature
    ///
    /// Gets the size of this surface
    pub fn get_surface_size(self) -> Size {
        self.size
    }

    /// Gets the inner uintptr_t value that resource uses.
    pub fn get_raw(self) -> uintptr_t {
        self.handle
    }

    /// ## Requires `wlc-wayland` feature
    ///
    /// Gets a list of subsurfaces from the given view
    pub fn get_subsurfaces(self) -> Vec<WlcResource> {
        self.subsurfaces
    }

    /// # Requires `wlc-wayland` feature
    ///
    /// Gets the subsurface geometry of this WlcResource
    pub fn get_subsurface_geometry(self) -> Geometry {
        self.subsurface_geometry
    }
}
