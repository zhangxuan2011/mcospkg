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
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <dirent.h>
#include "Extract.h"
#include "pmio.h"
#include "TextAttributes.h"

int checkVersion(char* package_name, char* version);

int copy_file(char* src, char* target) { // Copy files

    int copy_file_length = 13 + strlen(src) + strlen(target) + 1;
    char *copy_file = (char*) malloc(copy_file_length);
    snprintf(copy_file, copy_file_length, "sudo cp \"%s\" \"%s\"", src, target);
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

    int version_info_length = strlen("/etc/mcospkg/database/remove_info/") + package_name + 1;
    char* version_info = (char*) malloc(version_info_length);
    snprintf(version_info, version_info_length, "/etc/mcospkg/database/packages.toml", package_name);
    FILE* version_info_file = fopen(version_info, "a");
    fprintf(version_info_file, "[%s]version = \"%s\"\ndependencies = []\n\n", package_name);
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

	chmod(last_script_file, 777);
	
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
        exit(-1); // Just exit!
    }
    // 3. Start build
    tColorBlue(); // color:blue
    printf("II: ");
    textAttr_reset(); // reset text attributes
    printf("Start building.\n");
    chmod(build_script_file, 777); // Mode 777
    int build_command_length = strlen("sudo ") + build_script_file_length;
    char *build_command = (char*) malloc(build_command_length); // Alloc memory space
    snprintf(build_command, build_command_length, "sudo %s", build_script_file);
    system(build_command); // run build script
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
        exit(-1); // it is NULL, don't need free()
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
        exit(-1);
    }

    struct dirent *entry; // file entry
    while ((entry = readdir(directory_object)) != NULL) { // a loop
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..") || !strcmp(entry->d_name, "HOOKS") || !strcmp(entry->d_name, "UNHOOKS")) continue; // NOTE: DON'T REMOVE!
        int full_path_length = strlen(new_path) + strlen(entry->d_name) + 2;
        char* full_path = (char*)malloc(full_path_length);
        if (full_path == NULL) continue;
        snprintf(full_path, full_path_length, "%s/%s", new_path, entry->d_name);
        if (entry->d_type == 8) { // File
            FILE* fp = fopen(index_path, "a");
            if (fp == NULL) {
				tColorRed();
				printf("E: ");
				textAttr_reset();
				perror("Cannot create file object!");
				rm_file(work_path); // clean a bit
				exit(-1);
            }

            prefixPath(work_path, full_path);
            fprintf(fp, "%s\n", full_path);
            fclose(fp);
        } else if (entry->d_type == 4) { // Directory
            getDirectoryIndex(work_path, new_path, entry->d_name, index_path); // open it!
        }

        free(full_path);
    }

    closedir(directory_object);
    free(new_path);
}

int installPackageDirectly(char* work_path, char* package_name, char* version){
    tColorBlue();
    printf("II: ");
    textAttr_reset();
    printf("Installing package\t(Mode: Directory)\n");
    mkdir("/etc/mcospkg/database/remove_info", 777);
    // 1. Create Directory Index
    tColorBlue();
    printf("II: ");
    textAttr_reset();
    printf("Making installion index...\t");
    fflush(stdout);
    int index_path_length = strlen("/etc/mcospkg/database/remove_info/-file-index") + strlen(package_name) + 1;
    char* index_path = (char*) malloc(index_path_length);
    snprintf(index_path, index_path_length, "/etc/mcospkg/database/remove_info/%s-file-index", package_name);
    getDirectoryIndex(work_path, work_path, "", index_path);
    tColorGreen();
    printf("Done.\n");
    tColorBlue();
    // 2. Copy files
    tColorBlue();
    printf("II: ");
    textAttr_reset();
    printf("Coping files...\t");
    fflush(stdout);
    FILE *fp = fopen(index_path, "r");
    if(fp == NULL){
        tColorRed();
        printf("Error!\n\nE: ");
        textAttr_reset();
    	perror("Cannot open index files!");
    	rm_file(work_path);
    	exit(-1);
   	}
   	
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    while ((read = getline(&line, &len, fp)) != -1) {
            char* source_path = (char*) malloc(strlen(work_path) + strlen(line) + 1);
            strcpy(source_path, work_path);
            strcat(source_path, line);
            source_path[strcspn(source_path, "\n")] = '\0';
            line[strcspn(line, "\n")] = '\0';
            if(!strcmp(line,"/HOOKS") || !strcmp(line,"/UNHOOKS")) continue;
            copy_file(source_path, line);
            free(source_path);
    }
	free(line);
    fclose(fp);
    tColorGreen();
    printf("Done.\n");
    textAttr_reset();
    // 3. Run HOOKS file and clean directory
    cleanOperation(work_path, package_name, version);
    // 4. Clean pointers
    free(index_path);
    return 0;
}

