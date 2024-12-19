use uniffi;
use windres;

fn main() {
    // Build uniffi
    uniffi::generate_scaffolding("src/flm_ffi.udl").unwrap();
    
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        windres::Build::new().compile("AGWinFLM.rc").unwrap();
    }
}
