use std::env;
use std::path::PathBuf;
use uniffi;

fn main() {
    println!("cargo:rerun-if-changed=src/protobuf/*");

    prost_build::Config::new()
        .out_dir("src/protobuf_generated")
        .protoc_executable("/usr/local/bin/protoc") // TODO:
        .compile_protos(
            &[
                "src/protobuf/flm_interface.proto",
                "src/protobuf/configuration.proto",
            ],
            &["src/protobuf"],
        )
        .unwrap();

    uniffi::generate_scaffolding("src/flm_ffi.udl").unwrap();

    if let Ok(swift_lib_dir) = env::var("CARGO_CFG_SWIFT_LIB_DIR") {
        println!("cargo:warning=Linking with the static Swift library");

        println!("cargo:rustc-link-arg=-ObjC");
        println!("cargo:rustc-link-search={swift_lib_dir}");
        println!("cargo:rustc-link-lib=AdGuardFLM");

        let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let deps_dir = out_dir.join("../../../deps").canonicalize().unwrap();
        let deps_dir_str = deps_dir.display();
        println!("cargo:rustc-link-arg=-exported_symbols_list");
        println!("cargo:rustc-link-arg={deps_dir_str}/libAdGuardFLM.syms");
    }
}