void run_unhooks(char* package_name) {
    tColorBlue();
    printf("II: ");
    textAttr_reset();
	printf("Running uninstall script...\t");
	fflush(stdout);
	
    int unhook_file_length = strlen("/etc/mcospkg/database/remove_info/-UNHOOKS") + strlen(package_name) + 1;
    char* unhook_file = (char*) malloc(unhook_file_length);
    snprintf(unhook_file, unhook_file_length, "/etc/mcospkg/database/remove_info/%s-UNHOOKS", package_name);
	
	if (!exists(unhook_file)) { 
        tColorRed();
        printf("Error\nE: ");
        textAttr_reset();
        printf("package not exists!\n");
        free(unhook_file);
        exit(-1);
	}

	chmod(unhook_file, 777);
    char* unhook_command = malloc(5 + unhook_file_length);
    strcpy(unhook_command, "sudo ");
    strcat(unhook_command, unhook_file);
    strcat(unhook_command, " > /dev/null 2>&1"); // redirect output to null(ignore output)

    system(unhook_command);
    free(unhook_command);
    tColorGreen();
	printf("Done\n");
    textAttr_reset();

    remove(unhook_file);
}

void removePackage(char* package_name) {
    // 1. Get types
    int index_path_length = strlen("/etc/mcospkg/database/remove_info/-file-index") + strlen(package_name) + 1;
    char* index_path = (char*) malloc(index_path_length);
    snprintf(index_path, index_path_length, "/etc/mcospkg/database/remove_info/%s-file-index", package_name);
    tColorBlue();
    printf("I: ");
    textAttr_reset();
	printf("Package Name:%s\n", package_name); // NOTE: Output Package Name, can delete
    if (exists(index_path)) { // Directly
        tColorBlue();
        printf("II: ");
        textAttr_reset();
    	printf("Uninstalling package\t(Mode: Directly)\nRemoving Files...\t");
    	fflush(stdout);
    	
        FILE* fp = fopen(index_path, "r");
        char *line = NULL;
        size_t len = 0;
        ssize_t read;
        
        while((read = getline(&line, &len, fp)) != -1){
			rm_file(line);
        }
        tColorGreen();
        printf("Done.\n");
        textAttr_reset();
        
        free(line);
        remove(index_path);
        run_unhooks(package_name);
    } else { // BUILD-SCRIPTS
        // Run UNHOOKS and done.
        tColorBlue();
        printf("II: ");
        textAttr_reset();
        printf("Uninstalling package\t(Mode: Build & Install)\n");
        run_unhooks(package_name);
    }
    free(index_path);
}

int checkVersionNumber(char* old, char* new) { // the friendly AIGC
    // 1. Split version
    char* old_version = strtok(old, ".");
    char* new_version = strtok(new, ".");
    // 2. Compare version
    while(old_version != NULL && new_version != NULL){
        int old_version_number = atoi(old_version);
        int new_version_number = atoi(new_version);
        if(old_version_number < new_version_number){ // old < new
            return 1; // Upgrade
        }else if(old_version_number > new_version_number){ // old > new
            return 2; // Downgrade
        }
        old_version = strtok(NULL, ".");
        new_version = strtok(NULL, ".");
    }
    return 0; // Same version
}

