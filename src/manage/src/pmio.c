#include <stdio.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include "pmio.h"

void putError(const char* message){
    tColorRed(); // Text v
    textAttr_bold(); // Attributes
    putn("ERROR: ");
    textAttr_reset();
    putn(message);
}

int exists(char* fileName){
    struct stat stat_buffer; // Just a buffer
    if(stat(fileName, &stat_buffer) == -1 && errno == ENOENT){ // File not found,FIXED
        return -1;
    }
    return 0;
}