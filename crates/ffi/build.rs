use uniffi;

fn main() {
    // Build uniffi
    uniffi::generate_scaffolding("src/flm_ffi.udl").unwrap();
}
