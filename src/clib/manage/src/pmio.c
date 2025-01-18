#include <stdio.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>

int exists(char* file_name){
    struct stat stat_buffer; // Just a buffer
    if(stat(file_name, &stat_buffer) == -1 && errno == ENOENT){ // File not found,FIXED
        return -1;
    }
    return 0;
}

int exists_with_directory(char* directory, char* file_name){
    int full_path_length = strlen(directory) + 1 + strlen(file_name) + 1;
    char* full_path = (char*) malloc(full_path_length);
    snprintf(full_path, full_path_length, "%s/%s", directory, file_name);

    struct stat stat_buffer; // Just a buffer
    if(stat(full_path, &stat_buffer) == -1 && errno == ENOENT){ // File not found,FIXED
        return -1;
    }
    return 0;
}