use std::ffi::CStr;

use dll_injector::dll_injector;
use winapi::{
    shared::minwindef::{DWORD, FALSE},
    um::{
        handleapi::CloseHandle,
        tlhelp32::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
    },
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  list - list running processes");
        println!("  pid <exe> - get PID of process");
        println!("  inject <pid> <dll> - inject DLL into process");
        return;
    }

    match args[1].as_str() {
        "list" => {
            dll_injector::enumerate_processes();
        }
        "pid" => {
            if args.len() < 3 {
                println!("Need to specify exe file");
                return;
            }
            let exe = &args[2];
            match get_process_id_by_name(exe) {
                Some(pid) => println!("PID: {}", pid),
                None => println!("No process found with name: {}", exe),
            }
        }
        "inject" => {
            if args.len() < 4 {
                println!("Need to specify PID and DLL");
                return;
            }
            let pid = args[2].parse::<i32>().unwrap();
            let dll = &args[3];
            dll_injector::inject_dll(pid, dll.to_string());
        }
        _ => println!("Invalid command"),
    }
}

fn get_process_id_by_name(exe_name: &str) -> Option<DWORD> {
    unsafe {
        let h_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        if h_snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            eprintln!("Failed to create snapshot of current processes.");
            return None;
        }

        let mut pe32: PROCESSENTRY32 = std::mem::zeroed();
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;

        if Process32First(h_snapshot, &mut pe32) == FALSE {
            eprintln!("Failed to gather information about the first process.");
            CloseHandle(h_snapshot);
            return None;
        }

        loop {
            let process_name = CStr::from_ptr(pe32.szExeFile.as_ptr())
                .to_string_lossy()
                .into_owned();

            if process_name.to_lowercase() == exe_name.to_lowercase() {
                println!(
                    "Found process: {}, Process ID: {}",
                    process_name, pe32.th32ProcessID
                );
                CloseHandle(h_snapshot);
                return Some(pe32.th32ProcessID);
            }

            if Process32Next(h_snapshot, &mut pe32) == FALSE {
                eprintln!("No process found with name: {}", exe_name);
                CloseHandle(h_snapshot);
                return None;
            }
        }
    }
}
