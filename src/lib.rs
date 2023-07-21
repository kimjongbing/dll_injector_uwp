#[cxx::bridge]
pub mod dll_injector {
  unsafe extern "C++" {
    include!("F:/resources/codingstuff/dll_injector_uwp/cpp/bridge.h"); 

    fn launch_process(exe_path: String) -> i32;
    fn load_dll(dll_path: String) -> i32;
    fn enumerate_processes() -> i32;
    fn inject_dll(pid: i32, dll_path: String) -> i32;
  }
}

