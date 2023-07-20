extern crate winapi;

use std::env;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;
use widestring::U16CString;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use winapi::um::winnt::{MEM_COMMIT, PAGE_READWRITE, PROCESS_ALL_ACCESS};

fn enumerate_processes() {
    unsafe {
        let h_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        if h_snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            eprintln!("Failed to create snapshot of current processes.");
            return;
        }

        let mut pe32: PROCESSENTRY32 = std::mem::zeroed();
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;

        if Process32First(h_snapshot, &mut pe32) == FALSE {
            eprintln!("Failed to gather information about the first process.");
            CloseHandle(h_snapshot);
            return;
        }

        loop {
            let process_name = CStr::from_ptr(pe32.szExeFile.as_ptr())
                .to_string_lossy()
                .into_owned();

            println!(
                "Process ID: {}, Process Name: {}",
                pe32.th32ProcessID, process_name
            );

            if Process32Next(h_snapshot, &mut pe32) == FALSE {
                break;
            }
        }

        CloseHandle(h_snapshot);
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

fn inject_dll(pid: DWORD, dll_path: &str) {
    let dll_path_cstring = CString::new(dll_path.to_string()).expect("CString::new failed");

    unsafe {
        let process = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        if process.is_null() {
            eprintln!("Failed to open the target process.");
            return;
        }

        let addr = VirtualAllocEx(
            process,
            null_mut(),
            dll_path_cstring.to_bytes_with_nul().len(),
            MEM_COMMIT,
            PAGE_READWRITE,
        );
        if addr.is_null() {
            eprintln!("Failed to allocate memory in the target process.");
            return;
        }

        if winapi::um::memoryapi::WriteProcessMemory(
            process,
            addr,
            dll_path_cstring.as_ptr() as *const _,
            dll_path_cstring.to_bytes_with_nul().len(),
            null_mut(),
        ) == 0
        {
            eprintln!("Failed to write into the target process memory.");
            return;
        }

        let kernel32 = CString::new("kernel32.dll").expect("CString::new failed");
        let loadlibrarya = CString::new("LoadLibraryA").expect("CString::new failed");

        let h_kernel32 = GetModuleHandleA(kernel32.as_ptr());
        if h_kernel32.is_null() {
            eprintln!("Failed to get the handle of kernel32.dll.");
            return;
        }

        let h_loadlibrarya =
            winapi::um::libloaderapi::GetProcAddress(h_kernel32, loadlibrarya.as_ptr());
        if h_loadlibrarya.is_null() {
            eprintln!("Failed to get the address of LoadLibraryA.");
            return;
        }

        let thread = CreateRemoteThread(
            process,
            null_mut(),
            0,
            Some(std::mem::transmute(h_loadlibrarya)),
            addr as *mut _,
            0,
            null_mut(),
        );
        if thread.is_null() {
            eprintln!(
                "Failed to create a remote thread in the target process. Error: {}",
                GetLastError()
            );
            return;
        }

        println!("Successfully injected the DLL into the target process.");
        CloseHandle(thread);
        CloseHandle(process);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "list" {
        enumerate_processes();
    } else if args.len() == 3 && args[1] == "pid" {
        match get_process_id_by_name(&args[2]) {
            Some(pid) => println!("PID of {}: {}", args[2], pid),
            None => println!("Process not found: {}", args[2]),
        }
    } else if args.len() == 3 && args[1].parse::<DWORD>().is_ok() {
        let pid = args[1].parse::<DWORD>().unwrap();
        inject_dll(pid, &args[2]);
    } else {
        println!("Usage:");
        println!("1. List all processes: cargo run list");
        println!("2. Get PID of a process by name: cargo run pid <process name>");
        println!("3. Inject DLL into a process: cargo run <PID> <DLL path>");
    }
}
