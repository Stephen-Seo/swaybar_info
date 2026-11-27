use std::{env, fs::File, io::Write, path::PathBuf, process::Command};

fn main() {
    let cargo_manifest_dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let version_path = out_path.join("version.txt");
    let mut version_file =
        File::create(version_path).expect("Should be able to create version.txt at OUT_DIR");
    version_file
        .write_all(env!("CARGO_PKG_VERSION").as_bytes())
        .expect("Should be able to write to OUT_DIR/version.txt");

    let git_version_path = out_path.join("git_version.txt");
    let git_cmd_out_res = Command::new("git")
        .args(["describe", "--tags", "--long"])
        .output();

    if let Ok(git_cmd_out) = git_cmd_out_res {
        let git_version_file_res = File::create(git_version_path);
        if let Ok(mut git_version_file) = git_version_file_res {
            git_version_file
                .write_all(git_cmd_out.stdout.trim_ascii())
                .ok();
            println!("cargo::rerun-if-changed={cargo_manifest_dir}/.git");
        }
    }
}
