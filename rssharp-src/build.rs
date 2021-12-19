use std::{
    env::var,
    path::{Path, PathBuf},
    process::Command,
};

fn build_sharp(_flags: &[&str], src_dir: &Path, out_dir: &Path) {
    run(Command::new("make")
        .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
        .current_dir(&src_dir));
    //run(Command::new("make").arg("install").current_dir(&src_dir));
    fs_extra::dir::copy(
        src_dir.join("lib"),
        out_dir.join("lib"),
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            copy_inside: true,
            depth: 0,
            content_only: false,
        },
    )
    .unwrap();
}

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => {
            if !status.success() {
                panic!("`{:?}` failed: {}", command, status);
            }
        }
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        }
    }
}

fn main() {
    let src_dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap()).join("libsharp2");
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let out_src_dir = out_dir.join("src");
    fs_extra::dir::copy(
        src_dir,
        &out_src_dir,
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            copy_inside: true,
            depth: 0,
            content_only: false,
        },
    )
    .unwrap();
    //if !out_dir.join("lib/libsharp.a").exists() {
    build_sharp(&[], &out_src_dir, &out_dir);
    //}
    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=sharp");
}
