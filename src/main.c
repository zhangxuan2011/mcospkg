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
 *   along with this program.  If not, see <http://www.gnu.org/licenses/>.	
 ***************************************************************************/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <dirent.h>
#include "Extract.h"
#include "pmio.h"
#include <unistd.h>
#include <stddef.h>
#include "TextAttributes.h"

int checkVersion(char* package_name, char* version);
int rm_file(char* dir_name);

int xkexit(work_path, char* tmpdir, int errorlevel) {
	rm_file(tmpdir);
	exit(errorlevel);
	return 0;
}

int chmod3(char* filename, int mode) {
    int arglen = strlen("sudo chmod 777 ") + strlen(filename) + 1;

    char* arg = (char*) malloc(arglen);

    sprintf(arg, "sudo chmod 777 %s", filename);

    int result = system(arg);

    return result;
}

int copy_file(char* src, char* target) { // Copy files

    int copy_file_length = 17 + strlen(src) + strlen(target) + 1;
    char *copy_file = (char*) malloc(copy_file_length);
    snprintf(copy_file, copy_file_length, "sudo cp -f \"%s\" \"%s\"", src, target);
    return system(copy_file);
}

int rm_file(char* dir_name) { // Remove file or directory, rewrite it in future!(maybe)
	int rm_command_length = strlen("sudo rm -rf \"") + strlen(dir_name) + strlen("\" > /dev/null 2>&1") + 1;	//Get length
	char* rm_command = (char*) malloc(rm_command_length);	// Alloc memory space
	snprintf(rm_command, rm_command_length, "sudo rm -rf \"%s\" > /dev/null 2>&1", dir_name);	// Format command
	int status = system(rm_command);	// execute command and get return value
	free(rm_command);	// free memory space
	rm_command = NULL; // if the pointer is NULL, free() can't free it and no errors
	return status;	// return status,but actually i never use it lol.
}

void releaseObject(char* hook_file, char* build_script_file) { // NOTE: IMPORTANT!!!!BECAUSE I MUST TO FREE MEMORY SPACE!!!
    free(hook_file);	// free memory space
    free(build_script_file);	// too
    hook_file = NULL;
    build_script_file = NULL;	// you know why bro
    return;
}

void registerRemoveInfo(char* work_path, char* package_name, char* version) { // Register Remove Info, like its name
    mkdir("/etc/mcospkg/database/remove_info", 777);	// if not exists, make directory to save remove infos
    // 1. Copy script
    int unhook_file_length = strlen(work_path) + strlen("/UNHOOKS") + 1;	// get string length
    char* unhook_file = (char*) malloc(unhook_file_length);	  // alloc memory space
    snprintf(unhook_file, unhook_file_length, "%s/UNHOOKS", work_path);    // it's the full path of unhook file!

    int target_length = 44 + strlen(package_name); // get string length 
    char* target = (char*) malloc(target_length);	// alloc memory space
    snprintf(target, target_length, "/etc/mcospkg/database/remove_info/%s-UNHOOKS", package_name); // target full path
    copy_file(unhook_file, target);
    free(unhook_file); // free memory space
    unhook_file = NULL;

    int version_info_length = strlen("/etc/mcospkg/database/remove_info/") + strlen(package_name) + 1;
    char* version_info = (char*) malloc(version_info_length);
    snprintf(version_info, version_info_length, "/etc/mcospkg/database/packages.toml");
    FILE* version_info_file = fopen(version_info, "a");
    fprintf(version_info_file, "[%s]\nversion = \"%s\"\ndependencies = []\n\n", package_name, version);
    fclose(version_info_file);

    free(target);
    free(version_info);
    target = NULL;
    version_info = NULL;
    return;
}

void cleanOperation(char* work_path, char* package_name, char* version) { // NOTE:Run it in last, and don't do anything
    // 1. Run script
    tColorBlue(); // color:blue
    printf("II: ");
    textAttr_reset(); // reset text attributes
    printf("Running install script...\t");
    fflush(stdout); // flush buffer to output text to screen
    
    int last_script_file_length = strlen(work_path) + strlen("/HOOKS") + 1;
    char *last_script_file = (char*) malloc(last_script_file_length); // Alloc memory space
    snprintf(last_script_file, last_script_file_length, "%s/HOOKS", work_path); // Memory safe version

    chmod3(last_script_file, 777);
	
    int script_command_length = strlen("sudo ") + last_script_file_length;
    char *script_command = (char*) malloc(script_command_length); // Alloc memory space
    snprintf(script_command, script_command_length, "sudo %s", last_script_file); // Memory safe version

    system(script_command); // run install script
    tColorGreen(); // color:green
    printf("Done\n");
    textAttr_reset(); // reset text attributes
    // 2. Clean Directory
    tColorBlue(); // color:blue
    printf("II: ");
    textAttr_reset(); // reset text attributes
    printf("Removing trash directory...\t");
    fflush(stdout);
    registerRemoveInfo(work_path, package_name, version); // register remove info(for remove)
    rm_file(work_path); // remove work directory
    tColorGreen(); // color:green
    printf("Done!\n");
    textAttr_reset(); // reset text attributes
    
    free(last_script_file);
    free(script_command);
}

