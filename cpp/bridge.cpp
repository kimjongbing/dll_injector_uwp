#include "bridge.h"
#include <windows.h>
#include <iostream>
#include <tlhelp32.h>
#include <string>
#include "rust/cxx.h"

DWORD WINAPI ThreadProc(LPVOID lpParameter) {
  HMODULE hModule = LoadLibraryA(static_cast<LPCSTR>(lpParameter));
  return 0;
}

std::string cpp_string_from_cxx(rust::cxxbridge1::String s) {
  return std::string(s.data(), s.size());
}

int32_t launch_process(rust::cxxbridge1::String exePath) {
  std::string exePathStr = cpp_string_from_cxx(exePath);

  PROCESS_INFORMATION pi;

  std::string s = cpp_string_from_cxx(exePath);

  int32_t wchars_num =
      MultiByteToWideChar(CP_ACP, 0, exePathStr.c_str(), -1, NULL, 0);
  wchar_t* wstr = new wchar_t[wchars_num];
  MultiByteToWideChar(CP_ACP, 0, exePathStr.c_str(), -1, wstr, wchars_num);

  STARTUPINFOW si;
  ZeroMemory(&si, sizeof(si));
  si.cb = sizeof(si);
  GetStartupInfoW(&si);

  if (!CreateProcessW(NULL, wstr, NULL, NULL, FALSE, CREATE_SUSPENDED, NULL,
                      NULL, &si, &pi)) {
    std::cout << "Error launching suspended process" << std::endl;
    delete[] wstr;
    return 1;
  }

  delete[] wstr;

  std::cout << "Launched suspended process successfully" << std::endl;

  ResumeThread(pi.hThread);

  Sleep(5000);

  CloseHandle(pi.hThread);
  CloseHandle(pi.hProcess);

  return 0;
}

int32_t load_dll(rust::cxxbridge1::String dllPath) {
  std::string dllPathStr = cpp_string_from_cxx(dllPath);
  DWORD result = load_dll(dllPathStr.c_str());

  HMODULE hDLL = LoadLibraryA(dllPath.c_str());
  if (!hDLL) {
    std::cout << "Error loading DLL" << GetLastError() << std::endl;
    return 1;
  }

  return 0;
}

int32_t enumerate_processes() {
  HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
  if (hSnapshot == INVALID_HANDLE_VALUE) {
    std::cout << "Error getting snapshot" << std::endl;
    return 1;
  }

  PROCESSENTRY32 pe;
  pe.dwSize = sizeof(PROCESSENTRY32);
  if (!Process32First(hSnapshot, &pe)) {
    std::cout << "Error getting first process" << std::endl;
    CloseHandle(hSnapshot);
    return 1;
  }

  while (true) {
    std::cout << pe.szExeFile << std::endl;

    if (!Process32Next(hSnapshot, &pe)) {
      break;
    }
  }

  CloseHandle(hSnapshot);

  return 0;
}

int32_t inject_dll(int32_t pid, rust::cxxbridge1::String dllPath) {
  std::cout << "Injecting: " << dllPath << std::endl;

  HMODULE dll = LoadLibraryA(dllPath.c_str());
  FARPROC threadAddr = GetProcAddress(dll, "ThreadProc");

  LPTHREAD_START_ROUTINE pThreadProc = (LPTHREAD_START_ROUTINE)threadAddr;

  std::cout << "ThreadProc address: " << threadAddr << std::endl;

  std::string s = cpp_string_from_cxx(dllPath);
  HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);

  if (!hProcess) {
    std::cout << "Error opening process" << std::endl;
    return 1;
  }

  size_t dllPathLen = strlen(dllPath.c_str()) + 1;
  LPVOID addr = VirtualAllocEx(hProcess, NULL, dllPathLen, MEM_COMMIT,
                               PAGE_EXECUTE_READWRITE);
  if (!addr) {
    std::cout << "Error allocating memory" << std::endl;
    CloseHandle(hProcess);
    return 1;
  }

  if (!WriteProcessMemory(hProcess, addr, dllPath.c_str(), dllPath.size() + 1,
                          NULL)) {
    std::cout << "Error writing to memory" << std::endl;
    CloseHandle(hProcess);
    return 1;
  }

  HANDLE hThread =
      CreateRemoteThread(hProcess, NULL, 0, pThreadProc, addr, 0, NULL);
  if (!hThread) {
    std::cout << "Error creating remote thread, error: " << GetLastError()
              << std::endl;
    CloseHandle(hProcess);
    return 1;
  }

  WaitForSingleObject(hThread, INFINITE);

  DWORD result = GetLastError();
  if (result == WAIT_FAILED) {
    std::cout << "Wait failed: " << result << std::endl;
  }

  CloseHandle(hThread);
  CloseHandle(hProcess);

  return 0;
}
