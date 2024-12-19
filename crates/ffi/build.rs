use uniffi;

fn main() {
    // Build uniffi
    uniffi::generate_scaffolding("src/flm_ffi.udl").unwrap();
    compile_windows_resources();
}

#[cfg(target_os = "windows")]
fn compile_windows_resources() {
    use windres::Build;
    Build::new()
        .compile("AGWinFLM.rc")
        .expect("Failed to compile Windows resources");
}

#[cfg(not(target_os = "windows"))]
fn compile_windows_resources() {
    // No-op on non-Windows platforms
}
