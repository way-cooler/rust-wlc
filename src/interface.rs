//! Contains callback-holding struct `WlcInterface` which is used
//! to initialize wlc.

extern crate libc;

use std::option::Option;

use super::types::*;
use super::handle::{WlcOutput, WlcView};

/// Represents the wlc callback interface.
/// wlc initialization involves registering
/// a series of callbacks to the library
/// using this interface struct.
///
/// See `WlcInterface::new()` for usage.
#[repr(C)]
pub struct WlcInterface {
    /// Interface for output callbacks
    pub output: OutputInterface,
    /// Interface for view callbacks
    pub view: ViewInterface,
    /// Interface for keyboard callbacks
    pub keyboard: KeyboardInterface,
    /// Interface for pointer callbacks
    pub pointer: PointerInterface,
    /// Interface for touch callbacks
    pub touch: TouchInterface,
    /// Interface for compositor callbacks
    pub compositor: CompositorInterface,
    /// Interface for touch callbacks
    pub input: InputInterface
}

/// Represents window callbacks
#[repr(C)]
pub struct OutputInterface {
    /// Output was created
    pub created: Option<extern "C" fn(output: WlcOutput) -> bool>,
    /// Output lost or destroyed
    pub destroyed: Option<extern "C" fn(handle: WlcOutput)>,
    /// Output was focused
    pub focus: Option<extern "C" fn(handle: WlcOutput, focused: bool)>,
    /// Output resolution changed
    pub resolution: Option<extern "C" fn(handle: WlcOutput, old_size: &Size, new_size: &Size)>,
    /// Interface for render callbacks
    pub render: OutputRenderInterface
}

/// Represents rendering callbacks for outputs
#[repr(C)]
pub struct OutputRenderInterface {
    /// Pre render hook
    pub pre: Option<extern "C" fn(handle: WlcOutput)>,
    /// Post render hook
    pub post: Option<extern "C" fn(handle: WlcOutput)>
}

/// Represents window viewing callbacks
#[repr(C)]
pub struct ViewInterface {
    /// View was created. Return false if you want to destroy the view
    /// (e.g. failed to allocated data related to the view)
    pub created: Option<extern "C" fn(handle: WlcView) -> bool>,
    /// View was destroyed
    pub destroyed: Option<extern "C" fn(handle: WlcView)>,
    /// View lost or got focus
    pub focus: Option<extern "C" fn(handle: WlcView, focused: bool)>,
    /// View was moved to to output
    pub move_to_output: Option<extern "C" fn(current: WlcView,
                                                 from_output: WlcOutput,
                                                 to_output: WlcOutput)
                                                >,
    /// Interface for request callbacks
    pub request: RequestInterface
}

/// Represents window rendering callbacks
#[repr(C)]
pub struct RequestInterface {
    /// Request to set given geometry to view. Apply using
    /// wlc_view_set_geometry (handle.set_geometry) to agree.
    pub geometry: Option<extern "C" fn(handle: WlcView, geometry: &Geometry)>,
    /// Request to disable or enable the given state for a view.
    /// Apply using wlc_view_set_state to agree.
    pub state: Option<extern "C" fn(current: WlcView, state: ViewState, handled: bool)>,
    /// View requests to move itself. Start an interactive move to agree.
    pub move_: Option<extern "C" fn(handle: WlcView, destination: &Point)>,
    /// Request to resize itself with the given edges.
    /// Start and interactive move to agree
    pub resize: Option<extern "C" fn(handle: WlcView, edge: ResizeEdge, location: &Point)>,

    /// View rendering hooks
    pub render: ViewRenderInterface
}

/// Represents rendering callbacks for views
#[repr(C)]
pub struct ViewRenderInterface {
    /// Pre-render
    pub pre: Option<extern "C" fn(view: WlcView)>,
    /// Post-render
    pub post: Option<extern "C" fn(view: WlcView)>
}

/// Represents keyboard press callbacks
#[repr(C)]
pub struct KeyboardInterface {
    /// Key event was triggered, handle will be None if there was no focus
    /// Return true to prevent sending the event to clients.
    pub key: Option<extern "C" fn(view: WlcView,
                                      time: u32,
                                      mods: &KeyboardModifiers,
                                      key: u32,
                                      state: KeyState)
                                      -> bool>
}

/// Represents mouse input callbacks
#[repr(C)]
pub struct PointerInterface {
    /// Button event was triggered, view will be None if there was no
    /// focus. Return true to prevent sending the event to clients.
    pub button: Option<extern "C" fn(hande: WlcView,
                                         time: u32,
                                         mods: &KeyboardModifiers,
                                         button: u32,
                                         state: ButtonState,
                                         point: &Point)
                                         -> bool>,

