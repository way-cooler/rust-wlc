// This code will be used later
#![allow(dead_code)]
#![feature(libc)]
extern crate libc;

pub mod handle;
pub mod types;
pub mod wayland;

// External WLC functions
#[link(name = "wlc")]
extern "C" {
    fn wlc_exec(bin: *const libc::c_char, args: *const *const libc::c_char) -> ();

    fn wlc_init(interface: *const WlcInterface, argc: i32, argv: *const *const libc::c_char) -> bool;

    /// Runs WLC event loop
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
