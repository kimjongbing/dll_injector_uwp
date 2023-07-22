fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("cpp/bridge.cpp")
        .include(".")
        .compile("dll_injector_uwp");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/bridge.h");
}
