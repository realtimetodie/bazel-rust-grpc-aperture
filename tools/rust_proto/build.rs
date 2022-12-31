use std::env;
use std::fs;
use std::path::PathBuf;

// Runfiles lookup library from rules_rust.
extern crate runfiles;
use runfiles::Runfiles;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let r = Runfiles::create().unwrap();

    let root: String = env::var("PROTO_ROOT")
        .ok()
        .expect("Environment variable `PROTO_ROOT` is not defined");

    let manifest_filepath: PathBuf = env::var("PROTO_TYPES")
        .ok()
        .and_then(|str| str.parse().ok())
        .expect("Environment variable `PROTO_TYPES` is not defined");

    let manifest_content = fs::read_to_string(manifest_filepath)?;
    if manifest_content.is_empty() {
        panic!("Manifest file is empty");
    }

    // Google Protocol Buffer definition files (optional)
    let mut google_proto_files: Vec<PathBuf> = manifest_content
        .lines()
        .into_iter()
        .map(|p| r.rlocation(p.clone().strip_prefix("external/").unwrap_or(p.clone())))
        .collect();

    let mut proto_files: Vec<PathBuf> = glob::glob(&format!("../{}/**/*.proto", root))
        .expect("Failed to read glob pattern")
        .into_iter()
        .filter(|p| p.is_ok())
        .map(|p| p.unwrap())
        .collect();

    proto_files.append(&mut google_proto_files);

    // This can be replaced with `prost_build`
    // See https://crates.io/crates/prost-build
    tonic_build::configure().compile(
        &proto_files,
        &[
            "../",
            &r.rlocation("com_github_protocolbuffers_protobuf/src")
                .display()
                .to_string(),
        ],
    )?;

    Ok(())
}
