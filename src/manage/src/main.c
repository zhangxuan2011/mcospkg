/***************************************************************************
 *   Copyright (C)                                                         *
 *   Email:                                                                *
 *                                                                         *
 *   This program is free software: you can redistribute it and/or modify  *
 *   it under the terms of the GNU General Public License as published by  *
 *   the Free Software Foundation, either version 3 of the License, or     *
 *   (at your option) any later version.                                   *
 *                                                                         *
 *   This program is distributed in the hope that it will be useful,       *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of        *
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the         *
 *   GNU General Public License for more details.                          *
 *                                                                         *
 *   You should have received a copy of the GNU General Public License     *
 *   along with this program.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/

#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include "pmio.h"
#include "Extract.h"

void cleanOperation(const work_path){ // NOTE:Run it in last, and don't do anything
    // 1. Run script
    int last_script_file_length = strlen(work_path) + strlen("/HOOKS") + 1;
    char *last_script_file = (char*) malloc(last_script_file_length); // Alloc memory space
    snprintf(last_script_file, last_script_file_length, "%s/HOOKS", work_path); // Memory safe version

    int script_command_length = strlen("sudo ") + last_script_file_length;
    char *script_command = (char*) malloc(script_command_length); // Alloc memory space
    snprintf(script_command, script_command_length, "sudo %s", last_script_file); // Memory safe version

    system(script_command);
    // 2. Clean Directory
    rmdir(work_path);
}

int installPackageFromSource(char* work_path){ // NOTE:ONLY FOR TEST
    // 1. Prepare to build
    int build_script_file_length = strlen(work_path) + strlen("/BUILD-SCRIPT") + 1;
    char *build_script_file = (char*) malloc(build_script_file_length); // Alloc memory space
    snprintf(build_script_file, build_script_file_length, "%s/BUILD-SCRIPT", work_path); // Memory safe version

    chmod(build_script_file, 777); // Mode 777

    // 2. Start build
    int build_command_length = strlen("sudo ") + build_script_file_length;
    char *build_command = (char*) malloc(build_command_length); // Alloc memory space
    snprintf(build_command, build_command_length, "sudo %s", build_script_file);

    system(build_command);

    // 3. Clean
    cleanOperation();

    return 0;
}

int installPackage(char* package_path){ // NOTE:DON'T RUN IT!
    // 1. Create temp directory
    char directory_template[] = "/tmp/pkgTmpDirXXXXXX";
    char *temp_directory_name = mkdtemp(directory_template);
    // 2. Unpacked package
    extractArchiveLinux(package_path, temp_directory_name);
    // 3. Build or copy
    int build_script_file_length = strlen(temp_directory_name) + strlen("/BUILD-SCRIPT") + 1;
    char *build_script_file = (char*) malloc(build_script_file_length); // Alloc memory space
    snprintf(build_script_file, build_script_file_length, "%s/BUILD-SCRIPT", temp_directory_name); // Memory safe version

    struct stat stat_buffer; // Just a buffer
    if(stat(build_script_file, &buffer) == -1 && errno == ENOENT){ // File not found,FIXED
        // TODO: Copy files to /
    }else{
        installPackageFromSource(temp_directory_name); // Build from source code
    }

    return 0;
}