    /// Scroll event was triggered, view handle will be None if there was
    /// no focus. Return true to prevent sending the event to clients.
    pub scroll: Option<extern "C" fn(handle: WlcView,
                                         time: u32,
                                         mods: &KeyboardModifiers,
                                         axis: ScrollAxis,
                                         amount: [u64; 2])
                                         -> bool>,
    /// Mouse was moved, view will be none if there was no focus.
    /// Use wlc_pointer_set_position to agree. Return true to prevent
    /// sending event to clients.
    pub motion: Option<extern "C" fn(heights: WlcView, time: u32, point: &Point) -> bool>
}

/// Represents touchscreen callbacks
#[repr(C)]
pub struct TouchInterface {
    /// Screen was touched, handle will be None if there was no focus.
    /// Return true to prevent sending the event to clients.
    pub touch: Option<extern "C" fn(handle: WlcView,
                                        time: u32,
                                        mods: &KeyboardModifiers,
                                        touch: TouchType,
                                        slot: i32,
                                        point: &Point)
                                        -> bool>
}

/// Represents a callback for initializing the callback
#[repr(C)]
pub struct CompositorInterface {
    /// Compositor is ready to accept clients.
    pub ready: Option<extern "C" fn()>,
    /// The compositor is terminating.
    pub terminate: Option<extern "C" fn()>
}

/// Represents experimenal callbacks for libinput events
#[repr(C)]
pub struct InputInterface {
    /// Input created
    pub created: Option<extern "C" fn(device: &LibinputDevice) -> bool>,
    /// Input destroyed
    pub destroyed: Option<extern "C" fn(device: &LibinputDevice)>
}

// Running rustfmt on this makes it needlessly long.
#[cfg_attr(rustfmt, rustfmt_skip)]
impl WlcInterface {
    /// Creates a new WlcInterface builder that can have callbacks added.
    ///
    /// # Examples
    /// ```no_run
    /// # use rustwlc::handle::WlcOutput;
    /// # extern "C" fn output_created_callback(handle: WlcOutput) -> bool { true };
    /// use rustwlc::interface::WlcInterface;
    ///
    /// // Assuming there exists an output_created_callback function...
    /// let interface = WlcInterface::new()
    ///     .output_created(output_created_callback);
    ///     // .more_callbacks() ...
    ///
    /// if let Some(run_wlc) = rustwlc::init(interface) {
    ///     run_wlc();
    /// }
    /// else {
    ///     panic!("Unable to initialize wlc!");
    /// }
    /// ```
    pub fn new() -> WlcInterface {
        WlcInterface {
            output: OutputInterface {
                created: None, destroyed: None, focus: None, resolution: None,
                render: OutputRenderInterface { pre: None, post: None }
            },
            view: ViewInterface {
                created: None, destroyed: None, focus: None, move_to_output: None,
                request: RequestInterface {
                    geometry: None, state: None, move_: None, resize: None,
                    render: ViewRenderInterface { pre: None, post: None }
                }
            },
            keyboard: KeyboardInterface { key: None },
            pointer: PointerInterface { button: None, scroll: None, motion: None },
            touch: TouchInterface { touch: None },
            compositor: CompositorInterface { ready: None, terminate: None },
            input: InputInterface { created: None, destroyed: None }
        }
    }

    /// Callback invoked when an output is created. Return `true` to allow the output to exist.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcOutput;
    ///
    /// extern fn on_output_created(output: WlcOutput) -> bool {
    ///     println!("Output {} ({:?}) was created", output.get_name(), output);
    ///     return true;
    /// }
    /// # fn main() { }
    /// ```
    pub fn output_created(mut self,
                          func: extern "C" fn(output: WlcOutput) -> bool) -> WlcInterface {
        self.output.created = Some(func); self
    }

    /// Callback invoked when an output is destroyed.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcOutput;
    ///
    /// extern fn output_destroyed(output: WlcOutput) {
    ///     println!("Goodbye, {:?}", output);
    /// }
    /// # fn main() { }
    /// ```
    pub fn output_destroyed(mut self, func: extern "C" fn(output: WlcOutput)) -> WlcInterface {
        self.output.destroyed = Some(func); self
    }

    /// Callback invoked when an output gains focus.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcOutput;
    ///
    /// extern fn output_focus(output: WlcOutput, focused: bool) {
    ///     println!("Output {} {} focus", output.get_name(),
    ///                                    if focused { "gained" } else { "lost" });
    /// }
    /// # fn main() { }
    /// ```
    pub fn output_focus(mut self,
                        func: extern "C" fn(output: WlcOutput, focused: bool)) -> WlcInterface {
        self.output.focus = Some(func); self
    }

