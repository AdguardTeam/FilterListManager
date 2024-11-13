use cbindgen;
use cbindgen::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use uniffi;

fn build_proto() {
    // Encountered problems, while running in IDE
    if cfg!(unix) && env::var_os("PROTOC").is_none() {
        if let Some(shell) = env::var_os("SHELL") {
            let result = Command::new(shell.as_os_str())
                .arg("-lc")
                .arg("which protoc")
                .output();

            if let Ok(output) = result {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let lines = stdout.lines().collect::<Vec<&str>>();
                    let path = lines
                        .first() // May have few lines
                        .unwrap()
                        .trim();

                    env::set_var("PROTOC", path);
                }
            }
        }
    }

    prost_build::Config::new()
        .out_dir("src/protobuf_generated")
        .compile_protos(
            &[
                "src/protobuf/flm_interface.proto",
                "src/protobuf/configuration.proto",
            ],
            &["src/protobuf"],
        )
        .unwrap();

    println!("cargo:rerun-if-changed=src/protobuf");
}

fn run_cbindgen() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config_path = PathBuf::from(&crate_dir);
    config_path.push("cbindgen.toml");

    let config = Config::from_file(config_path).unwrap();
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .unwrap()
        .write_to_file("src/platforms/flm_native_interface.h");

    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=src/native_interface");
}

fn main() {
    // Build protobuf for rust-side
    build_proto();

    // Build a header file for native_interface
    run_cbindgen();

    // Build uniffi
    uniffi::generate_scaffolding("src/flm_ffi.udl").unwrap();

    if let Ok(swift_lib_dir) = env::var("CARGO_CFG_SWIFT_LIB_DIR") {
        println!("cargo:warning=Linking with the static Swift library");

        println!("cargo:rustc-link-arg=-ObjC");
        println!("cargo:rustc-link-search={swift_lib_dir}");
        println!("cargo:rustc-link-lib=AdGuardFLM");

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let deps_dir = out_dir.join("../../../deps").canonicalize().unwrap();
        let deps_dir_str = deps_dir.display();
        println!("cargo:rustc-link-arg=-exported_symbols_list");
        println!("cargo:rustc-link-arg={deps_dir_str}/libAdGuardFLM.syms");
    }
}
