use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let lacam_project_root_dir = manifest_dir.join("lacam3");

    let dst = Config::new(lacam_project_root_dir)
        .build_target("lacam3")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build/lacam3").display()
    );

    println!("cargo:rustc-link-lib=static=lacam3");

    let lacam3_actual_include_dir = manifest_dir.join("lacam3/lacam3/include");

    cxx_build::bridge("src/bridge.rs")
        .include(lacam3_actual_include_dir)
        .std("c++17")
        .compile("lacam_rs_bridge");

    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=lacam3/lacam3/include/lacam.hpp");
    println!("cargo:rerun-if-changed=lacam3/lacam3/CMakeLists.txt");
    println!("cargo:rerun-if-changed=lacam3/CMakeLists.txt");
}
