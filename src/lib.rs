#![warn(missing_docs)]

//! Module defining main wlc functions.
#![allow(improper_ctypes)] // We get warnings on WlcInterface

extern crate libc;

#[macro_use]
extern crate bitflags;

use std::ffi;
use std::ptr;
use std::ffi::CString;

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

    fn wlc_init(interface: *const WlcInterface, argc: i32, argv: *const *mut libc::c_char) -> bool;

    fn wlc_run();

    fn wlc_terminate();

    fn wlc_log_set_handler(callback: extern "C" fn(log_type: LogType, text: *const libc::c_char));
}

/// Initialize wlc with a `WlcInterface`.
///
/// Create a WlcInterface with the proper callback methods
/// and call `rustwlc::init` to initialize wlc (alternatively use init_with_args).
/// If it returns true, continue with `rustwlc::run_wlc()` to run wlc's event loop.
///
/// # Example
/// ```no_run
/// let interface = rustwlc::interface::WlcInterface::new();
/// // Set a default log callback
/// rustwlc::log_set_default_handler();
///
/// if !rustwlc::init(interface) {
///      panic!("Unable to init");
/// }
/// rustwlc::run_wlc();
/// ```
pub fn init(interface: WlcInterface) -> Option<fn() -> ()> {
    fn run_wlc() {
        unsafe { wlc_run() }
    }
    unsafe {
        if wlc_init(&interface, 0, ptr::null()) {
            Some(run_wlc)
        } else {
            None
        }
    }
}

/// Runs wlc's event loop.
///
/// The initialize functions will return this function in an Option.
/// If and only if they succeed can this function be called wlc with `rustwlc::init` call this method
/// to being wlc's main event loop.
///
/// # Example
/// ```no_run
/// let interface = rustwlc::interface::WlcInterface::new();
/// rustwlc::log_set_default_handler();
///
/// if !rustwlc::init(interface) {
///      panic!("Unable to init");
/// }
/// rustwlc::run_wlc();
/// ```
fn run_wlc() {
    unsafe {
        wlc_run();
    }
}

/// Executes a program in wayland.
/// Is passed the program and all arguments (the first should be the program)
pub fn exec(bin: String, args: Vec<String>) {
    unsafe {
        let bin_c = CString::new(bin).unwrap().as_ptr() as *const libc::c_char;

        let argv: Vec<CString> = args.into_iter()
                                     .map(|arg| CString::new(arg).unwrap())
                                     .collect();

        let args: Vec<*const libc::c_char> = argv.into_iter()
                                                 .map(|arg: CString| {
                                                     arg.as_ptr() as *const libc::c_char
                                                 })
                                                 .collect();

        wlc_exec(bin_c, args.as_ptr() as *const *const libc::c_char);
    }
}

/// Halts execution of wlc.
pub fn terminate() {
    unsafe {
        wlc_terminate();
    }
}

/// Registers a callback for wlc logging
#[allow(dead_code)]
pub fn log_set_handler(handler: extern "C" fn(type_: LogType, text: *const libc::c_char)) {
    unsafe {
        wlc_log_set_handler(handler);
    }
}

#[allow(dead_code)]
extern "C" fn default_log_callback(log_type: LogType, text: *const libc::c_char) {
    let string_text = unsafe { pointer_to_string(text) };
    println!("wlc [{:?}] {}", log_type, string_text);
}

/// Sets the wlc log callback to a simple function that prints to console.
///
/// Not calling any log_set_handler will have no logging, use this or
/// log_set_handler with a callback to use wlc logging.
///
/// # Example
/// ```no_run
/// let interface = rustwlc::interface::WlcInterface::new();
/// rustwlc::log_set_default_handler();
/// if !rustwlc::init(interface) {
///      panic!("Unable to init");
/// }
/// rustwlc::run_wlc();
/// ```
pub fn log_set_default_handler() {
    log_set_handler(default_log_callback);
}

/// Converts a `*const libc::c_char` to an owned `String`.
/// Useful for log callbacks.
pub unsafe fn pointer_to_string(pointer: *const libc::c_char) -> String {
    let slice = ffi::CStr::from_ptr(pointer);
    slice.to_string_lossy().into_owned()
}
