use uniffi;

fn main() {
    uniffi::generate_scaffolding("src/flm_ffi.udl").unwrap();
}