int checkVersion(char* package_name, char* version){
    // 1. Get types
    int index_path_length = strlen("/etc/mcospkg/database/remove_info/packages.toml") + 1;
    char* index_path = (char*) malloc(index_path_length);
    snprintf(index_path, index_path_length, "/etc/mcospkg/database/remove_info/%s.toml", package_name);
    // 2. Check exists
    if(!exists(index_path)){
        return -1;
    }
    // 3. Check version
    FILE* fp = fopen(index_path, "r");    
    char *line = NULL;
    size_t len = 0;
    ssize_t read;
    char* ifpkgname = (char*) malloc(strlen(package_name) + 5);
    strcpy(ifpkgname, "[");
    strcat(ifpkgname, package_name);
    strcat(ifpkgname, "]\n");
    while((read = getline(&line, &len, fp))!= -1){
        if(strcmp(line, ifpkgname) == 0){
            while((read = getline(&line, &len, fp))!= -1){
                if(strncmp(line, "version = ", 10) == 0){
                    char* version_str = (char*) malloc(strlen(line) - 10);
                    strcpy(version_str, line + 10);
                    version_str[strcspn(version_str, "\n")] = '\0';
                    return checkVersionNumber(version_str, version);
                    break;
                }
            }
        }
    }
    /* Tips: return value -2 means have some errors
    * return -1 means package not found
    * return 0 means version is same
    * return 1 means version is upgrade
    * return 2 means version will downgrade
    */
    return -2;
}

int installPackage(char* package_path, char* package_name, char* version, char* SHA256){
    // Tips: SHA256 is not implemented yet.just null okay
    tColorBlue();
    printf("I: ");
    textAttr_reset();
	printf("Package Name:%s\n", package_name); // NOTE: Output Package Name, can delete
    mkdir("/etc/mcospkg/database", 777);
    // 1. Create temp directory
    printf("Extracting... ");
    fflush(stdout); 
    char directory_template[] = "/tmp/pkgTmpDirXXXXXX";
    char *temp_directory_name = mkdtemp(directory_template);
    // 2. Unpack package
    extractArchiveLinux(package_path, temp_directory_name);
    printf("Done.\n\n");
    // 3. Build or copy
    int build_script_file_length = strlen(temp_directory_name) + strlen("/BUILD-SCRIPT") + 1;
    char *build_script_file = (char*) malloc(build_script_file_length); // Alloc memory space
    snprintf(build_script_file, build_script_file_length, "%s/BUILD-SCRIPT", temp_directory_name); // Memory safe version

    int hook_file_length = strlen(temp_directory_name) + strlen("/HOOKS") + 1;
    char *hook_file = (char*) malloc(hook_file_length); // Alloc memory space
    snprintf(hook_file, hook_file_length, "%s/HOOKS", temp_directory_name); // Memory safe version
    
    int unhook_file_length = strlen(temp_directory_name) + strlen("/UNHOOKS") + 1;
    char *unhook_file = (char*) malloc(unhook_file_length); // Alloc memory space
    snprintf(unhook_file, unhook_file_length, "%s/UNHOOKS", temp_directory_name); // Memory safe version
    
    if(!(exists(hook_file) || exists(unhook_file))){
        releaseObject(hook_file, build_script_file);
        tColorRed();
        printf("E: ");
        textAttr_reset();
        printf("Invalid package!\n");
        rm_file(temp_directory_name);
        exit(-1);
    }
    if(checkVersion(package_name, version) >= 0){
        releaseObject(hook_file, build_script_file);
        tColorRed();
        printf("E: ");
        textAttr_reset();
        printf("Package version is same! skiped.\n");
        rm_file(temp_directory_name);
        exit(-1);
    }
    if(checkVersion(package_name, version) == 2){
        releaseObject(hook_file, build_script_file);
        tColorRed();
        printf("E: ");
        textAttr_reset();
        printf("Downgrade is not allowed!\n");
        rm_file(temp_directory_name);
        exit(-1);
    }
    if(checkVersion(package_name, version) == 1){
        removePackage(package_name);
    }
    if(!exists(build_script_file)){
        installPackageDirectly(temp_directory_name, package_name, version);
    }else{
        installPackageFromSource(temp_directory_name, package_name, version); // Build from source code
    }
    
    releaseObject(hook_file, build_script_file);
    return 0;
}
