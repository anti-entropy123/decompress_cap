extern crate dunce;
use std::{env, path::PathBuf, process::Command};

fn add_lib(lib_name: &str, lib_path: &str) {
    let library_name = lib_name;
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let library_dir = dunce::canonicalize(root.join(lib_path)).unwrap();
    println!("cargo:rustc-link-lib=static={}", library_name);
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[library_dir]).unwrap().to_str().unwrap()
    );
}

fn cheri_gen_bind(lib_path: &str) {
    let dir = PathBuf::from(lib_path);

    if !Command::new("clang")
        .args([
            "-nostdlib",
            "-c",
            dir.join("wrapper.c").to_str().unwrap(),
            "-o",
            dir.join("cheri.o").to_str().unwrap(),
        ])
        .output()
        .expect("compile failed.")
        .status
        .success()
    {
        panic!("could not compile object file")
    };

    if !Command::new("ar")
        .args([
            "-r",
            dir.join("libcheri.a").to_str().unwrap(),
            dir.join("cheri.o").to_str().unwrap(),
        ])
        .output()
        .expect("compile failed.")
        .status
        .success()
    {
        panic!("could not create static library")
    }

    let bindings = bindgen::Builder::default()
        .header("cheri-compressed-cap/cheri_compressed_cap.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    cheri_gen_bind("binding");
    add_lib("cheri", "binding");
}
