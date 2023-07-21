#pragma once
#include "rust/cxx.h"

int launch_process(rust::cxxbridge1::String exePath);

int load_dll(rust::cxxbridge1::String dllPath);

int enumerate_processes();

int inject_dll(int pid, rust::cxxbridge1::String dllPath);
