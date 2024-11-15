use cbindgen::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn build_proto(crate_path: PathBuf) {
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

    let mut src_path = crate_path.clone();
    src_path.push("src");

    let mut out_path = src_path.clone();
    out_path.push("protobuf_generated");

    let mut protobuf_path = src_path.clone();
    protobuf_path.push("protobuf");

    prost_build::Config::new()
        .out_dir(out_path)
        .compile_protos(
            &["flm_interface.proto", "configuration.proto"]
                .into_iter()
                .map(|file| {
                    let mut p = protobuf_path.clone();
                    p.push(file);

                    p
                })
                .collect::<Vec<PathBuf>>(),
            &[protobuf_path],
        )
        .unwrap();
}

fn run_cbindgen(crate_path: PathBuf) {
    let mut config_path = crate_path.clone();
    config_path.push("cbindgen.toml");

    let mut output_path = crate_path.clone();
    output_path.push("src");
    output_path.push("platforms");
    output_path.push("flm_native_interface.h");

    let config = Config::from_file(config_path).unwrap();
    cbindgen::Builder::new()
        .with_crate(&crate_path)
        .with_config(config)
        .generate()
        .unwrap()
        .write_to_file(output_path);
}

fn main() {
    let mut crate_path = env::current_dir().unwrap();
    crate_path.push(file!());
    crate_path.pop();
    crate_path.pop();

    run_cbindgen(crate_path.clone());

    build_proto(crate_path.clone());
}
