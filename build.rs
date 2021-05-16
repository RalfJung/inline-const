extern crate autocfg;

fn main() {
    let ac = autocfg::new();

    // Check for a minimum version for a few features
    if ac.probe_rustc_version(1, 51) {
        println!("cargo:rustc-cfg=const_generics");
    }
}
