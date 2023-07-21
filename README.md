# dll_injector_uwp
This was originally a normal [dll injector](https://github.com/kimjongbing/dll_injector) using the Windows API but it was not working on Microsoft Store applications (UWP) so I made this.


## Usage
1. dll_injector.exe <list> : will list all pids
2. dll_injector.exe <pid> <file.exe> : will return pid of running exe
3. dll_injector.exe <pid> <payload.dll> : will inject into process


## File-setup
**This is temporary until I just make the C++ a library to import functions**
1. Go to ``lib.rs`` and change ``include!("F:/resources/codingstuff/dll_injector_uwp/cpp/bridge.h");`` to the file path of where bridge.h is on your computer
2. Do the same in ``build.rs``, change the file paths to match your system. 



## DLL Setup Permissions
**For now you have to do this manually until I set this to be done automatically for you**
1.  Right-click the ``.dll`` file you want to inject.
2.  Click the ``Security`` tab.
3.  Click ``Edit`` next to ``To change permissions, click Edit:``
4.  Click ``Add`` and type ``All`` in the box called ``Enter the objects names to select (examples):``
5.  Click OK
6.  Make sure it has ``Read & execute`` and ``Read`` enabled 

## Caution
Please make sure your DLL is on a hard drive with an NTFS file system. You will not be able to see the ``Security`` tab otherwise. 
