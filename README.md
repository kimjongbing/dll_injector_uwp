# dll_injector_uwp
This was originally a normal [dll injector](https://github.com/kimjongbing/dll_injector) using the Windows API but it was not working on Microsoft Store applications (UWP) so I made this injector. This works on both UWP applications and normal applications. 


## Caution
Please make sure your DLL is on a hard drive with an NTFS file system. You will not be able to see the ``Security`` tab otherwise. Look at the [DLL Setup Permissions](#dll-setup-permissions) section to see how to set up the security permissions of your DLL. 


## Usage
1. **List all processes**: Run the following command to list all process IDs (PIDs).

    ```bash
    dll_injector.exe list
    ```
    This will return a list of all PIDs currently running on your system.

2. **Get PID of a running executable**: If you know the name of an executable and it's currently running, you can get its PID by running:

    ```bash
    dll_injector.exe pid <file.exe>
    ```
    Replace `<file.exe>` with the name of the running executable. This will return the PID of the specified executable.

    Example:
    ```bash
    dll_injector.exe pid notepad.exe
    ```

3. **Inject a DLL into a process**: To inject a DLL into a process, you need the PID of the target process and the path to the DLL you want to inject. Use the following command:

    ```bash
    dll_injector.exe inject <pid> <payload.dll>
    ```
    Replace `<pid>` with the PID of the target process and `<payload.dll>` with the path to the DLL you want to inject.

    Example:
    ```bash
    dll_injector.exe inject 1234 C:/path/to/your/payload.dll
    ```


## Compile Guide

1. **Clone the repository**: Run the following command in the terminal to download the source code to your local machine.

    ```bash
    git clone https://github.com/kimjongbing/dll_injector_uwp
    ```

2. **Navigate to the project folder**: Change the current directory to the project's root folder by running:

    ```bash
    cd dll_injector_uwp-main
    ```

3. **Compile the project**: Build the project in release mode by running the following command in the root directory of the project.

    ```bash
    cargo build --release
    ```

4. **Navigate to the build output**: Change directory to where the built executable is located by running:

    ```bash
    cd ./target/release
    ```

5. **Run the executable**: Start the application by running `dll_injector.exe`. Upon execution, the console will display usage instructions. You can then provide the necessary arguments based on what you want to do.

This is a command-line application, so all inputs (arguments) should be passed in the command line. Run the command-line as administrator if you have any issues.


## DLL Setup Permissions
**For now you have to do this manually until I set this to be done automatically for you**
1.  Right-click the ``.dll`` file you want to inject.
2.  Click the ``Security`` tab.
3.  Click ``Edit`` next to ``To change permissions, click Edit:``
4.  Click ``Add`` and type ``All`` in the box called ``Enter the objects names to select (examples):``
5.  Click OK
6.  Make sure it has ``Read & execute`` and ``Read`` enabled 

