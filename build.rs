fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .flag_if_supported("-Wno-deprecated-declarations")
        .include("vendor")
        .include("vendor/pdq/cpp")
        .file("vendor/pdq_wrapper.cpp")
        .file("vendor/pdq/cpp/common/pdqhashtypes.cpp")
        .file("vendor/pdq/cpp/common/pdqhamming.cpp")
        .file("vendor/pdq/cpp/common/pdqutils.cpp")
        .file("vendor/pdq/cpp/hashing/pdqhashing.cpp")
        .file("vendor/pdq/cpp/hashing/torben.cpp")
        .file("vendor/pdq/cpp/downscaling/downscaling.cpp")
        .compile("pdq-cpp");

    let bindings = bindgen::Builder::default()
        .header("vendor/pdq_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("bindgen failed");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("pdq_bindings.rs"))
        .expect("couldn't write bindings");

    println!("cargo:rerun-if-changed=vendor/pdq_wrapper.h");
    println!("cargo:rerun-if-changed=vendor/pdq_wrapper.cpp");
}