void installPackageFromSource(char* work_path, char* package_name, char* version){ // NOTE:BUILD CODE AND STILL NOT TESTED!
    // 1. Prepare to build
    int build_script_file_length = strlen(work_path) + strlen("/BUILD-SCRIPT") + 1;
    char *build_script_file = (char*) malloc(build_script_file_length); // Alloc memory space
    snprintf(build_script_file, build_script_file_length, "%s/BUILD-SCRIPT", work_path); // Memory safe version
    // 2. Change to the working directory(by zhangxuan2011)
    if (chdir(work_path) != 0) {
    	tColorRed(); // color:red
    	printf("E: ");
    	textAttr_reset(); // reset text attributes
        perror("Cannot change work directory to extracted directory!");
        free(build_script_file);
        xkexit(work_path, -1); // Just exit!
    }
    // 3. Start build
    tColorBlue(); // color:blue
    printf("II: ");
    textAttr_reset(); // reset text attributes
    printf("Building package (This may take some time)...\n");

    chmod3(build_script_file, 777); // Mode 777

    int build_command_length = strlen("sudo ") + build_script_file_length + strlen(" > /dev/null") + 1;
    char *build_command = (char*) malloc(build_command_length); // Alloc memory space
    snprintf(build_command, build_command_length, "sudo %s > /dev/null", build_script_file);
    int build_status = system(build_command); // run build script

    if(build_status != 0) {
        tColorRed(); // color:green
        printf("Build\t--Failed(error %d)\n", build_status);
        textAttr_reset(); // reset text attributes
        xkexit(work_path, 1);
    }

    tColorGreen(); // color:green
    printf("Build\t--Over\n");
    textAttr_reset(); // reset text attributes

    // 4. Clean operation
    cleanOperation(work_path, package_name, version);
    free(build_script_file);
    free(build_command);
    return;
}

void prefixPath(char* work_path, char* str) { // NOTE: C Style!
    if (strncmp(str, work_path, strlen(work_path)) == 0) { // if find work_path in string
        int remaining_len = strlen(str) - strlen(work_path); // get new length
        if (remaining_len == 0) { // if null
            str[0] = '\0'; // just null
        } else { // else...
            memmove(str, str + strlen(work_path), remaining_len + 1); // wow, move memory!AMAZING!
        }
    }
}

void getDirectoryIndex(char* work_path, char* path, char* name, char* index_path){ // NOTE: Actually I can't read it!
    int new_path_length = strlen(path) + strlen(name) + 2;
    char* new_path = (char*)malloc(new_path_length);
    if (new_path == NULL) {
    	tColorRed(); // color:red
    	printf("E: ");
    	textAttr_reset(); // reset text attributes
        perror("Memory allocation failed!");
        xkexit(work_path, -1); // it is NULL, don't need free()
    }
    if (!strcmp(name,"")) snprintf(new_path, new_path_length, "%s", path); // DEFAULT
    else snprintf(new_path, new_path_length, "%s/%s", path, name); // OR

    DIR *directory_object = opendir(new_path);
    if (directory_object == NULL) {
        tColorRed();
    	printf("E: ");
    	textAttr_reset();
        perror("Cannot open directory!");
        rm_file(work_path); // clean a bit
        xkexit(work_path, -1);
    }

    struct dirent *entry; // file entry
    while ((entry = readdir(directory_object)) != NULL) { // a loop
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..") || !strcmp(entry->d_name, "HOOKS") || !strcmp(entry->d_name, "UNHOOKS")) continue; // NOTE: DON'T REMOVE!
        int full_path_length = strlen(new_path) + strlen(entry->d_name) + 2;
        char* full_path = (char*)malloc(full_path_length);
        if (full_path == NULL) continue;
        snprintf(full_path, full_path_length, "
