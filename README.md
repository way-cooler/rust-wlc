# rust-wlc
Rust bindings for [wlc](https://github.com/Cloudef/wlc), the Wayland compositor library.

rust-wlc is written in 100% Rust and  to be a narrow wrapper around wlc. 

### Rust Example

```rust
// For more functional example see example/src/main.rs

extern crate rustwlc;
use rustwlc::interface::WlcView;
use rustwlc::types::*;

// Callbacsk must be labeled extern as they will be called from C
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

    if !rustwlc::init(interface) {
        panic!("Unable to initialize!");
    }
    
    rustwlc::run_wlc();
}
```

### Usage
```toml
[depdenencies]
rustwlc = { git = "https://github.com/ImmingtonIndustries/rust-wlc.git" }
```
Make sure `rustc` can find wlc - either install it from a package manager (i.e. wlc-git on the AUR for Arch users) or see their instructions.

### Documentation
At the moment, we have Cargo documentation. 
```shell
$ git clone "https://github.com/ImmingtonIndustries/rust-wlc.git"
$ cd rust-wlc
$ cargo doc
```
And look at the files in your web browser. If the documentation isn't clear enough or in the wrong places, please let us know.

### Safety
`rust-wlc` is written to be a clean Rust wrapper around wlc. While we've taken the liberty to make the code more Rust-friendly (such as creating instance methods for `WlcView` and `WlcOutput`) we did not try to extend wlc itself. 

The callbacks registered from `WlcInterface` must be labeled `extern` (or `extern "C"`) because they are called from C code. In addition, as per the Rust spec, panicing from C is undefined behavior (although it's worked for us).

You should be able to use our code without the need for `unsafe` blocks (as we do in way-cooler). One thing we found was that the callback structure necessitated the use of global mutable state, i.e. for the compositor to keep track of whether the user was resizing or not. See the example for details.

## Contributing
We accept pull requests! If you find a bug or would like to contribute (wlc isn't versioned, we may be a few commits behind their API) please submit a request.
