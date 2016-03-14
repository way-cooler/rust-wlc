# rust-wlc

[![Join the chat at https://gitter.im/Immington-Industries/rust-wlc](https://badges.gitter.im/Immington-Industries/rust-wlc.svg)](https://gitter.im/Immington-Industries/rust-wlc?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.2.1-orange.svg)](https://crates.io/crate/rustwlc)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Immington-Industries/rust-wlc/)

Rust bindings for [wlc](https://github.com/Cloudef/wlc), the Wayland compositor library.

Requires wlc more recent than [651ebc8](https://github.com/Cloudef/wlc/commit/651ebc8f7da750e77fd26f09182043e7e7c036c1) (add `wlc_view_get_visible_geometry`).
### Rust Example

```rust
// For more functional example see example/src/main.rs

extern crate rustwlc;
use rustwlc::interface::WlcView;
use rustwlc::types::*;

// Callbacks must be labeled extern as they will be called from C
extern fn view_created(view: WlcView) -> bool {
    view.bring_to_front();
    view.focus();
    return true;
}

extern fn view_focus(view: WlcView, focused: bool) {
    view.set_state(VIEW_ACTIVATED, focused);
}

fn main() {
    let interface = WlcInterface::new()
            .view_created(view_created)
            .view_focus(view_focus);

    // The default log handler will print wlc logs to stdout
    rustwlc::log_set_default_handler();
    let run_fn = rustwlc::init(interface).expect("Unable to initialize!");
    run_fn();
}
```

### Usage
We're on [crates.io](https://crates.io/crates/rustwlc), so to use the library simply add:
```toml
[depdenencies]
rustwlc = "0.2.1"
```
to your Cargo.toml.

You also need to setup the wlc library so that rust-wlc can see it. If you simply install the library, that will be sufficient.

If you are looking to use a custom version of wlc (to ensure compatiblity by building against a specific version or to build the library with debug symbols for example), then you simply need to set the `LD_LIBRARY_PATH` environment variable to the full path of the share library object file (.so). To verify that the correct shared library is seen by rust-wlc, use the `ldd` utility to view the full paths of shared libraries, the entry that starts with "libwlc.so" should point to your custom path.

So if you wlc install is at `/opt/wlc`, then the full path will probably be `/opt/wlc/target/src`. Notice that we only include the directory that containts the .so, not the .so itself. For more information on using DLLs in Linux, see [this link](http://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html#AEN77).

### Documentation
At the moment, we have Cargo documentation hosted at [our github.io site](http://immington-industries.github.io). 
You can also generate it with cargo doc:
```shell
$ git clone "https://github.com/ImmingtonIndustries/rust-wlc.git"
$ cd rust-wlc
$ cargo doc
```
If the documentation isn't clear enough or in the wrong places, please let us know.

### Safety
`rust-wlc` is written to be a clean Rust wrapper around wlc. While we've taken the liberty to make the code more Rust-friendly (such as creating instance methods for `WlcView` and `WlcOutput`), but we did not try to extend wlc itself. 

The callbacks registered from `WlcInterface` must be labeled `extern` (or `extern "C"`) because they are called from C code. In addition, as per the Rust spec, panicking from C is undefined behavior (although it's worked for us).

Compositors using rust-wlc can do so without any `unsafe` code. The only exception to this is registering a callback for wlc's logging function exposed through  `rustwlc::log_set_handler` (the callback must take in a `*const libc::c_char`). We have provided a `println!`-powered default enabled with `rustwlc::log_set_default_handler()`. In addition, the methods `get_user_data` and `set_user_data` in `WlcView` and `WlcOutput` are unsafe because they use C raw types (`void*`) underneath, and proper usage requires a deeper understanding of wlc itself.

## Contributing
We accept pull requests! If you find a bug or would like to contribute (wlc isn't versioned, we may be a few commits behind their API) please submit an issue/pull request.
