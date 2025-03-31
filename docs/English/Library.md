plaintext  
# How to call some APIs of mcospkg
As an open-source project, it's definitely necessary to write APIs, right [doge]

Regarding this, we have written some interfaces that can help you use certain functions without directly using commands.

## What are they specifically?
For example, installation (install) and uninstallation (remove) are definitely essential parts for a package manager.

So, there are the following functions:
 - `install_pkg`,
 - `remove_pkg`

Next, I will tell you how to use these functions correctly.

## Installation
### Function Prototype
**This is the function prototype in Rust:**

```rust
pub fn rust_install_pkg(packages: Vec<Package>) -> i32
```
 
(Of course, this  Package  is a struct that has been defined in  src/lib.rs . In future versions, we will implement some methods for it.)
 
[Note: This is not the focus of this document. If you want to know about it, please execute  cargo doc --no-deps --open  to view the relevant help. (Of course, this is in English QwQ).]
 
**This is the function defined in Rust for the C language:**
 
```rust
#[unsafe(no_mangle)]
pub extern "C" fn c_install_pkg(
    package_id: *const c_char,
    package_path: *const c_char,
    version: *const c_char,
) -> c_int
```
 
**It is roughly like this when converted into the C language style:**
 
```c
int c_install_pkg(
    char* package_id,
    char* package_path,
    char* version,
);
```
 
 
### Field Explanation
 
For the function facing the C language, there are generally the following fields:
- `package_id`,
- `package_path`,
- `version`
 
Now, let's explain these fields according to the specific functions of the function:
 
`package_id` -- It is the unique identifier of a package
(similar to "python", "com.example", etc.);
 
`package_path` -- Specifies the path of the compressed package of a package
(For example: "/var/cache/mcospkg/python.tar.xz");
 
`version` -- Specifies the version number of a package
(For example: "0.1.0", "3.12.8")
 
Attention!! The version number can only contain numbers and decimal points, and cannot contain any other characters except them!! Otherwise, we cannot determine the newness or oldness of the version number!!
 
### Example Usage
 
In the Rust language:
 
```rust
// Assume that the main function has been written and the relevant functions have been imported.
let package_id = "mcospkg";
let package_path = "/path/to/mcospkg.tar.xz";
let version = "0.9.1";

// Convert it into a Package struct. (Of course, we will implement some methods to convert it more conveniently in the future.)
let package = Package {
    package_id,
    package_path,
    version,
};

// Wrap it with a Vector.
let packages: Vec<Package> = vec![package];

// Finally, call the function. (Here we don't receive the return value.)
let _ = rust_install_pkg(packages);
```
 
In the C language:
 
```c
// Assume that the main function has been written, and the function has been correctly declared and can be linked properly.
char* package_id = "python";
char* package_path = "/path/to/mcospkg.tar.xz";
char* version = "0.9.1";

// Call the function directly.
int status = c_install_pkg(package_id, package_path, version);

// Some processing can be done on the status later. For example:
if (status == 0) {
    printf("The software package is installed successfully\n");
} else {
    printf("The software package installation fails\n");
}
```
 
 
## Uninstallation
### Function Prototype
 
The declarations of the functions in Rust and C are almost the same.
 
For example, in Rust:
 
```rust
pub fn rust_remove_pkg(packages: Vec<String>) -> i32
```
 
The function defined in Rust for the C language:
 
```rust
#[unsafe(no_mangle)]
pub extern "C" fn c_remove_pkg(
    package_name: *const c_char,
) -> c_int
```
 
It is roughly like this when converted into the C language style:
 
```c
int c_remove_pkg(char* package_name);
```
 
### Field Explanation
 
For the function facing the C language, there is only one field:
 
- `package_name`, which refers to the name of the package to be removed, that is, the package_id used during the previous installation. Through this name, mcospkg can locate the specific software package to be uninstalled.
 
### Example Usage
 
In the Rust language:
```rust
// Assume that the main function has been written and the relevant functions have been imported.
let package_name = "mcospkg".to_string();
// Wrap it with a Vector.
let packages: Vec<String> = vec![package_name];
// Call the function. (Here we don't receive the return value.)
let _ = rust_remove_pkg(packages);
```
 
In the C language:
 
```c
// Assume that the main function has been written, and the function has been correctly declared and can be linked properly.
char* package_name = "python";
// Call the function directly.
int status = c_remove_pkg(package_name);
// Some processing can be done on the status later. For example:
if (status == 0) {
    printf("The software package is uninstalled successfully\n");
} else {
    printf("The software package uninstallation fails\n");
}
```

# How to Compile?

You have written the relevant code, and surely it needs to be compiled. But how should you do it? You may not know what the compilation command is either.

Well, think about it. If you have looked at the  install.sh  file (installation script) under this project, you will find that there is a copy command in it. If you know the copy command, don't you know how to do the linking? [doge] [doge]

~~(At this moment, you might be saying: I really want to curse the author *****)~~

Since you know how to do the linking, you should also think of using this command:

```bash
gcc your_code.c -lmcospkg -o your_code        # Make sure the `-lmcospkg` option is specified.
```

Of course, the  `libmcospkg.so`  is installed under `$(PREFIX)/lib` . If you have specified a different installation path, remember to add the  `-L <path>`  option as well!

# Notes
 
1. When using these functions, ensure the accuracy of the paths. Whether it is the path of the installation package (package_path) or the management path of the software package itself, an incorrect path may lead to the failure of function execution. For example, the installation package cannot be found or the software package to be uninstalled cannot be located.
 
2. When calling these functions in the C language, pay attention to memory management. Since character pointers are passed, make sure there is no memory leak after use. For example, if the string memory allocated for package_id, package_path, etc. through malloc, it needs to be freed by calling free when it is no longer in use.
 
3. Since these functions involve the installation and uninstallation operations of software packages, there may be some permission requirements. In the actual operating environment, ensure that the program runs with sufficient permissions ( Such as using `sudo` privilege ). Otherwise, the installation or uninstallation may fail due to insufficient permissions.
 
4. For the processing of the version number, it must strictly follow the specified format. As mentioned above, the version number can only contain numbers and decimal points. Check carefully when entering the version number to avoid problems in installation or other version-related operations due to incorrect version number formats.
