#include <stdlib.h>
#include <string.h>
#include <stdio.h>
int extractArchiveLinux(const char* fileName, const char* extractPath){ //tar.xz
    int commandLength = strlen("tar -xzf  -C ") + strlen(fileName) + strlen(extractPath); // Get string length for alloc
    char* command = (char*)malloc(commandLength); // Alloc Memory space
    sprintf(command, "tar -xzf %s -C %s", fileName, extractPath);
    //execute
    int errorLevel = system(command);
    if(errorLevel != 0){ // Whoa! Extract error!
        return -1;
    }
    return 0;
}