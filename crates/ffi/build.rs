use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    if cfg!(feature = "win-res") {
        println!("cargo:rerun-if-changed=resources/AGWinFLM.rc.in");
        println!("cargo:rerun-if-env-changed=FLM_VERSION");
        println!("cargo:rerun-if-changed=../../.git/HEAD");
        println!("cargo:rerun-if-changed=../../.git/refs/heads");

        let rc_path = generate_rc(&resolve_version());

        #[cfg(all(windows, feature = "win-res"))]
        {
            let _ = windres::Build::new().compile(rc_path.to_str().unwrap());
        }

        let _ = &rc_path;
    }
}

/// Generates `AGWinFLM.rc` from `AGWinFLM.rc.in` into `OUT_DIR`, substituting
/// the version, and returns the path to the generated file.
fn generate_rc(version: &str) -> PathBuf {
    let template = fs::read_to_string("resources/AGWinFLM.rc.in")
        .expect("failed to read resources/AGWinFLM.rc.in");
    let contents = template
        .replace("@VERSION_COMMAS@", &version_commas(version))
        .replace("@VERSION@", version);

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let rc_path = Path::new(&out_dir).join("AGWinFLM.rc");
    fs::write(&rc_path, contents).expect("failed to write generated AGWinFLM.rc");

    rc_path
}

/// Resolves the crate version: the `FLM_VERSION` env var, else `git describe`
/// over `v*` tags, else `CARGO_PKG_VERSION`. A leading `v` is stripped.
fn resolve_version() -> String {
    let version = std::env::var("FLM_VERSION")
        .ok()
        .filter(|v| !v.is_empty())
        .or_else(|| {
            let out = std::process::Command::new("git")
                .args(["describe", "--tags", "--match=v*", "--abbrev=0"])
                .output()
                .ok()?;
            let v = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            (out.status.success() && !v.is_empty()).then_some(v)
        })
        .unwrap_or_else(|| std::env::var("CARGO_PKG_VERSION").unwrap_or_default());

    version.strip_prefix('v').unwrap_or(&version).to_owned()
}

/// Builds a comma-separated `major,minor,patch,build` value for the Windows
/// `FILEVERSION` / `PRODUCTVERSION` fields, taking the leading numeric part of
/// each of the first four dot-separated components and padding with zero.
///
/// e.g. `2.6.13-3-gc233f78-modified` -> `2,6,13,0`.
fn version_commas(version: &str) -> String {
    let mut parts = [0u16; 4];
    for (slot, component) in parts.iter_mut().zip(version.split('.')) {
        let digits: String = component.chars().take_while(char::is_ascii_digit).collect();
        *slot = digits.parse().unwrap_or(0);
    }

    format!("{},{},{},{}", parts[0], parts[1], parts[2], parts[3])
}
