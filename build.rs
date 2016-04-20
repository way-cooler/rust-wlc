fn main() {
    let libs = &[(Some("../wlc/target/src/"), "wlc")];
    for &(ref m_path, ref lib) in libs {
        if let &Some(static_path) = m_path {
            println!("cargo:rustc-link-search={}", &static_path);
        }
        println!("cargo:rustc-link-lib={}", &lib);
    }
}
