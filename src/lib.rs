// This code will be used later
#![allow(dead_code)]
extern crate libc;

use std::env;
use std::ffi;
use std::ffi::{CString};

pub mod handle;
pub mod interface;
pub mod types;
pub mod input;
pub mod wayland;

use types::LogType;
use interface::WlcInterface;

// External WLC functions
#[link(name = "wlc")]
extern "C" {
    fn wlc_exec(bin: *const libc::c_char, args: *const *const libc::c_char);

    fn wlc_init(interface: *const WlcInterface, argc: i32, argv: *const *const libc::c_char) -> bool;

    fn wlc_run();

    fn wlc_terminate();

    fn wlc_log_set_handler(callback: extern fn(log_type: LogType, text: *const libc::c_char));
}

/// Initialize wlc with a `WlcInterface`.
///
/// Create a WlcInterface with the proper callback methods
/// and call `rustwlc::init` to initialize wlc. If it returns
/// true, continue with `rustwlc::run_wlc()` to run wlc's event loop.
pub fn init(interface: WlcInterface) -> bool {
    log_set_handler(default_log_callback);
    unsafe {
        let args: Vec<*const libc::c_char> = env::args().into_iter()
            .map(|arg| arg.as_ptr() as *const libc::c_char ).collect();

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

/// Executes a program in wayland.
/// Is passed the program and all arguments (the first should be the program)
pub fn exec(bin: String, args: Vec<String>) {
    unsafe {

        //let bin_c = CString::new(bin).unwrap().into_raw();

        let bin_c = CString::new(bin).unwrap().as_ptr() as *const libc::c_char;

        let argv: Vec<CString> = args.into_iter()
            .map(|arg| CString::new(arg).unwrap() ).collect();

        let args: Vec<*const libc::c_char> = argv.into_iter()
            .map(|arg: CString| { arg.as_ptr() as *const libc::c_char }).collect();

        wlc_exec(bin_c, args.as_ptr() as *const *const libc::c_char);
    }
}

/// Halts execution of wlc.
pub fn terminate() {
    unsafe { wlc_terminate(); }
}

pub fn log_set_handler(handler: extern fn(type_: LogType, text: *const libc::c_char)) {
    unsafe { wlc_log_set_handler(handler); }
}

extern fn default_log_callback(log_type: LogType, text: *const libc::c_char) {
	let string_text = pointer_to_string(text);
	// Add fancier logging, with debug levels and all that. Find a nice library
	println!("wlc log: {:?}: {}", log_type, string_text);
}


pub fn pointer_to_string(pointer: *const libc::c_char) -> String {
    let slice = unsafe { ffi::CStr::from_ptr(pointer) };
    slice.to_string_lossy().into_owned()
}
