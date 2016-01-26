// This code will be used later
#![allow(dead_code)]
#![feature(libc)]
extern crate libc;

pub mod types;
pub mod wayland;

/// Represents the wlc callback interface.
/// wlc initialization involves registering a series of callbacks to the library
/// using this interface struct.
#[repr(C)]
#[no_mangle]
pub struct WlcInterface {
    pub output: OutputInterface,
    pub view: ViewInterface,
    pub keyboard: KeyboardInterface,
    pub pointer: PointerInterface,
    pub touch: TouchInterface,
    pub compositor: CompositorInterface,
    pub input: InputInterface
}

/// Represents window callbacks
#[repr(C)]
pub struct OutputInterface {
    pub created: Option<extern "C" fn(handle: WLCHandle) -> bool>,
    pub destroyed: InterfaceHandler,
    pub focus: Option<extern "C" fn(handle: WLCHandle, focused: bool) -> ()>,
    pub resolution: Option<extern "C" fn(handle: WLCHandle, old_size: WLCSize, new_size: WLCSize) -> ()>,
    pub render: RenderInterface,
}

/// Represents global rendering callbacks
#[repr(C)]
pub struct RenderInterface {
    pub pre: InterfaceHandler,
    pub post: InterfaceHandler,
}

/// Represents window viewing callbacks
#[repr(C)]
pub struct ViewInterface {
    pub created: Option<extern "C" fn(handle: WLCHandle) -> bool>,
    pub destroyed: InterfaceHandler,
    pub focus: Option<extern "C" fn(handle: WLCHandle, focused: bool) -> ()>,
    pub move_to_output: Option<extern "C" fn(current: WLCHandle, WLCHandle, WLCHandle) -> ()>,
    pub request: RequestInterface,
}

/// Represents window rendering callbacks
#[repr(C)]
pub struct RequestInterface {
    pub geometry: Option<extern "C" fn(handle: WLCHandle, geometry: Geometry) -> ()>,
    pub state: Option<extern "C" fn(current: WLCHandle, state: ViewState, handled: bool) -> ()>,
    pub move_: Option<extern "C" fn(handle: WLCHandle, destination: Point) -> ()>,
    pub resize: Option<extern "C" fn(handle: WLCHandle, edge: ResizeEdge, location: Point) -> ()>,
    pub render: RenderInterface,
}

/// Represents keyboard press callbacks
#[repr(C)]
pub struct KeyboardInterface {
    // WARNING TODO key and time might need to be switched in keyboard example
    pub key: Option<extern "C" fn(handle: WLCHandle, key: u32, mods: KeyboardModifiers, time: u32, state: KeyState) -> bool>,
}

/// Represents mouse input callbacks
#[repr(C)]
pub struct PointerInterface {
    pub button: Option<extern "C" fn(hande: WLCHandle, button: libc::c_uint, mods: KeyboardModifiers, time: u32, state: ButtonState, point: Point) -> bool>,
    pub scroll: Option<extern "C" fn(handle: WLCHandle, button: u32, mods: KeyboardModifiers, axis: ScrollAxis, heights: [u64; 2]) -> bool>,
    // dist?
    pub motion: Option<extern "C" fn(heights: WLCHandle, dist: u32, point: Point)>,
}

/// Represents touchscreen callbacks
#[repr(C)]
pub struct TouchInterface {
    /// NOTE WARNING TODO Not sure if key and touch need to be switched
    pub touch: Option<extern "C" fn(handle: WLCHandle, time: libc::c_uint, mods: KeyboardModifiers, touch: TouchType, key: libc::c_int, point: Point) -> bool>,
}

/// Represents a callback for initializing the callback
#[repr(C)]
pub struct CompositorInterface {
    pub ready: Option<extern "C" fn() -> ()>
}

/// Represents callbacks for window creation and destruction
#[repr(C)]
pub struct InputInterface {
    pub created: Option<extern "C" fn(device: LibinputDevice) -> bool>,
    pub destroyed: Option<extern "C" fn(device: LibinputDevice) -> ()>
}

/// Not currently supporting libinput
//#[repr(C)]
pub enum LibinputDevice {}

/// Represents a wayland display.
enum WLDisplay { }

/// Represents a wayland resource.
/// This object can be rendered in pre and post render hooks.
enum WLResource { }

// External WLC functions
#[link(name = "wlc")]
extern "C" {
    fn wlc_exec(bin: *const libc::c_char, args: *const *const libc::c_char) -> ();

    fn wlc_init(interface: *const WlcInterface, argc: i32, argv: *const *const libc::c_char) -> bool;

    /// Runs WLC run loop;
    fn wlc_run() -> ();
}

/// Initialize wlc with a `WlcInterface`.
///
/// Create a WlcInterface with the proper callback methods
/// and call `rustwlc::init` to initialize wlc. If it returns
/// true, continue with `rustwlc::run_wlc()` to run wlc's event loop.
pub fn init(interface: WlcInterface) -> bool {
    unsafe {
        let argv: Vec<ffi::CString> = env::args().into_iter()
            .map(|arg| ffi::CString::new(arg).unwrap() ).collect();

        let args: Vec<*const libc::c_char> = argv.into_iter()
            .map(|arg: ffi::CString| { arg.as_ptr() as *const libc::c_char }).collect();

        wlc_init(&interface, args.len() as i32, args.as_ptr() as *const *const libc::c_char)
    }
}

/// Runs wlc's event loop.
///
/// After initalizing wlc with `rustwlc::init` call this method
/// to being wlc's main event loop.
///
/// # Example
///
/// You should call `rustwlc::init` with a `WlcInterface` first.
///
/// ```no_run
/// # let interface: WlcInterface;
/// rustwlc::init(interface);
/// rustwlc::run_wlc();
/// ```
pub fn run_wlc() {
    unsafe { wlc_run(); }
}
