fn main() {
    // On Windows, copy libmpv-wrapper.dll and libmpv-2.dll from src-tauri/lib/
    // to the output directory so the plugin can find them at runtime during dev.
    #[cfg(target_os = "windows")]
    {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        // OUT_DIR is something like target/debug/build/<pkg>/out — go up to target/debug/
        let target_dir = std::path::Path::new(&out_dir)
            .ancestors()
            .nth(3)
            .unwrap()
            .to_path_buf();

        let lib_dir = std::path::Path::new("lib");
        for dll in &["libmpv-wrapper.dll", "libmpv-2.dll"] {
            let src = lib_dir.join(dll);
            if src.exists() {
                let dst = target_dir.join(dll);
                if !dst.exists() {
                    std::fs::copy(&src, &dst).ok();
                }
            }
        }
    }

    tauri_build::build()
}