    /// Callback invoked when an output's resolution changes.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcOutput;
    /// use rustwlc::types::Size;
    ///
    /// extern fn output_resolution(output: WlcOutput, old_size: &Size, new_size: &Size) {
    ///     println!("Output {} went from {} to {}", output.get_name(), old_size, new_size);
    /// }
    /// # fn main() { }
    /// ```
    pub fn output_resolution(mut self, func: extern "C" fn(output: WlcOutput,
                                               old_size: &Size, new_size: &Size)) -> WlcInterface {
        self.output.resolution = Some(func); self
    }

    /// Callback invoked pre-render for an output.
    pub fn output_render_pre(mut self, func: extern "C" fn(output: WlcOutput)) -> WlcInterface {
        self.output.render.pre = Some(func); self
    }

    /// Callback invoked post-render for an output.
    pub fn output_render_post(mut self, func: extern "C" fn(output: WlcOutput)) -> WlcInterface {
        self.output.render.post = Some(func); self
    }

    /// Callback invoked when a view is created. Return `true` to allow the view to be created.
    ///
    /// When a new view is created, the following should probably be applied:
    /// * Set the view's mask to the output's mask
    /// * Focus the view
    /// * Bring the view to the front
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    ///
    /// extern fn view_created(view: WlcView) -> bool {
    ///     println!("View \"{}\" was created ({:?})", view.get_class(), view);
    ///     view.set_mask(view.get_output().get_mask());
    ///     view.bring_to_front();
    ///     view.focus();
    ///     return true;
    /// }
    /// # fn main() { }
    /// ```
    pub fn view_created(mut self, func: extern "C" fn(view: WlcView) -> bool) -> WlcInterface {
        self.view.created = Some(func); self
    }

    /// Callback invoked when a view is destroyed.
    ///
    /// When a view is destroyed, it's a good idea to shift focus to
    /// some other view, i.e. the last one used.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    ///
    /// extern fn view_destroyed(view: WlcView) {
    ///     println!("Goodbye, {:?}", view);
    /// }
    /// # fn main() { }
    /// ```
    pub fn view_destroyed(mut self, func: extern "C" fn(view: WlcView)) -> WlcInterface {
        self.view.destroyed = Some(func); self
    }

    /// Callback invoked when a view is focused.
    ///
    /// The view's `ViewState::VIEW_ACTIVATED` bit should be set to true here.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    /// use rustwlc::types::*;
    ///
    /// extern fn view_focus(view: WlcView, focused: bool) {
    ///     println!("View {:?} is {} focus, updating...",
    ///               view, if focused { "in" } else { "out of" });
    ///     view.set_state(VIEW_ACTIVATED, focused);
    /// }
    /// ```
    pub fn view_focus(mut self,
                      func: extern "C" fn(handle: WlcView, focused: bool)) -> WlcInterface {
        self.view.focus = Some(func); self
    }

    /// Callback invoked when a view switches outputs.
    ///
    /// Moving views between outputs is unsupported in wlc at the time of writing.
    /// Wayland mandates each output have its own memory buffer so it may take wlc some time before
    // this is implemented.
    pub fn view_move_to_output(mut self, func: extern "C" fn(view: WlcView,
                                   old_output: WlcOutput, new_output: WlcOutput)) -> WlcInterface {
        self.view.move_to_output = Some(func); self
    }

    /// Callback invoked when a view requests geometry.
    pub fn view_request_geometry(mut self,
                        func: extern "C" fn(handle: WlcView, geometry: &Geometry)) -> WlcInterface {
        self.view.request.geometry = Some(func); self
    }

    /// Callback invoked when a view requests a `ViewState`.
    pub fn view_request_state(mut self,
           func: extern "C" fn(current: WlcView, state: ViewState, handled: bool)) -> WlcInterface {
        self.view.request.state = Some(func); self
    }

    /// Callback invoked when a view requests a move.
    pub fn view_request_move(mut self,
                        func: extern "C" fn(handle: WlcView, destination: &Point)) -> WlcInterface {
        self.view.request.move_ = Some(func); self
    }

    /// Callback invoked when a view requests a resize.
    pub fn view_request_resize(mut self,
         func: extern "C" fn(handle: WlcView, edge: ResizeEdge, location: &Point)) -> WlcInterface {
        self.view.request.resize = Some(func); self
    }

    /// Callback invoked pre-view-render.
    pub fn view_request_render_pre(mut self, func: extern "C" fn(view: WlcView)) -> WlcInterface {
        self.view.request.render.pre = Some(func); self
    }

    /// Callback invoked post-view-render.
    pub fn view_request_render_post(mut self, func: extern "C" fn(view: WlcView)) -> WlcInterface {
        self.view.request.render.post = Some(func); self
    }

