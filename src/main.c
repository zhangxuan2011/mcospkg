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

void releaseObject(char* unhook_file, char* hook_file, char* temp_directory_name, char* build_script_file){ // NOTE:IMPORTANT!!!!
    free(unhook_file);
    free(hook_file);
    free(temp_directory_name);
    free(build_script_file);
}

void registerRemoveInfo(char* work_path, char* package_name){ // Memory Unsafe!!
    mkdir("/etc/mcospkg/database/remove_info", 777);
    // 1. Copy script
    int unhook_file_length = strlen(work_path) + strlen("/UNHOOKS") + 1;
    char* unhook_file = (char*) malloc(unhook_file_length);
    snprintf(unhook_file, unhook_file_length, "%s%s", work_path, unhook_file);

    int copy_command_length = strlen("sudo cp ") + unhook_file_length + 35 + strlen(package_name) + strlen("-UNHOOKS");
    char* copy_command = (char*) malloc(copy_command_length);
    snprintf(copy_command, copy_command_length, "sudo cp %s /etc/mcospkg/database/remove_info/%s-UNHOOKS", unhook_file, work_path);
    free(unhook_file);
    free(copy_command);
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
    free(last_script_file);
    free(script_command);
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
    free(build_script_file);
    free(build_command);
    return 0;
}

char* substr(const char* str, int start, int len) {
    // 获取字符串的长度
    int str_len = strlen(str);
    
    // 检查起始位置和长度是否有效
    if (start < 0 || start >= str_len) {
        return NULL; // 起始位置无效
    }
    if (len < 0) {
        return NULL; // 长度无效
    }
    if (start + len > str_len) {
        len = str_len - start; // 调整长度，确保不超过字符串末尾
    }

    // 分配内存来存储子串，+1 为字符串结束符 '\0' 留位置
    char* sub = (char*)malloc(len + 1);
    if (sub == NULL) {
        perror("malloc failed");
        exit(EXIT_FAILURE);
    }

    // 复制子串
    strncpy(sub, str + start, len);
    sub[len] = '\0'; // 确保子串以 '\0' 结束

    return sub;
}

void prefixPath(char* work_path, char* str){
    char* result = strstr(str, work_path);
    int index = result - work_path + 1;
    substr(str, index, strlen(str));
    char* astr = malloc(1 + strlen(str) + 1);
    strcpy(astr, "/");
    strcat(astr, str);
    strcpy(str, astr);
}

void getDirectoryIndex(char* work_path, char* path, char* name, char* index_path){
    int new_path_length = strlen(path) + strlen(name) + 2;
    char* new_path = (char*)malloc(new_path_length);
    if(name[0] == '\0'){
        snprintf(new_path, new_path_length, "%s", path);
    }else{
        snprintf(new_path, new_path_length, "%s/%s", path, name);
    }

    DIR *directory_object = opendir(new_path);
    if(directory_object == NULL){
        perror("opendir failed");
        free(new_path);
        exit(-1);
        return;
    }

    struct dirent *entry;
    while((entry = readdir(directory_object)) != NULL){
        if(strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0){
            continue;
        }

        int full_path_length = strlen(new_path) + strlen(entry->d_name) + 2;
        char* full_path = (char*)malloc(full_path_length);
        snprintf(full_path, full_path_length, "%s/%s", new_path, entry->d_name);

        if(entry->d_type == 8){ // File
            FILE* fp = fopen(index_path, "w+");
            prefixPath(work_path, full_path);
            fprintf(fp, "%s\n", full_path);
            close(fp);
        }else if(entry->d_type == 4){ // Directory
            getDirectoryIndex(new_path, work_path, entry->d_name, index_path);
        }

        free(full_path);
    }

    closedir(directory_object);
    free(new_path);
}

int copy_file(char* src, char* target){ // TODO: Rewrite it in future!
    int copy_file_length = 13 + strlen(src) + strlen(target) + 1;
    char *copy_file = (char*) malloc(copy_file_length);
    snprintf(copy_file, copy_file_length, "sudo cp \"%s\" \"%s\"", src, target);
    printf("CMD:%s\n", copy_file);
    return 0;
}

int installPackageDirectly(char* work_path, char* package_name){
    mkdir("/etc/mcospkg/database/remove_info", 777);
    // 1. Create Directory Index
    int index_path_length = strlen("/etc/mcospkg/database/remove-info/-file-index") + strlen(package_name);
    char* index_path = (char*) malloc(index_path_length);
    snprintf(index_path, index_path_length, "/etc/mcospkg/database/remove-info/%s-file-index", package_name);
    getDirectoryIndex(work_path, work_path, "", package_name);
    // 2. Copy files
    FILE *fp = fopen(index_path, "r+");
    char now_char = '/0';
    char buffer[FILENAME_MAX];
    int buffer_length = -1;
    while((now_char = getc(fp)) != EOF){
        if(now_char == '\n'){
            char source_path = (char*) malloc(4096);
            strcpy(source_path, work_path);
            strcpy(source_path, buffer);
            copy_file(source_path, buffer);
            strcpy(buffer, "");
            free(source_path);
        }
        buffer[buffer_length++] = now_char;
    }
    close(fp);
    // 3. Run HOOKS file and clean directory
    cleanOperation(work_path, package_name);
    // 4. Clean pointers
    free(index_path);
}

int installPackage(char* package_path, char* package_name){
    mkdir("/etc/mcospkg/database", 777);
    // 1. Create temp directory
    char directory_template[] = "/tmp/pkgTmpDirXXXXXX";
    char *temp_directory_name = mkdtemp(directory_template);
    // 2. Unpacked package
    extractArchiveLinux(package_path, temp_directory_name);
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
    if(!(exists(hook_file) && exists(unhook_file))){
        releaseObject(unhook_file, hook_file, temp_directory_name, build_script_file);
        tColorRed();
        textAttr_bold();
        printf("E: ");
        textAttr_clear();
        textAttr_reset();
        printf("Invalid package!\n");
        rmdir(temp_directory_name);
        exit(-1);
    }
    if(!exists(build_script_file)){
        installPackageDirectly(temp_directory_name, package_name);
    }else{
        installPackageFromSource(temp_directory_name, package_name); // Build from source code
    }
    releaseObject(unhook_file, hook_file, temp_directory_name, build_script_file);
    return 0;
}
