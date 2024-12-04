use rustc_version::{version_meta, Channel};

fn main() {
    if matches!(version_meta().unwrap().channel, Channel::Nightly) {
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    }
}