    /// Callback invoked on keypresses. Return `true` to block the press from the view.
    ///
    /// # Arguments
    /// The first `u32` is a timestamp, the second is the key code. The view may be the root window.
    /// Proper values for `key` can be found in `input.h` or a similar library/crate - see wlc
    /// documentation on the subject, it may not support your keyboard layout at the moment.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    /// use rustwlc::types::{KeyboardModifiers, KeyState};
    ///
    /// extern fn keyboard_key(view: WlcView, time: u32, mods: &KeyboardModifiers,
    ///                        key: u32, state: KeyState) -> bool {
    ///     println!("Key {} {:?} on {:?} at {} with modifiers {:?}",
    ///              key, view, state, time, mods);
    ///     return false;
    /// }
    /// # fn main() { }
    /// ```
    pub fn keyboard_key(mut self, func: extern "C" fn(view: WlcView, time: u32,
                     mods: &KeyboardModifiers, key: u32, state: KeyState) -> bool) -> WlcInterface {
        self.keyboard.key = Some(func); self
    }

    /// Callback invoked on mouse clicks. Return `true` to block the click from the view.
    ///
    /// # Arguments
    /// The first u32 is a timestamp, the second is the button code. The view may be the root
    /// window. Probper values for `button` can be found in `input.h` or a similar library/crate.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    /// use rustwlc::types::{KeyboardModifiers, ButtonState, Point};
    ///
    /// extern fn pointer_button(view: WlcView, time: u32, mods: &KeyboardModifiers, button: u32,
    ///                          state: ButtonState, point: &Point) -> bool {
    ///     println!("Button {} {:?} at {} at {} in {:?}, keyboard mods: {:?}",
    ///              button, state, time, point, view, mods);
    ///     return false;
    /// }
    /// # fn main() { }
    /// ```
    pub fn pointer_button(mut self, func: extern "C" fn(view: WlcView, time: u32,
                                                        mods: &KeyboardModifiers, button: u32,
                                                        state: ButtonState, point: &Point) -> bool)
                          -> WlcInterface {
        self.pointer.button = Some(func); self
    }

    /// Callback invoked on mouse scroll. Return `true` to block the scroll from the view.
    ///
    /// # Arguments
    /// The first u32 is a timestamp, the amount is measured in scrollx and scrolly.
    pub fn pointer_scroll(mut self, func: extern "C" fn(view: WlcView, time: u32,
                                                        mods: &KeyboardModifiers, axis: ScrollAxis,
                                                        amount: [u64; 2]) -> bool) -> WlcInterface {
        self.pointer.scroll = Some(func); self
    }

    /// Callback invoked on pointer motion. Return `true` to block the motion from the view.
    ///
    /// `rustwlc::input::pointer::set_position` must be invoked to actually move the cursor!
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    /// use rustwlc::types::Point;
    /// use rustwlc::input::pointer;
    ///
    /// extern fn pointer_motion(view: WlcView, time: u32, point: &Point) -> bool {
    ///     println!("Pointer was moved to {} in {:?} at {}", point, view, time);
    ///     // This is very important.
    ///     pointer::set_position(point);
    ///     return false;
    /// }
    /// # fn main() { }
    /// ```
    pub fn pointer_motion(mut self, func: extern "C" fn(view: WlcView,
                                                        time: u32,
                                                        point: &Point) -> bool) -> WlcInterface {
        self.pointer.motion = Some(func); self
    }

    /// Callback invoked on touchscreen touch. Return `true` to block the touch from the view.
    ///
    /// # Arguments
    /// * `mods`: Which keyboard modifiers are being pressed during the event
    /// * `touch`: What kind of event it is (a touch down, a frame being made, a touch release).
    /// In the case of `TouchType::Frame`, `slot` and `point` will both be zero.
    /// * `slot`: Which finger - in cases of multiple touches down - is causing the event
    /// * `point`: Where the touch event happened
    pub fn touch_touch(mut self, func: extern "C" fn(handle: WlcView, time: u32,
                                                     mods: &KeyboardModifiers, touch: TouchType,
                                                     slot: i32, point: &Point) -> bool)
                       -> WlcInterface {
        self.touch.touch = Some(func); self
    }

    /// Callback invoked by wlc after `rustwlc::init` is called.
    pub fn compositor_ready(mut self, func: extern "C" fn()) -> WlcInterface {
        self.compositor.ready = Some(func); self
    }

    /// Callback invoked by wlc when a compositor is terminating
    pub fn compositor_terminate(mut self, func: extern "C" fn()) -> WlcInterface {
        self.compositor.terminate = Some(func); self
    }

    // Not supporting input and output through the builder...
}
