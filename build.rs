fn main() {

  cxx_build::bridge("src/lib.rs")
    .file("F:/resources/codingstuff/dll_injector/cpp/bridge.cpp")
    .include("F:/resources/codingstuff/dll_injector/cpp")
    .compile("dll_injector");

  println!("cargo:rerun-if-changed=src/lib.rs");
  println!("cargo:rerun-if-changed=cpp/bridge.cpp");
  println!("cargo:rerun-if-changed=cpp/bridge.h");
}