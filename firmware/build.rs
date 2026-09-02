use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Select the memory layout by board feature (embedded targets only).
    if env::var("CARGO_FEATURE_RP2040").is_ok() {
        File::create(out.join("memory.x"))
            .unwrap()
            .write_all(include_bytes!("memory-rp2040.x"))
            .unwrap();
        println!("cargo:rerun-if-changed=memory-rp2040.x");
    } else if env::var("CARGO_FEATURE_RP2350").is_ok() {
        File::create(out.join("memory.x"))
            .unwrap()
            .write_all(include_bytes!("memory-rp2350.x"))
            .unwrap();
        println!("cargo:rerun-if-changed=memory-rp2350.x");
    }

    // The linker search path and the RP2040 boot2 link script are only
    // relevant for bare-metal builds; the host `cargo test` build must not
    // receive embedded linker flags (clang rejects --nmagic / -T).
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "none" {
        println!("cargo:rustc-link-search={}", out.display());
        // RP2040 needs the boot2 section, provided by embassy-rp's link-rp.x.
        if env::var("CARGO_FEATURE_RP2040").is_ok() {
            println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
