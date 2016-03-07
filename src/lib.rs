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
pub mod xkb;

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
/// use rustwlc;
///
/// let interface = rustwlc::interface::WlcInterface::new();
/// // Set a default log callback
/// rustwlc::log_set_default_handler();
///
/// if let Some(run_wlc) = rustwlc::init(interface) {
///      run_wlc()
/// }
/// else {
///      panic!("Unable to initialize wlc!");
/// }
/// ```
pub fn init(interface: WlcInterface) -> Option<fn() -> ()> {
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
fn run_wlc() {
    unsafe {
        wlc_run();
    }
}

/// Deprecated, do not use.
///
/// # Deprecated
/// This function does not seem to work across the FFI boundary, and Rust provides a
/// much better interface in the `std::command::Command` class to executing programs.
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

/// Registers a callback for wlc logging.
///
/// Note that `rustwlc::log_set_default_handler()` will register a simple callback
/// that will print the type and text to the console.
///
/// # Parameters
/// The `handler` callback has two parameters:
/// * `type`: The `LogType` of the message being printed.
/// * `text`: The text to be logged, currently in C form. One may call `rustwlc::pointer_to_string`
/// to convert it to a Rust String.
///
/// # Safety
/// The callback function (like other callbacks in `rustwlc`) must be marked as extern as it is called
/// from C code.
///
/// In addition, `unsafe` will be required to convert the text into a Rust String.
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
/// Not calling any `log_set_handler` will have no logging, use this or
/// `log_set_handler` with a callback to use wlc logging.
///
/// # Example
/// ```no_run
/// use rustwlc;
///
/// let interface = rustwlc::interface::WlcInterface::new();
/// rustwlc::log_set_default_handler();
///
/// if let Some(run_wlc) = rustwlc::init(interface) {
///      run_wlc();
/// }
/// else {
///     panic!("Unable to initialize wlc!");
/// }
/// ```
pub fn log_set_default_handler() {
    log_set_handler(default_log_callback);
}

/// Unsafe strings conversion function.
///
/// Converts a `*const libc::c_char` to an owned `String`.
/// Useful for log callbacks.
///
/// # Example
/// Standard usage may be for the log callbacks.
/// ```rust
/// extern "C" fn default_log_callback(log_type: LogType, text: *const libc::c_char) {
///     let string = unsafe { pointer_to_string(text) };
///     println!("wlc [{:?}]: {}", log_type, string);
/// }
/// ```
pub unsafe fn pointer_to_string(pointer: *const libc::c_char) -> String {
    let slice = ffi::CStr::from_ptr(pointer);
    slice.to_string_lossy().into_owned()
}
