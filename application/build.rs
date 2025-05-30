fn main() {
    println!("cargo:rustc-env=PROGRAM_NAME=FurinaOCR");
    println!("cargo:rustc-env=PROGRAM_VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=PROGRAM_AUTHOR={}", env!("CARGO_PKG_AUTHORS"));
    println!("cargo:rustc-env=PROGRAM_DESCRIPTION={}", env!("CARGO_PKG_DESCRIPTION"));
    println!("cargo:rustc-env=PROGRAM_REPOSITORY={}", env!("CARGO_PKG_REPOSITORY"));
    println!("cargo:rustc-env=PROGRAM_LICENSE={}", env!("CARGO_PKG_LICENSE"));
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() == "windows" {
        let mut res = winres::WindowsResource::new();
        // https://github.com/mxre/winres/pull/24
        // https://github.com/mxre/winres/issues/42
        #[cfg(target_os = "macos")]
        if std::env::var("CARGO_CFG_TARGET_ENV").unwrap().as_str() == "gnu" {
            res.set_ar_path("x86_64-w64-mingw32-ar");
            res.set_windres_path("x86_64-w64-mingw32-windres");
        }
        res.set_manifest_file("../assets/manifest.xml");
        res.compile().unwrap();
    }
}
