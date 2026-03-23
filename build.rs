use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=vendor/pdq_wrapper.h");
    println!("cargo:rerun-if-changed=vendor/pdq_wrapper.cpp");
    println!("cargo:rerun-if-env-changed=WASI_SDK_PATH");

    let target = env::var("TARGET").expect("TARGET not set");
    let host   = env::var("HOST").expect("HOST not set");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .cpp_link_stdlib(None)
        .no_default_flags(true)
        .target(&target)
        .flag("-O3")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fno-exceptions")
        .flag("-std=c++11")
        .flag_if_supported("-Wno-deprecated-declarations")
        .include("vendor")
        .include("vendor/pdq/cpp")
        .file("vendor/pdq_wrapper.cpp")
        .file("vendor/pdq/cpp/common/pdqhamming.cpp")
        .file("vendor/pdq/cpp/common/pdqutils.cpp")
        .file("vendor/pdq/cpp/hashing/pdqhashing.cpp")
        .file("vendor/pdq/cpp/hashing/torben.cpp")
        .file("vendor/pdq/cpp/downscaling/downscaling.cpp");

    if target == "wasm32-wasip2" {
        env::var("WASI_SDK_PATH")
                .expect("WASI_SDK_PATH must be set for wasm32-wasip2 builds");    
    }

    build.compile("pdq-cpp");

    bindgen::Builder::default()
        .header("vendor/pdq_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-x").clang_arg("c")
        .clang_arg("-Ivendor")
        .clang_arg(format!("--target={host}"))
        .layout_tests(false)
        .generate()
        .expect("bindgen failed")
        .write_to_file(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("pdq_bindings.rs"),
        )
        .expect("couldn't write bindings");

}
