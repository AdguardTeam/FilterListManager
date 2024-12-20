use uniffi;

fn main() {
    // Build uniffi
    uniffi::generate_scaffolding("src/flm_ffi.udl").unwrap();

    #[cfg(windows)]
    {
        let _ = windres::Build::new().compile("resources/AGWinFLM.rc");
    }
}
