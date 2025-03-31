# Installation
**Required files in the package: HOOKS, UNHOOKS**

1. Extract the package file (`.tar.gz`).
2. Proceed with the installation.

## 1. Compilation - based Installation (There is a BUILD - SCRIPT in the extracted directory)
1. Execute the `BUILD` - SCRIPT for compilation.
2. Execute the `HOOKS` for installation operations.
3. Place the `UNHOOKS` in the uninstallation database. Use this script for uninstallation.

## 2. Root - directory Installation
1. Create a file index (for the files to be copied).
2. Perform file copying.
3. Execute the `HOOKS` for configuration. The installation is complete.
4. Save the created file index and the `UNHOOKS` file in the directory, and place them in the uninstallation database for uninstallation.

# Uninstallation
## Uninstall a Package Installed via Compilation
1. Locate the `UNHOOKS` file from the uninstallation database.
2. Execute the `UNHOOKS` file.

## Uninstall a Package Installed via Root - directory Installation
1. Locate the `UNHOOKS` file from the uninstallation database.
2. Execute the `UNHOOKS` file to clean up miscellaneous files.
3. Delete the copied files and folders one by one according to the file index created during installation.

