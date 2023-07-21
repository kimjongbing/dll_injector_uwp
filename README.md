# dll_injector_uwp
this was originally a normal dll injector using the windows api but it was not working on Microsoft Store applications (UWP) so i made this


!!this does not work right now!!
the file paths are also still hardcoded because im still debugging

usage(make nice later):

1. dll_injector.exe <list> : will list all pids
2. dll_injector.exe <pid> <file.exe> : will return pid of running exe
3. dll_injector.exe <pid> <payload.dll> : will inject into process
