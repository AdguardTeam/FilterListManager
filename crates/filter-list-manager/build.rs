fn main() {
    println!("cargo:rustc-env=FLM_PKG_VERSION={}", resolve_version());
    println!("cargo:rerun-if-env-changed=FLM_VERSION");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/heads");
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
