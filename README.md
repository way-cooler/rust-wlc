# rust-wlc
Rust bindings for [wlc](https://github.com/Cloudef/wlc), the Wayland compositor library.

For wlc more recent than [651ebc8](https://github.com/Cloudef/wlc/commit/651ebc8f7da750e77fd26f09182043e7e7c036c1) (add `wlc_view_get_visible_geometry`).
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
rustwlc = "0.1.0"
```
to your Cargo.toml.

You also need to setup the wlc library so that rust-wlc can see it. If you simply install the library, that will be sufficient.

If you are looking to use a custom version of wlc (to ensure compatiblity by building against a specific version or to build the library with debug symbols for example), then you simply need to set the `LD_LIBRARY_PATH` environment variable to the full path of the share library object file (.so).

### Documentation
At the moment, we have Cargo documentation hosted at our github.io site [here](http://immington-industries.github.io). 
You can also generate it with cargo doc:
```shell
$ git clone "https://github.com/ImmingtonIndustries/rust-wlc.git"
$ cd rust-wlc
$ cargo doc
```
If the documentation isn't clear enough or in the wrong places, please let us know.

### Safety
`rust-wlc` is written to be a clean Rust wrapper around wlc. While we've taken the liberty to make the code more Rust-friendly (such as creating instance methods for `WlcView` and `WlcOutput`) we did not try to extend wlc itself. 

The callbacks registered from `WlcInterface` must be labeled `extern` (or `extern "C"`) because they are called from C code. In addition, as per the Rust spec, panicking from C is undefined behavior (although it's worked for us).

Compositors using rust-wlc can do so without any `unsafe` code. The only exception to this is registering a callback for wlc's logging function exposed through  `rustwlc::log_set_handler` (the callback must take in a `*const libc::c_char`). We have provided a `println!`-powered default enabled with `rustwlc::log_set_default_handler()`.

One thing we found was that the callback structure necessitated the use of global mutable state, i.e. for the compositor to keep track of whether the user was resizing or not. See the example for details.

## Contributing
We accept pull requests! If you find a bug or would like to contribute (wlc isn't versioned, we may be a few commits behind their API) please submit an issue/pull request.
