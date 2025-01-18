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
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include "Extract.h"
#include "pmio.h"

void registerRemoveInfo(const work_path, char* package_name){ // Memory Unsafe!!
    mkdir("/etc/mcospkg/database/remove_info", 777);
    // 1. Copy script
    int unhook_file_length = strlen(work_path) + strlen("/UNHOOKS") + 1;
    char* unhook_file = (char*) malloc(unhook_file_length);
    snprintf(unhook_file, unhook_file_length, "%s%s", work_path, unhook_file);

    int copy_command_length = strlen("sudo cp ") + unhook_file_length + 35 + strlen(package_name) + strlen("-UNHOOKS");
    char* copy_command = (char*) malloc(copy_command_length);
    snprintf(copy_command, copy_command_length, "sudo cp %s /etc/mcospkg/database/remove_info/%s-UNHOOKS", unhook_file, work_path);
}

void cleanOperation(char* work_path, char* package_name){ // NOTE:Run it in last, and don't do anything
    // 1. Run script
    int last_script_file_length = strlen(work_path) + strlen("/HOOKS") + 1;
    char *last_script_file = (char*) malloc(last_script_file_length); // Alloc memory space
    snprintf(last_script_file, last_script_file_length, "%s/HOOKS", work_path); // Memory safe version

    int script_command_length = strlen("sudo ") + last_script_file_length;
    char *script_command = (char*) malloc(script_command_length); // Alloc memory space
    snprintf(script_command, script_command_length, "sudo %s", last_script_file); // Memory safe version

    system(script_command);
    // 2. Clean Directory
    registerRemoveInfo(work_path, package_name);
    rmdir(work_path);
}

int installPackageFromSource(char* work_path, char* package_name){ // NOTE:ONLY FOR TEST
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
    cleanOperation(work_path, package_name);

    return 0;
}

int installPackage(char* package_path, char* package_name){ // NOTE:DON'T RUN IT!
    // 1. Create temp directory
    char directory_template[] = "/tmp/pkgTmpDirXXXXXX";
    char *temp_directory_name = mkdtemp(directory_template);
    // 2. Unpacked package
    extractArchiveLinux(package_path, temp_directory_name);
    // 3. Build or copy
    int build_script_file_length = strlen(temp_directory_name) + strlen("/BUILD-SCRIPT") + 1;
    char *build_script_file = (char*) malloc(build_script_file_length); // Alloc memory space
    snprintf(build_script_file, build_script_file_length, "%s/BUILD-SCRIPT", temp_directory_name); // Memory safe version

    
    if(exists(build_script_file)){
        // TODO: Copy files to /
    }else{
        installPackageFromSource(temp_directory_name, package_name); // Build from source code
    }

    return 0;
}