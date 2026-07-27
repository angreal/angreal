//! Build script for angreal.
//!
//! angreal embeds Python via PyO3 (`auto-initialize`). Any binary that links it
//! — notably the `cargo test` unit/integration harnesses — must find the Python
//! shared library at runtime. On macOS framework builds the dylib is loaded as
//! `@rpath/Python3.framework/Versions/3.x/Python3`, so without an rpath those
//! test binaries crash at launch with:
//!
//! ```text
//! dyld: Library not loaded: @rpath/Python3.framework/Versions/3.x/Python3
//! ```
//!
//! We emit the rpath from PyO3's own resolved build config, so it always matches
//! whichever interpreter PyO3 linked against — no hardcoded paths, no PATH or
//! DYLD_* workarounds. (The extension-module build uses dynamic lookup and does
//! not embed libpython, so the extra rpath there is a harmless no-op.)

fn main() {
    pyo3_build_config::use_pyo3_cfgs();

    let config = pyo3_build_config::get();
    if let Some(lib_dir) = &config.lib_dir {
        // Framework builds load the dylib as @rpath/Python3.framework/..., so the
        // rpath must point at the directory *containing* the framework, not the
        // inner lib dir.
        let rpath = if lib_dir.contains(".framework/") {
            let parts: Vec<&str> = lib_dir.splitn(2, ".framework/").collect();
            std::path::Path::new(parts[0])
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| lib_dir.clone())
        } else {
            lib_dir.clone()
        };

        println!("cargo:rustc-link-arg=-Wl,-rpath,{rpath}");
    }
}
