use std::env;

fn main() {
    if env::var("CARGO_FEATURE_STATIC_WLC").is_ok() {
        println!("cargo:rustc-link-search=/usr/local/lib64");
        println!("cargo:rustc-link-lib=dylib=wayland-client");
        println!("cargo:rustc-link-lib=dylib=wayland-server");
        println!("cargo:rustc-link-lib=dylib=systemd");
        println!("cargo:rustc-link-lib=dylib=input");
        println!("cargo:rustc-link-lib=dylib=udev");
        println!("cargo:rustc-link-lib=dylib=GLESv2");
        println!("cargo:rustc-link-lib=dylib=drm");
        println!("cargo:rustc-link-lib=dylib=gbm");
        println!("cargo:rustc-link-lib=dylib=xcb");
        println!("cargo:rustc-link-lib=dylib=xcb-composite");
        println!("cargo:rustc-link-lib=dylib=xcb-ewmh");
        println!("cargo:rustc-link-lib=dylib=xcb-xkb");
        println!("cargo:rustc-link-lib=dylib=xcb-image");
        println!("cargo:rustc-link-lib=dylib=xcb-xfixes");
        println!("cargo:rustc-link-lib=dylib=pixman-1");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=X11-xcb");
        println!("cargo:rustc-link-lib=dylib=EGL");
        println!("cargo:rustc-link-search=native=/usr/include");
        println!("cargo:rustc-link-lib=static=chck-atlas");
        println!("cargo:rustc-link-lib=static=chck-pool");
        println!("cargo:rustc-link-lib=static=chck-buffer");
        println!("cargo:rustc-link-lib=static=chck-buffer");
        println!("cargo:rustc-link-lib=static=chck-dl");
        println!("cargo:rustc-link-lib=static=chck-fs");
        println!("cargo:rustc-link-lib=static=chck-lut");
        println!("cargo:rustc-link-lib=static=chck-pool");
        println!("cargo:rustc-link-lib=static=chck-sjis" );
        println!("cargo:rustc-link-lib=static=chck-string");
        println!("cargo:rustc-link-lib=static=chck-tqueue");
        println!("cargo:rustc-link-lib=static=chck-unicode");
        println!("cargo:rustc-link-lib=static=chck-xdg");
        println!("cargo:rustc-link-lib=static=wlc-protos");
        println!("cargo:include=/home/timidger/include");
    }
}
